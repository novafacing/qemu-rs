//! Qemu library!
//!
//! This library provides a way to access QEMU binaries for all supported architectures
//! from rust code by wrapping the QEMU build system and then giving you binaries as
//! big constant byte arrays. Stay with me, this is a good way to do this! For example,
//! if you want to distributed a QEMU plugin written in rust, you can use this library
//! to build a plugin-supported QEMU binary and distribute it directly along with your
//! plugin as a rust crate.
//!
//! For very simple examples, see the binariesnamed `qemu-<arch>` in this workspace.
//!
//! In addition, if you want to do wild stuff that "doesn't circumvent the GPL", you can build
//! a debug binary, use something like [goblin](https://github.com/m4b/goblin) to figure out
//! where to hook, bytepatch the binary, then run it with your hooks. Is that insane? Maybe,
//! but you can do it, and it's a lot more efficient to just have the binary as bytes to
//! do so.
//!
//! To use, just configure your feature flags appropriately (see the README) and then use
//! one of the `QEMU_<arch>` constants here to obtain your binary. Then, you can either
//! write it to disk and run it, or you can be very efficient and use something like
//! [memfd-exec](https://crates.io/crates/memfd-exec) to run it from memory directly, or on
//! a separate thread, whatever!
//!
//! To install with binaries, `cargo install qemu --features=binaries,plugins,lto`

pub const QEMU_VERSION: &str = "8.2.2";

#[cfg(all(feature = "aarch64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-aarch64
pub const QEMU_AARCH64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-aarch64"));

#[cfg(all(feature = "aarch64_be-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-aarch64_be
pub const QEMU_AARCH64_BE_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-aarch64_be"));

#[cfg(all(feature = "alpha-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-alpha
pub const QEMU_ALPHA_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-alpha"));

#[cfg(all(feature = "arm-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-arm
pub const QEMU_ARM_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-arm"));

#[cfg(all(feature = "armeb-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-armeb
pub const QEMU_ARMEB_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-armeb"));

#[cfg(all(feature = "cris-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-cris
pub const QEMU_CRIS_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-cris"));

#[cfg(all(feature = "hexagon-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-hexagon
pub const QEMU_HEXAGON_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-hexagon"));

#[cfg(all(feature = "hppa-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-hppa
pub const QEMU_HPPA_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-hppa"));

#[cfg(all(feature = "i386-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-i386
pub const QEMU_I386_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-i386"));

#[cfg(all(feature = "loongarch64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-loongarch64
pub const QEMU_LOONGARCH64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-loongarch64"));

#[cfg(all(feature = "m68k-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-m68k
pub const QEMU_M68K_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-m68k"));

#[cfg(all(feature = "microblaze-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-microblaze
pub const QEMU_MICROBLAZE_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-microblaze"));

#[cfg(all(feature = "microblazeel-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-microblazeel
pub const QEMU_MICROBLAZEEL_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-microblazeel"));

#[cfg(all(feature = "mips-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-mips
pub const QEMU_MIPS_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-mips"));

#[cfg(all(feature = "mips64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-mips64
pub const QEMU_MIPS64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-mips64"));

#[cfg(all(feature = "mips64el-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-mips64el
pub const QEMU_MIPS64EL_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-mips64el"));

#[cfg(all(feature = "mipsel-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-mipsel
pub const QEMU_MIPSEL_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-mipsel"));

#[cfg(all(feature = "mipsn32-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-mipsn32
pub const QEMU_MIPSN32_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-mipsn32"));

#[cfg(all(feature = "mipsn32el-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-mipsn32el
pub const QEMU_MIPSN32EL_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-mipsn32el"));

#[cfg(all(feature = "nios2-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-nios2
pub const QEMU_NIOS2_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-nios2"));

#[cfg(all(feature = "or1k-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-or1k
pub const QEMU_OR1K_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-or1k"));

#[cfg(all(feature = "ppc-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-ppc
pub const QEMU_PPC_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-ppc"));

#[cfg(all(feature = "ppc64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-ppc64
pub const QEMU_PPC64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-ppc64"));

#[cfg(all(feature = "ppc64le-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-ppc64le
pub const QEMU_PPC64LE_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-ppc64le"));

#[cfg(all(feature = "riscv32-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-riscv32
pub const QEMU_RISCV32_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-riscv32"));

#[cfg(all(feature = "riscv64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-riscv64
pub const QEMU_RISCV64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-riscv64"));

#[cfg(all(feature = "s390x-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-s390x
pub const QEMU_S390X_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-s390x"));

#[cfg(all(feature = "sh4-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-sh4
pub const QEMU_SH4_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-sh4"));

