//! Executable binary for qemu-sh4eb

use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_SH4EB_LINUX_USER;
use std::env::args;

fn main() {
    let mut args: Vec<String> = args().collect();

    args.remove(0);

    MemFdExecutable::new("qemu-sh4eb", QEMU_SH4EB_LINUX_USER)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start QEMU process")
        .wait()
        .expect("QEMU process failed");
}