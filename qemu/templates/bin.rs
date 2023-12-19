//! Executable binary for QEMU_BINARY_NAME

use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_BINARY_CONST;
use std::env::args;

fn main() {
    let mut args: Vec<String> = args().collect();

    args.remove(0);

    MemFdExecutable::new("QEMU_BINARY_NAME", QEMU_BINARY_CONST)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start QEMU process")
        .wait()
        .expect("QEMU process failed");
}