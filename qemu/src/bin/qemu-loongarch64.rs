//! Executable binary for qemu-loongarch64

use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_LOONGARCH64_LINUX_USER;
use std::env::args;

fn main() {
    let mut args: Vec<String> = args().collect();

    args.remove(0);

    MemFdExecutable::new("qemu-loongarch64", QEMU_LOONGARCH64_LINUX_USER)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start QEMU process")
        .wait()
        .expect("QEMU process failed");
}