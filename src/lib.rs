#[cfg(feature = "aarch64-softmmu")]
pub const QEMU_SYSTEM_AARCH64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-aarch64"
));

#[cfg(feature = "alpha-softmmu")]
pub const QEMU_SYSTEM_ALPHA: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-alpha"
));

#[cfg(feature = "arm-softmmu")]
pub const QEMU_SYSTEM_ARM: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-arm"
));

#[cfg(feature = "avr-softmmu")]
pub const QEMU_SYSTEM_AVR: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-avr"
));

#[cfg(feature = "cris-softmmu")]
pub const QEMU_SYSTEM_CRIS: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-cris"
));

#[cfg(feature = "hppa-softmmu")]
pub const QEMU_SYSTEM_HPPA: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-hppa"
));

#[cfg(feature = "i386-softmmu")]
pub const QEMU_SYSTEM_I386: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-i386"
));

#[cfg(feature = "loongarch64-softmmu")]
pub const QEMU_SYSTEM_LOONGARCH64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-loongarch64"
));

#[cfg(feature = "m68k-softmmu")]
pub const QEMU_SYSTEM_M68K: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-m68k"
));

#[cfg(feature = "microblazeel-softmmu")]
pub const QEMU_SYSTEM_MICROBLAZEEL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-microblazeel"
));

#[cfg(feature = "microblaze-softmmu")]
pub const QEMU_SYSTEM_MICROBLAZE: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-microblaze"
));

#[cfg(feature = "mips64el-softmmu")]
pub const QEMU_SYSTEM_MIPS64EL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mips64el"
));

#[cfg(feature = "mips64-softmmu")]
pub const QEMU_SYSTEM_MIPS64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mips64"
));

#[cfg(feature = "mipsel-softmmu")]
pub const QEMU_SYSTEM_MIPSEL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mipsel"
));

#[cfg(feature = "mips-softmmu")]
pub const QEMU_SYSTEM_MIPS: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-mips"
));

#[cfg(feature = "nios2-softmmu")]
pub const QEMU_SYSTEM_NIOS2: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-nios2"
));

#[cfg(feature = "or1k-softmmu")]
pub const QEMU_SYSTEM_OR1K: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-or1k"
));

#[cfg(feature = "ppc64-softmmu")]
pub const QEMU_SYSTEM_PPC64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-ppc64"
));

#[cfg(feature = "ppc-softmmu")]
pub const QEMU_SYSTEM_PPC: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-ppc"
));

#[cfg(feature = "riscv32-softmmu")]
pub const QEMU_SYSTEM_RISCV32: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-riscv32"
));

#[cfg(feature = "riscv64-softmmu")]
pub const QEMU_SYSTEM_RISCV64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-riscv64"
));

#[cfg(feature = "rx-softmmu")]
pub const QEMU_SYSTEM_RX: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-rx"
));

#[cfg(feature = "s390x-softmmu")]
pub const QEMU_SYSTEM_S390X: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-s390x"
));

#[cfg(feature = "sh4eb-softmmu")]
pub const QEMU_SYSTEM_SH4EB: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-sh4eb"
));

#[cfg(feature = "sh4-softmmu")]
pub const QEMU_SYSTEM_SH4: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-sh4"
));

#[cfg(feature = "sparc64-softmmu")]
pub const QEMU_SYSTEM_SPARC64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-sparc64"
));

#[cfg(feature = "sparc-softmmu")]
pub const QEMU_SYSTEM_SPARC: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-sparc"
));

#[cfg(feature = "tricore-softmmu")]
pub const QEMU_SYSTEM_TRICORE: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-tricore"
));

#[cfg(feature = "x86_64-softmmu")]
pub const QEMU_SYSTEM_X86_64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-x86_64"
));

#[cfg(feature = "xtensaeb-softmmu")]
pub const QEMU_SYSTEM_XTENSAEB: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-xtensaeb"
));

#[cfg(feature = "xtensa-softmmu")]
pub const QEMU_SYSTEM_XTENSA: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-system-xtensa"
));

#[cfg(feature = "aarch64_be-linux-user")]
pub const QEMU_USER_AARCH64_BE: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-aarch64_be"
));

