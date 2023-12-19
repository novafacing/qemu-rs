//! Executable binary for qemu-system-riscv64

use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_RISCV64_SOFTMMU;
use std::env::args;

fn main() {
    let mut args: Vec<String> = args().collect();

    args.remove(0);

    MemFdExecutable::new("qemu-system-riscv64", QEMU_RISCV64_SOFTMMU)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start QEMU process")
        .wait()
        .expect("QEMU process failed");
}