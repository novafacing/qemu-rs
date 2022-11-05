#[cfg(feature = "qemu-system-aarch64")]
/// Returns the qemu-system-aarch64 binary
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
/// Returns the qemu-system-alpha binary
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
/// Returns the qemu-system-arm binary
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
/// Returns the qemu-system-avr binary
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
/// Returns the qemu-system-cris binary
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
/// Returns the qemu-system-hppa binary
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
/// Returns the qemu-system-i386 binary
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
/// Returns the qemu-system-loongarch64 binary
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
/// Returns the qemu-system-m68k binary
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
/// Returns the qemu-system-microblazeel binary
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
/// Returns the qemu-system-microblaze binary
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
/// Returns the qemu-system-mips64el binary
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
/// Returns the qemu-system-mips64 binary
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
/// Returns the qemu-system-mipsel binary
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
/// Returns the qemu-system-mips binary
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
/// Returns the qemu-system-nios2 binary
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
/// Returns the qemu-system-or1k binary
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
/// Returns the qemu-system-ppc64 binary
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
/// Returns the qemu-system-ppc binary
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
/// Returns the qemu-system-riscv32 binary
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
/// Returns the qemu-system-riscv64 binary
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
/// Returns the qemu-system-rx binary
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
/// Returns the qemu-system-s390x binary
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
/// Returns the qemu-system-sh4eb binary
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
/// Returns the qemu-system-sh4 binary
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
/// Returns the qemu-system-sparc64 binary
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
/// Returns the qemu-system-sparc binary
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
/// Returns the qemu-system-tricore binary
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
/// Returns the qemu-system-x86_64 binary
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
/// Returns the qemu-system-xtensaeb binary
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
/// Returns the qemu-system-xtensa binary
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
/// Returns the qemu-aarch64_be binary
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
/// Returns the qemu-aarch64 binary
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
/// Returns the qemu-alpha binary
pub fn qemu_alpha() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-alpha"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-armeb")]
/// Returns the qemu-armeb binary
pub fn qemu_armeb() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-armeb"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-arm")]
/// Returns the qemu-arm binary
pub fn qemu_arm() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-arm"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-cris")]
/// Returns the qemu-cris binary
pub fn qemu_cris() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-cris"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-hexagon")]
/// Returns the qemu-hexagon binary
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
/// Returns the qemu-hppa binary
pub fn qemu_hppa() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-hppa"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-i386")]
/// Returns the qemu-i386 binary
pub fn qemu_i386() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-i386"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-loongarch64")]
/// Returns the qemu-loongarch64 binary
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
/// Returns the qemu-m68k binary
pub fn qemu_m68k() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-m68k"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-microblazeel")]
/// Returns the qemu-microblazeel binary
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
/// Returns the qemu-microblaze binary
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
/// Returns the qemu-mips64el binary
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
/// Returns the qemu-mips64 binary
pub fn qemu_mips64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mipsel")]
/// Returns the qemu-mipsel binary
pub fn qemu_mipsel() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mipsel"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mips")]
/// Returns the qemu-mips binary
pub fn qemu_mips() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-mips"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-mipsn32el")]
/// Returns the qemu-mipsn32el binary
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
/// Returns the qemu-mipsn32 binary
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
/// Returns the qemu-nios2 binary
pub fn qemu_nios2() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-nios2"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-or1k")]
/// Returns the qemu-or1k binary
pub fn qemu_or1k() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-or1k"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-ppc64le")]
/// Returns the qemu-ppc64le binary
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
/// Returns the qemu-ppc64 binary
pub fn qemu_ppc64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-ppc")]
/// Returns the qemu-ppc binary
pub fn qemu_ppc() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-ppc"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-riscv32")]
/// Returns the qemu-riscv32 binary
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
/// Returns the qemu-riscv64 binary
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
/// Returns the qemu-s390x binary
pub fn qemu_s390x() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-s390x"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sh4eb")]
/// Returns the qemu-sh4eb binary
pub fn qemu_sh4eb() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4eb"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sh4")]
/// Returns the qemu-sh4 binary
pub fn qemu_sh4() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sh4"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-sparc32plus")]
/// Returns the qemu-sparc32plus binary
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
/// Returns the qemu-sparc64 binary
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
/// Returns the qemu-sparc binary
pub fn qemu_sparc() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-sparc"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-x86_64")]
/// Returns the qemu-x86_64 binary
pub fn qemu_x86_64() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-x86_64"));
    return PROGRAM.to_vec();
}

#[cfg(feature = "qemu-xtensaeb")]
/// Returns the qemu-xtensaeb binary
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
/// Returns the qemu-xtensa binary
pub fn qemu_xtensa() -> Vec<u8> {
    pub const PROGRAM: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/install", "/bin", "/qemu-xtensa"));
    return PROGRAM.to_vec();
}
