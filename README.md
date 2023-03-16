# qemu

This crate provides an installer for QEMU binaries. You can use it to install QEMU
system and user mode emulators and use them in your code.

## Table of Contents

- [qemu](#qemu)
  - [Table of Contents](#table-of-contents)
  - [Dependencies](#dependencies)
    - [Install Required Dependencies on Ubuntu](#install-required-dependencies-on-ubuntu)
    - [Install Required Dependencies on Fedora](#install-required-dependencies-on-fedora)
  - [Usage](#usage)
    - [Rust-executable wrapper for user emulator](#rust-executable-wrapper-for-user-emulator)
      - [Cargo.toml](#cargotoml)
  - [Feature Flags](#feature-flags)
    - [Just install qemu-x86\_64 usermode emulator with default options](#just-install-qemu-x86_64-usermode-emulator-with-default-options)
    - [Install an optimized qemu-x86\_64 usermode emulator](#install-an-optimized-qemu-x86_64-usermode-emulator)
    - [Install qemu-system-arm emulator with customized options](#install-qemu-system-arm-emulator-with-customized-options)
  - [Important Note](#important-note)
  - [Contributing](#contributing)

## Dependencies

To install this crate, you need all the dependencies required to build QEMU for your
system. There are some packages that are always required. The updated list can be found
[here](https://wiki.qemu.org/Hosts/Linux#Required_additional_packages). As of QEMU 7.3,
you can install the required packages with the distro-specific commands below. If you
encounter any other problems building, try checking the
[build instructions](https://github.com/qemu/qemu#building) for your platform. If you are
unable to fix your issue, please file an issue here!

### Install Required Dependencies on Ubuntu

```sh
$ sudo apt-get install git libglib2.0-dev libfdt-dev \
    libpixman-1-dev zlib1g-dev ninja-build
```

### Install Required Dependencies on Fedora

```sh
$ sudo dnf install git glib2-devel libfdt-devel \
    pixman-devel zlib-devel bzip2 ninja-build python3
```

## Usage

See the feature flags section for information on enabling targets, but once you have
an installation, you can use the binary!

### Rust-executable wrapper for user emulator

There are crates available for binary distributions of each qemu program, and they all
essentially implement this pattern. This executable will run `qemu-aarch64` as a wrapper
and pass through command line args and stdio to the executable. Much more complicated
things are possible now that we have a binary available straight in Rust though, so
the sky is the limit!

#### Cargo.toml

```toml
[package]
name = "qemu-aarch64"
version = "0.1.0"
edition = "2021"
description = "QEMU binary installer for qemu-aarch64"
license = "MIT"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
memfd-exec = "0.1.4"
qemu = { version = "0.1.3", features = ["qemu-aarch64"] }
```

```rust
use memfd_exec::{MemFdExecutable, Stdio};
use qemu::qemu_aarch64;

use std::env::args;

fn main() {
    let qemu = qemu_aarch64();
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    MemFdExecutable::new("qemu-aarch64", qemu)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start qemu process")
        .wait()
        .expect("Qemu process failed");
}
```

## Feature Flags

The feature flags of this crate provide an interface to the configure options for
QEMU. By default, all flags are set just as QEMU's `configure` script sets them with
the exception of targets (see [Important Note](#important-note)). Some examples of how
to configure this crate as a dependency:

### Just install qemu-x86_64 usermode emulator with default options

This will make the `qemu-x86_64` binary available.

```toml
qemu = { version = "0.1.4", features = ["qemu-x86_64"] }
```

### Install an optimized qemu-x86_64 usermode emulator

This will also make the `qemu-x86_64` binary available, but will strip and optimize it
with `lto`.

```toml
qemu = { version = "0.1.4", features = ["qemu-x86_64", "optimized"]
```

### Install qemu-system-arm emulator with customized options

We now selectively opt in to features. These options implicitly set
"disable-default-features", and enabling *any* of them requires you
to opt in to all features you need. Use this only if you really need
it! These are all enabled by default if they are available anyway! See
the [qemu documentation](https://www.qemu.org/docs/master/devel/build-system.html#stage-1-configure)
about configure options for more details.

```toml
qemu = {
    version = "0.1.4",
    default-features = false,
    features = [
        # Specify just one target we want
        "qemu-system-x86_64",
        # Specify compile options
        "stack-protector",
        "coroutine-pool",
        "install-blobs",
        "werror",
        "lto",
        "strip",
        "debug",
        # These are default-on options that we have disabled and are now
        # selectively enabling
        "blkio",
        "bpf",
        "cap-ng",
        "capstone",
        "curl",
        "curses",
        "fuse",
        "fuse-lseek",
        "kvm",
    ]
}
```

## Important Note

Due to [bugs](https://github.com/rust-lang/rust/pull/103812)
[in](https://github.com/rust-lang/rust/issues/103979) [rustc](https://github.com/rust-lang/rust/issues/65818)
this crate does *nothing* with the default feature flags. This will be changed once `103812`
is merged, but for now this crate will cause a `rustc` crash if installed with *all*
targets enabled.

## Contributing

If you notice the binary distributions contain out of date dependencies on, for example,
`memfd-exec`, please run `cargo make update-binary-deps` to update dependencies for all
of them and PR the resulting diff. Contributions are welcome for any reason!