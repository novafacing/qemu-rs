//! Executable binary for qemu-hppa

use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_HPPA_LINUX_USER;
use std::env::args;

fn main() {
    let mut args: Vec<String> = args().collect();

    args.remove(0);

    MemFdExecutable::new("qemu-hppa", QEMU_HPPA_LINUX_USER)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start QEMU process")
        .wait()
        .expect("QEMU process failed");
}