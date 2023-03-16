use memfd_exec::{MemFdExecutable, Stdio};
use qemu::qemu_system_loongarch64;

use std::env::args;

fn main() {
    let qemu = qemu_system_loongarch64();
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    MemFdExecutable::new("qemu-system-loongarch64", qemu)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start qemu process")
        .wait()
        .expect("Qemu process failed");
}
