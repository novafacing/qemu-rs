# QEMU-RS

QEMU for Rust, and Rust for QEMU!

## Crates

This repository provides several QEMU-related crates:

* [qemu](https://github.com/novaafcing/qemu-rs/tree/main/qemu): QEMU binary installer
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