#[cfg(all(feature = "sh4eb-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-sh4eb
pub const QEMU_SH4EB_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-sh4eb"));

#[cfg(all(feature = "sparc-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-sparc
pub const QEMU_SPARC_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-sparc"));

#[cfg(all(feature = "sparc32plus-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-sparc32plus
pub const QEMU_SPARC32PLUS_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-sparc32plus"));

#[cfg(all(feature = "sparc64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-sparc64
pub const QEMU_SPARC64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-sparc64"));

#[cfg(all(feature = "x86_64-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-x86_64
pub const QEMU_X86_64_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-x86_64"));

#[cfg(all(feature = "xtensa-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-xtensa
pub const QEMU_XTENSA_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-xtensa"));

#[cfg(all(feature = "xtensaeb-linux-user", not(docs_rs)))]
/// QEMU binary for qemu-xtensaeb
pub const QEMU_XTENSAEB_LINUX_USER: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-xtensaeb"));

#[cfg(all(feature = "aarch64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-aarch64
pub const QEMU_AARCH64_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-aarch64"));

#[cfg(all(feature = "alpha-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-alpha
pub const QEMU_ALPHA_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-alpha"));

#[cfg(all(feature = "arm-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-arm
pub const QEMU_ARM_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-arm"));

#[cfg(all(feature = "avr-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-avr
pub const QEMU_AVR_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-avr"));

#[cfg(all(feature = "cris-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-cris
pub const QEMU_CRIS_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-cris"));

#[cfg(all(feature = "hppa-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-hppa
pub const QEMU_HPPA_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-hppa"));

#[cfg(all(feature = "i386-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-i386
pub const QEMU_I386_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-i386"));

#[cfg(all(feature = "loongarch64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-loongarch64
pub const QEMU_LOONGARCH64_SOFTMMU: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/qemu/bin/qemu-system-loongarch64"
));

#[cfg(all(feature = "m68k-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-m68k
pub const QEMU_M68K_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-m68k"));

#[cfg(all(feature = "microblaze-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-microblaze
pub const QEMU_MICROBLAZE_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-microblaze"));

#[cfg(all(feature = "microblazeel-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-microblazeel
pub const QEMU_MICROBLAZEEL_SOFTMMU: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/qemu/bin/qemu-system-microblazeel"
));

#[cfg(all(feature = "mips-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-mips
pub const QEMU_MIPS_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-mips"));

#[cfg(all(feature = "mips64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-mips64
pub const QEMU_MIPS64_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-mips64"));

#[cfg(all(feature = "mips64el-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-mips64el
pub const QEMU_MIPS64EL_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-mips64el"));

#[cfg(all(feature = "mipsel-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-mipsel
pub const QEMU_MIPSEL_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-mipsel"));

#[cfg(all(feature = "nios2-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-nios2
pub const QEMU_NIOS2_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-nios2"));

#[cfg(all(feature = "or1k-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-or1k
pub const QEMU_OR1K_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-or1k"));

#[cfg(all(feature = "ppc-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-ppc
pub const QEMU_PPC_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-ppc"));

#[cfg(all(feature = "ppc64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-ppc64
pub const QEMU_PPC64_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-ppc64"));

#[cfg(all(feature = "riscv32-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-riscv32
pub const QEMU_RISCV32_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-riscv32"));

#[cfg(all(feature = "riscv64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-riscv64
pub const QEMU_RISCV64_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-riscv64"));

#[cfg(all(feature = "rx-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-rx
pub const QEMU_RX_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-rx"));

#[cfg(all(feature = "s390x-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-s390x
pub const QEMU_S390X_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-s390x"));

#[cfg(all(feature = "sh4-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-sh4
pub const QEMU_SH4_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-sh4"));

#[cfg(all(feature = "sh4eb-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-sh4eb
pub const QEMU_SH4EB_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-sh4eb"));

#[cfg(all(feature = "sparc-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-sparc
pub const QEMU_SPARC_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-sparc"));

#[cfg(all(feature = "sparc64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-sparc64
pub const QEMU_SPARC64_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-sparc64"));

#[cfg(all(feature = "tricore-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-tricore
pub const QEMU_TRICORE_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-tricore"));

#[cfg(all(feature = "x86_64-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-x86_64
pub const QEMU_X86_64_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-x86_64"));

#[cfg(all(feature = "xtensa-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-xtensa
pub const QEMU_XTENSA_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-xtensa"));

#[cfg(all(feature = "xtensaeb-softmmu", not(docs_rs)))]
/// QEMU binary for qemu-system-xtensaeb
pub const QEMU_XTENSAEB_SOFTMMU: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/qemu/bin/qemu-system-xtensaeb"));
