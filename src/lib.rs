#[cfg(feature = "aarch64-softmmu")]
pub fn qemu_system_aarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-aarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "alpha-softmmu")]
pub fn qemu_system_alpha() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-alpha"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "arm-softmmu")]
pub fn qemu_system_arm() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-arm"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "avr-softmmu")]
pub fn qemu_system_avr() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-avr"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "cris-softmmu")]
pub fn qemu_system_cris() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-cris"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "hppa-softmmu")]
pub fn qemu_system_hppa() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-hppa"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "i386-softmmu")]
pub fn qemu_system_i386() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-i386"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "loongarch64-softmmu")]
pub fn qemu_system_loongarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-loongarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "m68k-softmmu")]
pub fn qemu_system_m68k() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-m68k"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "microblazeel-softmmu")]
pub fn qemu_system_microblazeel() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-microblazeel"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "microblaze-softmmu")]
pub fn qemu_system_microblaze() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-microblaze"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mips64el-softmmu")]
pub fn qemu_system_mips64el() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mips64el"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mips64-softmmu")]
pub fn qemu_system_mips64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mips64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mipsel-softmmu")]
pub fn qemu_system_mipsel() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mipsel"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mips-softmmu")]
pub fn qemu_system_mips() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mips"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "nios2-softmmu")]
pub fn qemu_system_nios2() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-nios2"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "or1k-softmmu")]
pub fn qemu_system_or1k() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-or1k"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "ppc64-softmmu")]
pub fn qemu_system_ppc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-ppc64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "ppc-softmmu")]
pub fn qemu_system_ppc() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-ppc"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "riscv32-softmmu")]
pub fn qemu_system_riscv32() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-riscv32"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "riscv64-softmmu")]
pub fn qemu_system_riscv64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-riscv64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "rx-softmmu")]
pub fn qemu_system_rx() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-rx"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "s390x-softmmu")]
pub fn qemu_system_s390x() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-s390x"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sh4eb-softmmu")]
pub fn qemu_system_sh4eb() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sh4eb"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sh4-softmmu")]
pub fn qemu_system_sh4() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sh4"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sparc64-softmmu")]
pub fn qemu_system_sparc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sparc64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sparc-softmmu")]
pub fn qemu_system_sparc() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sparc"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "tricore-softmmu")]
pub fn qemu_system_tricore() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-tricore"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "x86_64-softmmu")]
pub fn qemu_system_x86_64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-x86_64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "xtensaeb-softmmu")]
pub fn qemu_system_xtensaeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-xtensaeb"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "xtensa-softmmu")]
pub fn qemu_system_xtensa() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-xtensa"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "aarch64_be-linux-user")]
pub fn qemu_user_aarch64_be() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-aarch64_be"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "aarch64-linux-user")]
pub fn qemu_user_aarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-aarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "alpha-linux-user")]
pub fn qemu_user_alpha() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-alpha"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "armeb-linux-user")]
pub fn qemu_user_armeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-armeb"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "arm-linux-user")]
pub fn qemu_user_arm() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-arm"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "cris-linux-user")]
pub fn qemu_user_cris() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-cris"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "hexagon-linux-user")]
pub fn qemu_user_hexagon() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-hexagon"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "hppa-linux-user")]
pub fn qemu_user_hppa() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-hppa"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "i386-linux-user")]
pub fn qemu_user_i386() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-i386"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "loongarch64-linux-user")]
pub fn qemu_user_loongarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-loongarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "m68k-linux-user")]
pub fn qemu_user_m68k() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-m68k"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "microblazeel-linux-user")]
pub fn qemu_user_microblazeel() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-microblazeel"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "microblaze-linux-user")]
pub fn qemu_user_microblaze() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-microblaze"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mips64el-linux-user")]
pub fn qemu_user_mips64el() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-mips64el"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mips64-linux-user")]
pub fn qemu_user_mips64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mipsel-linux-user")]
pub fn qemu_user_mipsel() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mipsel"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mips-linux-user")]
pub fn qemu_user_mips() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mipsn32el-linux-user")]
pub fn qemu_user_mipsn32el() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-mipsn32el"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "mipsn32-linux-user")]
pub fn qemu_user_mipsn32() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-mipsn32"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "nios2-linux-user")]
pub fn qemu_user_nios2() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-nios2"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "or1k-linux-user")]
pub fn qemu_user_or1k() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-or1k"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "ppc64le-linux-user")]
pub fn qemu_user_ppc64le() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-ppc64le"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "ppc64-linux-user")]
pub fn qemu_user_ppc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "ppc-linux-user")]
pub fn qemu_user_ppc() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "riscv32-linux-user")]
pub fn qemu_user_riscv32() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-riscv32"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "riscv64-linux-user")]
pub fn qemu_user_riscv64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-riscv64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "s390x-linux-user")]
pub fn qemu_user_s390x() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-s390x"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sh4eb-linux-user")]
pub fn qemu_user_sh4eb() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4eb"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sh4-linux-user")]
pub fn qemu_user_sh4() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sparc32plus-linux-user")]
pub fn qemu_user_sparc32plus() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-sparc32plus"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sparc64-linux-user")]
pub fn qemu_user_sparc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-sparc64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "sparc-linux-user")]
pub fn qemu_user_sparc() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sparc"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "x86_64-linux-user")]
pub fn qemu_user_x86_64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-x86_64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "xtensaeb-linux-user")]
pub fn qemu_user_xtensaeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-xtensaeb"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "xtensa-linux-user")]
pub fn qemu_user_xtensa() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-xtensa"));
    return PROGRAM.to_vec();
}
