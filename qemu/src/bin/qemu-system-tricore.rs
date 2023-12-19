//! Executable binary for qemu-system-tricore

use memfd_exec::{MemFdExecutable, Stdio};
use qemu::QEMU_TRICORE_SOFTMMU;
use std::env::args;

fn main() {
    let mut args: Vec<String> = args().collect();

    args.remove(0);

    MemFdExecutable::new("qemu-system-tricore", QEMU_TRICORE_SOFTMMU)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start QEMU process")
        .wait()
        .expect("QEMU process failed");
}