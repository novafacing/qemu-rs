# QEMU-RS

QEMU for Rust, and Rust for QEMU!

## Crates

This repository provides several QEMU-related crates:

* [qemu](https://github.com/novafacing/qemu-rs/tree/main/qemu): QEMU binary installer
* [qemu-plugin-sys](https://github.com/novafacing/qemu-rs/tree/main/qemu-plugin-sys): Low level bindings to the QEMU plugin API
* [qemu-plugin](https://github.com/novafacing/qemu-rs/tree/main/qemu-plugin): High level bindings to the QEMU plugin API

The crates work together to enable building QEMU utilities in Rust and running QEMU from
Rust code in a machine-specified way.


## Try it Out

To see what the crate can do, trace the execution (including syscalls, memory accesses,
and instructions) of a program like:

```sh
cargo run -r --bin tracer -- -a /bin/ls -- -lah
```

## Installing QEMU

This repository also provides a crate (`qemu`) which builds QEMU from source and
installs Rust wrappers for QEMU as binaries.

You can install QEMU with (add any additional features you need, e.g. `plugins`):

```sh
cargo install qemu@8.2.0-v1  --features=binaries
```

On some systems, particularly BTRFS systems, `/tmp` may not be large enough for the
temporary build directory (QEMU is quite large to build). In this case, create a
directory on your root filesystem (e.g. `$HOME/.cargo/tmp`) and set
`CARGO_TARGET_DIR=$HOME/.cargo/tmp` when running the install command.