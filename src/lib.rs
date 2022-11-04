#[cfg(feature = "qemu-system-aarch64")]
pub fn qemu_system_aarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-aarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-alpha")]
pub fn qemu_system_alpha() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-alpha"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-arm")]
pub fn qemu_system_arm() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-arm"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-avr")]
pub fn qemu_system_avr() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-avr"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-cris")]
pub fn qemu_system_cris() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-cris"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-hppa")]
pub fn qemu_system_hppa() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-hppa"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-i386")]
pub fn qemu_system_i386() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-i386"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-loongarch64")]
pub fn qemu_system_loongarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-loongarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-m68k")]
pub fn qemu_system_m68k() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-m68k"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-microblazeel")]
pub fn qemu_system_microblazeel() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-microblazeel"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-microblaze")]
pub fn qemu_system_microblaze() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-microblaze"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-mips64el")]
pub fn qemu_system_mips64el() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mips64el"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-mips64")]
pub fn qemu_system_mips64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mips64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-mipsel")]
pub fn qemu_system_mipsel() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mipsel"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-mips")]
pub fn qemu_system_mips() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-mips"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-nios2")]
pub fn qemu_system_nios2() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-nios2"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-or1k")]
pub fn qemu_system_or1k() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-or1k"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-ppc64")]
pub fn qemu_system_ppc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-ppc64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-ppc")]
pub fn qemu_system_ppc() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-ppc"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-riscv32")]
pub fn qemu_system_riscv32() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-riscv32"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-riscv64")]
pub fn qemu_system_riscv64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-riscv64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-rx")]
pub fn qemu_system_rx() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-rx"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-s390x")]
pub fn qemu_system_s390x() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-s390x"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-sh4eb")]
pub fn qemu_system_sh4eb() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sh4eb"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-sh4")]
pub fn qemu_system_sh4() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sh4"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-sparc64")]
pub fn qemu_system_sparc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sparc64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-sparc")]
pub fn qemu_system_sparc() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-sparc"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-tricore")]
pub fn qemu_system_tricore() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-tricore"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-x86_64")]
pub fn qemu_system_x86_64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-x86_64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-xtensaeb")]
pub fn qemu_system_xtensaeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-xtensaeb"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-system-xtensa")]
pub fn qemu_system_xtensa() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-system-xtensa"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-aarch64_be")]
pub fn qemu_aarch64_be() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-aarch64_be"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-aarch64")]
pub fn qemu_aarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-aarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-alpha")]
pub fn qemu_alpha() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-alpha"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-armeb")]
pub fn qemu_armeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-armeb"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-arm")]
pub fn qemu_arm() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-arm"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-cris")]
pub fn qemu_cris() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-cris"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-hexagon")]
pub fn qemu_hexagon() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-hexagon"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-hppa")]
pub fn qemu_hppa() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-hppa"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-i386")]
pub fn qemu_i386() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-i386"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-loongarch64")]
pub fn qemu_loongarch64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-loongarch64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-m68k")]
pub fn qemu_m68k() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-m68k"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-microblazeel")]
pub fn qemu_microblazeel() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-microblazeel"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-microblaze")]
pub fn qemu_microblaze() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-microblaze"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mips64el")]
pub fn qemu_mips64el() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-mips64el"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mips64")]
pub fn qemu_mips64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mipsel")]
pub fn qemu_mipsel() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mipsel"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mips")]
pub fn qemu_mips() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mipsn32el")]
pub fn qemu_mipsn32el() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-mipsn32el"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mipsn32")]
pub fn qemu_mipsn32() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-mipsn32"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-nios2")]
pub fn qemu_nios2() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-nios2"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-or1k")]
pub fn qemu_or1k() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-or1k"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-ppc64le")]
pub fn qemu_ppc64le() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-ppc64le"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-ppc64")]
pub fn qemu_ppc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-ppc")]
pub fn qemu_ppc() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-riscv32")]
pub fn qemu_riscv32() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-riscv32"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-riscv64")]
pub fn qemu_riscv64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-riscv64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-s390x")]
pub fn qemu_s390x() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-s390x"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sh4eb")]
pub fn qemu_sh4eb() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4eb"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sh4")]
pub fn qemu_sh4() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sparc32plus")]
pub fn qemu_sparc32plus() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-sparc32plus"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sparc64")]
pub fn qemu_sparc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-sparc64"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sparc")]
pub fn qemu_sparc() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sparc"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-x86_64")]
pub fn qemu_x86_64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-x86_64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-xtensaeb")]
pub fn qemu_xtensaeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/install",
        "/bin",
        "/qemu-xtensaeb"
    ));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-xtensa")]
pub fn qemu_xtensa() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-xtensa"));
    return PROGRAM.to_vec();
}
