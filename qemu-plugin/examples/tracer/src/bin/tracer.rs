use anyhow::{anyhow, Error, Result};
use clap::Parser;
use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_X86_64_LINUX_USER;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_cbor::Deserializer;
use serde_json::to_string;
use std::{
    fs::OpenOptions,
    io::{stdout, BufRead, BufReader, Write},
    os::unix::net::UnixListener,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{read, remove_file, write},
    join, main, spawn,
    task::spawn_blocking,
};
use tracer::Event;

#[cfg(debug_assertions)]
const PLUGIN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../target/debug/libtracer.so"
));

#[cfg(not(debug_assertions))]
const PLUGIN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../target/release/libtracer.so"
));

fn tmp(prefix: &str, suffix: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}{}{}",
        prefix,
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>(),
        suffix
    ))
}

#[derive(Parser, Debug, Clone)]
/// Run QEMU with a plugin that logs events. To pass arguments to QEMU, use the QEMU environment
/// variables.
struct Args {
    #[clap(short = 'i', long)]
    /// Whether instructions should be logged
    pub log_insns: bool,
    #[clap(short = 'm', long)]
    /// Whether memory accesses should be logged
    pub log_mem: bool,
    #[clap(short = 's', long)]
    /// Whether syscalls should be logged
    pub log_syscalls: bool,
    #[clap(short = 'a', long)]
    /// Whether all events should be logged
    pub log_all: bool,
    #[clap(short = 'I', long)]
    /// An input file to use as the program's stdin, otherwise the driver's stdin is used
    pub input_file: Option<PathBuf>,
    #[clap(short = 'O', long)]
    /// An output file to write the trace to, otherwise stdout is used
    pub output_file: Option<PathBuf>,
    /// The program to run
    #[clap()]
    pub program: PathBuf,
    /// The arguments to the program
    #[clap(num_args = 1.., last = true)]
    pub args: Vec<String>,
}

impl Args {
    fn to_plugin_args(&self) -> String {
        format!(
            "log_insns={},log_mem={},log_syscalls={}",
            self.log_insns | self.log_all,
            self.log_mem | self.log_all,
            self.log_syscalls | self.log_all
        )
    }

    fn to_qemu_args(&self, socket_path: &Path, plugin_path: &Path) -> Result<Vec<String>> {
        let mut qemu_args = vec![
            "-plugin".to_string(),
            format!(
                "{},{},socket_path={}",
                plugin_path.display(),
                self.to_plugin_args(),
                socket_path.display()
            ),
            "--".to_string(),
            self.program
                .to_str()
                .ok_or_else(|| anyhow!("Failed to convert program path to string"))?
                .to_string(),
        ];

        qemu_args.extend(self.args.clone());

        Ok(qemu_args)
    }
}

async fn run(input: Option<Vec<u8>>, args: Vec<String>) -> Result<()> {
    let mut exe = MemFdExecutable::new("qemu", QEMU_X86_64_LINUX_USER)
        .args(args)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(input) = input {
        let mut stdin = exe.stdin.take().ok_or_else(|| anyhow!("No stdin"))?;
        spawn_blocking(move || stdin.write_all(&input));
    }

    let stdout = exe.stdout.take().ok_or_else(|| anyhow!("No stdout"))?;

    let out_reader = spawn_blocking(move || {
        let mut line = String::new();
        let mut out_reader = BufReader::new(stdout);

        loop {
            line.clear();

            if let 0 = out_reader.read_line(&mut line)? {
                break;
            }

            let line = line.trim();

            if !line.is_empty() {
                println!("{line}");
            }
        }

        Ok::<(), Error>(())
    });

    let stderr = exe.stderr.take().ok_or_else(|| anyhow!("No stderr"))?;

    let err_reader = spawn_blocking(move || {
        let mut line = String::new();
        let mut err_reader = BufReader::new(stderr);

        loop {
            line.clear();

            if let 0 = err_reader.read_line(&mut line)? {
                break;
            }

            let line = line.trim();

            if !line.is_empty() {
                eprintln!("{line}");
            }
        }

        Ok::<(), Error>(())
    });

    let waiter = spawn_blocking(move || exe.wait());

    let (out_res, err_res, waiter_res) = join!(out_reader, err_reader, waiter);

    out_res??;
    err_res??;
    waiter_res??;

    Ok(())
}

fn listen<P>(listen_sock: UnixListener, outfile: Option<P>) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut outfile_stream = if let Some(outfile) = outfile.as_ref() {
        Box::new(OpenOptions::new().create(true).append(true).open(outfile)?) as Box<dyn Write>
    } else {
        Box::new(stdout()) as Box<dyn Write>
    };

    let (mut stream, _) = listen_sock.accept()?;
    let it = Deserializer::from_reader(&mut stream).into_iter::<Event>();

    for event in it {
        outfile_stream.write(to_string(&event?)?.as_bytes())?;
        outfile_stream.write(b"\n")?;
    }

    Ok(())
}

#[main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let socket_path = tmp("/tmp/qemu-", ".sock");
    let plugin_path = tmp("/tmp/qemu-", ".so");

    write(&plugin_path, PLUGIN).await?;

    let input = if let Some(input_file) = args.input_file.as_ref() {
        let Ok(input_file) = input_file.canonicalize() else {
            return Err(anyhow!("Failed to canonicalize input file"));
        };

        Some(read(input_file).await?)
    } else {
        None
    };

    let listen_sock = UnixListener::bind(&socket_path)?;

    let qemu_args = args.to_qemu_args(&socket_path, &plugin_path)?;
    let qemu_task = spawn(async move { run(input, qemu_args).await });

    let socket_task = spawn_blocking(move || listen(listen_sock, args.output_file.as_ref()));

    let (qemu_res, socket_res) = join!(qemu_task, socket_task);

    remove_file(&plugin_path).await?;
    remove_file(&socket_path).await?;

    qemu_res??;

    socket_res??;

    Ok(())
}