#[cfg(feature = "aarch64-linux-user")]
pub const QEMU_USER_AARCH64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-aarch64"
));

#[cfg(feature = "alpha-linux-user")]
pub const QEMU_USER_ALPHA: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-alpha"));

#[cfg(feature = "armeb-linux-user")]
pub const QEMU_USER_ARMEB: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-armeb"));

#[cfg(feature = "arm-linux-user")]
pub const QEMU_USER_ARM: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-arm"));

#[cfg(feature = "cris-linux-user")]
pub const QEMU_USER_CRIS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-cris"));

#[cfg(feature = "hexagon-linux-user")]
pub const QEMU_USER_HEXAGON: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-hexagon"
));

#[cfg(feature = "hppa-linux-user")]
pub const QEMU_USER_HPPA: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-hppa"));

#[cfg(feature = "i386-linux-user")]
pub const QEMU_USER_I386: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-i386"));

#[cfg(feature = "loongarch64-linux-user")]
pub const QEMU_USER_LOONGARCH64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-loongarch64"
));

#[cfg(feature = "m68k-linux-user")]
pub const QEMU_USER_M68K: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-m68k"));

#[cfg(feature = "microblazeel-linux-user")]
pub const QEMU_USER_MICROBLAZEEL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-microblazeel"
));

#[cfg(feature = "microblaze-linux-user")]
pub const QEMU_USER_MICROBLAZE: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-microblaze"
));

#[cfg(feature = "mips64el-linux-user")]
pub const QEMU_USER_MIPS64EL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-mips64el"
));

#[cfg(feature = "mips64-linux-user")]
pub const QEMU_USER_MIPS64: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips64"));

#[cfg(feature = "mipsel-linux-user")]
pub const QEMU_USER_MIPSEL: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mipsel"));

#[cfg(feature = "mips-linux-user")]
pub const QEMU_USER_MIPS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips"));

#[cfg(feature = "mipsn32el-linux-user")]
pub const QEMU_USER_MIPSN32EL: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-mipsn32el"
));

#[cfg(feature = "mipsn32-linux-user")]
pub const QEMU_USER_MIPSN32: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-mipsn32"
));

#[cfg(feature = "nios2-linux-user")]
pub const QEMU_USER_NIOS2: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-nios2"));

#[cfg(feature = "or1k-linux-user")]
pub const QEMU_USER_OR1K: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-or1k"));

#[cfg(feature = "ppc64le-linux-user")]
pub const QEMU_USER_PPC64LE: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-ppc64le"
));

#[cfg(feature = "ppc64-linux-user")]
pub const QEMU_USER_PPC64: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc64"));

#[cfg(feature = "ppc-linux-user")]
pub const QEMU_USER_PPC: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc"));

#[cfg(feature = "riscv32-linux-user")]
pub const QEMU_USER_RISCV32: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-riscv32"
));

#[cfg(feature = "riscv64-linux-user")]
pub const QEMU_USER_RISCV64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-riscv64"
));

#[cfg(feature = "s390x-linux-user")]
pub const QEMU_USER_S390X: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-s390x"));

#[cfg(feature = "sh4eb-linux-user")]
pub const QEMU_USER_SH4EB: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4eb"));

#[cfg(feature = "sh4-linux-user")]
pub const QEMU_USER_SH4: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4"));

#[cfg(feature = "sparc32plus-linux-user")]
pub const QEMU_USER_SPARC32PLUS: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-sparc32plus"
));

#[cfg(feature = "sparc64-linux-user")]
pub const QEMU_USER_SPARC64: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-sparc64"
));

#[cfg(feature = "sparc-linux-user")]
pub const QEMU_USER_SPARC: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sparc"));

#[cfg(feature = "x86_64-linux-user")]
pub const QEMU_USER_X86_64: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-x86_64"));

#[cfg(feature = "xtensaeb-linux-user")]
pub const QEMU_USER_XTENSAEB: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    "/install",
    "/bin",
    "/qemu-xtensaeb"
));

#[cfg(feature = "xtensa-linux-user")]
pub const QEMU_USER_XTENSA: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-xtensa"));
