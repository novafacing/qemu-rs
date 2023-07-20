use std::{env::var, fs::create_dir, path::{Path, PathBuf}, process::{Command, Stdio}};

use git2::{build::CheckoutBuilder, Repository};

const QEMU_GIT_URL: &str = "https://github.com/qemu/qemu.git";

fn get_target_list() -> Vec<String> {
    let mut target_architectures = Vec::new();

    if cfg!(feature = "qemu-system-aarch64") {
        target_architectures.push("aarch64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-alpha") {
        target_architectures.push("alpha-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-arm") {
        target_architectures.push("arm-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-avr") {
        target_architectures.push("avr-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-cris") {
        target_architectures.push("cris-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-hppa") {
        target_architectures.push("hppa-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-i386") {
        target_architectures.push("i386-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-loongarch64") {
        target_architectures.push("loongarch64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-m68k") {
        target_architectures.push("m68k-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-microblazeel") {
        target_architectures.push("microblazeel-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-microblaze") {
        target_architectures.push("microblaze-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-mips64el") {
        target_architectures.push("mips64el-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-mips64") {
        target_architectures.push("mips64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-mipsel") {
        target_architectures.push("mipsel-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-mips") {
        target_architectures.push("mips-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-nios2") {
        target_architectures.push("nios2-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-or1k") {
        target_architectures.push("or1k-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-ppc64") {
        target_architectures.push("ppc64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-ppc") {
        target_architectures.push("ppc-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-riscv32") {
        target_architectures.push("riscv32-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-riscv64") {
        target_architectures.push("riscv64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-rx") {
        target_architectures.push("rx-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-s390x") {
        target_architectures.push("s390x-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-sh4eb") {
        target_architectures.push("sh4eb-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-sh4") {
        target_architectures.push("sh4-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-sparc64") {
        target_architectures.push("sparc64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-sparc") {
        target_architectures.push("sparc-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-tricore") {
        target_architectures.push("tricore-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-x86_64") {
        target_architectures.push("x86_64-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-xtensaeb") {
        target_architectures.push("xtensaeb-softmmu".to_string());
    }
    if cfg!(feature = "qemu-system-xtensa") {
        target_architectures.push("xtensa-softmmu".to_string());
    }

    if cfg!(feature = "qemu-aarch64_be") {
        target_architectures.push("aarch64_be-linux-user".to_string());
    }
    if cfg!(feature = "qemu-aarch64") {
        target_architectures.push("aarch64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-alpha") {
        target_architectures.push("alpha-linux-user".to_string());
    }
    if cfg!(feature = "qemu-armeb") {
        target_architectures.push("armeb-linux-user".to_string());
    }
    if cfg!(feature = "qemu-arm") {
        target_architectures.push("arm-linux-user".to_string());
    }
    if cfg!(feature = "qemu-cris") {
        target_architectures.push("cris-linux-user".to_string());
    }
    if cfg!(feature = "qemu-hexagon") {
        target_architectures.push("hexagon-linux-user".to_string());
    }
    if cfg!(feature = "qemu-hppa") {
        target_architectures.push("hppa-linux-user".to_string());
    }
    if cfg!(feature = "qemu-i386") {
        target_architectures.push("i386-linux-user".to_string());
    }
    if cfg!(feature = "qemu-loongarch64") {
        target_architectures.push("loongarch64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-m68k") {
        target_architectures.push("m68k-linux-user".to_string());
    }
    if cfg!(feature = "qemu-microblazeel") {
        target_architectures.push("microblazeel-linux-user".to_string());
    }
    if cfg!(feature = "qemu-microblaze") {
        target_architectures.push("microblaze-linux-user".to_string());
    }
    if cfg!(feature = "qemu-mips64el") {
        target_architectures.push("mips64el-linux-user".to_string());
    }
    if cfg!(feature = "qemu-mips64") {
        target_architectures.push("mips64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-mipsel") {
        target_architectures.push("mipsel-linux-user".to_string());
    }
    if cfg!(feature = "qemu-mips") {
        target_architectures.push("mips-linux-user".to_string());
    }
    if cfg!(feature = "qemu-mipsn32el") {
        target_architectures.push("mipsn32el-linux-user".to_string());
    }
    if cfg!(feature = "qemu-mipsn32") {
        target_architectures.push("mipsn32-linux-user".to_string());
    }
    if cfg!(feature = "qemu-nios2") {
        target_architectures.push("nios2-linux-user".to_string());
    }
    if cfg!(feature = "qemu-or1k") {
        target_architectures.push("or1k-linux-user".to_string());
    }
    if cfg!(feature = "qemu-ppc64le") {
        target_architectures.push("ppc64le-linux-user".to_string());
    }
    if cfg!(feature = "qemu-ppc64") {
        target_architectures.push("ppc64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-ppc") {
        target_architectures.push("ppc-linux-user".to_string());
    }
    if cfg!(feature = "qemu-riscv32") {
        target_architectures.push("riscv32-linux-user".to_string());
    }
    if cfg!(feature = "qemu-riscv64") {
        target_architectures.push("riscv64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-s390x") {
        target_architectures.push("s390x-linux-user".to_string());
    }
    if cfg!(feature = "qemu-sh4eb") {
        target_architectures.push("sh4eb-linux-user".to_string());
    }
    if cfg!(feature = "qemu-sh4") {
        target_architectures.push("sh4-linux-user".to_string());
    }
    if cfg!(feature = "qemu-sparc32plus") {
        target_architectures.push("sparc32plus-linux-user".to_string());
    }
    if cfg!(feature = "qemu-sparc64") {
        target_architectures.push("sparc64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-sparc") {
        target_architectures.push("sparc-linux-user".to_string());
    }
    if cfg!(feature = "qemu-x86_64") {
        target_architectures.push("x86_64-linux-user".to_string());
    }
    if cfg!(feature = "qemu-xtensaeb") {
        target_architectures.push("xtensaeb-linux-user".to_string());
    }
    if cfg!(feature = "qemu-xtensa") {
        target_architectures.push("xtensa-linux-user".to_string());
    }
    target_architectures
}

fn build_qemu_configure_args<P: AsRef<Path>>(build_path: P) -> Vec<String> {
    let mut configure_args = Vec::new();

    // Conditional target options
    configure_args.push(format!("--target-list={}", get_target_list().join(",")));

    // Negative options
    if cfg!(feature = "disable-default-features") {
        configure_args.push("--disable-default-features".to_string());
    }

    if cfg!(feature = "disable-default-devices") {
        configure_args.push("--disable-default-devices".to_string());
    }

    if !cfg!(feature = "werror") {
        configure_args.push("--disable-werror".to_string());
    }

    if !cfg!(feature = "stack-protector") {
        configure_args.push("--disable-stack-protector".to_string());
    }

    if !cfg!(feature = "coroutine-pool") {
        configure_args.push("--disable-coroutine-pool".to_string());
    }

    if !cfg!(feature = "install-blobs") {
        configure_args.push("--disable-install-blobs".to_string());
    }

    if cfg!(feature = "static") {
        configure_args.push("--static".to_string());
    }

    if cfg!(feature = "debug") {
        configure_args.push("--enable-debug".to_string());
    }

    if cfg!(feature = "sanitizers") {
        configure_args.push("--enable-sanitizers".to_string());
    }

    if cfg!(feature = "tsan") {
        configure_args.push("--enable-tsan".to_string());
    }

    if cfg!(feature = "plugins") {
        configure_args.push("--enable-plugins".to_string());
    }

    if cfg!(feature = "cfi") {
        configure_args.push("--enable-cfi".to_string());
    }

    if cfg!(feature = "cfi-debug") {
        configure_args.push("--enable-cfi-debug".to_string());
    }

    if cfg!(feature = "debug-mutex") {
        configure_args.push("--enable-debug-mutex".to_string());
    }

    if cfg!(feature = "debug-stack-usage") {
        configure_args.push("--enable-debug-stack-usage".to_string());
    }

    if cfg!(feature = "fuzzing") {
        configure_args.push("--enable-fuzzing".to_string());
    }

    if cfg!(feature = "gcov") {
        configure_args.push("--enable-gcov".to_string());
    }

    if cfg!(feature = "gprof") {
        configure_args.push("--enable-gprof".to_string());
    }

    if cfg!(feature = "lto") {
        configure_args.push("--enable-lto".to_string());
    }

    if cfg!(feature = "module-upgrades") {
        configure_args.push("--enable-module-upgrades".to_string());
    }

    if cfg!(feature = "profiler") {
        configure_args.push("--enable-profiler".to_string());
    }

    if cfg!(feature = "qom-cast-debug") {
        configure_args.push("--enable-qom-cast-debug".to_string());
    }

    if cfg!(feature = "rng-none") {
        configure_args.push("--enable-rng-none".to_string());
    }

    if cfg!(feature = "strip") {
        configure_args.push("--enable-strip".to_string());
    }

    if cfg!(feature = "tcg-interpreter") {
        configure_args.push("--enable-tcg-interpreter".to_string());
    }

    if cfg!(feature = "disable-default-features") {
        if cfg!(feature = "alsa") {
            configure_args.push("--enable-alsa".to_string());
        }
        if cfg!(feature = "attr") {
            configure_args.push("--enable-attr".to_string());
        }
        if cfg!(feature = "auth-pam") {
            configure_args.push("--enable-auth-pam".to_string());
        }
        if cfg!(feature = "avx2") {
            configure_args.push("--enable-avx2".to_string());
        }
        if cfg!(feature = "avx512f") {
            configure_args.push("--enable-avx512f".to_string());
        }
        if cfg!(feature = "blkio") {
            configure_args.push("--enable-blkio".to_string());
        }
        if cfg!(feature = "bochs") {
            configure_args.push("--enable-bochs".to_string());
        }
        if cfg!(feature = "bpf") {
            configure_args.push("--enable-bpf".to_string());
        }
        if cfg!(feature = "brlapi") {
            configure_args.push("--enable-brlapi".to_string());
        }
        if cfg!(feature = "bzip2") {
            configure_args.push("--enable-bzip2".to_string());
        }
        if cfg!(feature = "canokey") {
            configure_args.push("--enable-canokey".to_string());
        }
        if cfg!(feature = "cap-ng") {
            configure_args.push("--enable-cap-ng".to_string());
        }
        if cfg!(feature = "capstone") {
            configure_args.push("--enable-capstone".to_string());
        }
        if cfg!(feature = "cloop") {
            configure_args.push("--enable-cloop".to_string());
        }
        if cfg!(feature = "cocoa") {
            configure_args.push("--enable-cocoa".to_string());
        }
        if cfg!(feature = "coreaudio") {
            configure_args.push("--enable-coreaudio".to_string());
        }
        if cfg!(feature = "crypto-afalg") {
            configure_args.push("--enable-crypto-afalg".to_string());
        }
        if cfg!(feature = "curl") {
            configure_args.push("--enable-curl".to_string());
        }
        if cfg!(feature = "curses") {
            configure_args.push("--enable-curses".to_string());
        }
        if cfg!(feature = "dbus-display") {
            configure_args.push("--enable-dbus-display".to_string());
        }
        if cfg!(feature = "dmg") {
            configure_args.push("--enable-dmg".to_string());
        }
        if cfg!(feature = "docs") {
            configure_args.push("--enable-docs".to_string());
        }
        if cfg!(feature = "dsound") {
            configure_args.push("--enable-dsound".to_string());
        }
        if cfg!(feature = "fuse") {
            configure_args.push("--enable-fuse".to_string());
        }
        if cfg!(feature = "fuse-lseek") {
            configure_args.push("--enable-fuse-lseek".to_string());
        }
        if cfg!(feature = "gcrypt") {
            configure_args.push("--enable-gcrypt".to_string());
        }
        if cfg!(feature = "gettext") {
            configure_args.push("--enable-gettext".to_string());
        }
        if cfg!(feature = "gio") {
            configure_args.push("--enable-gio".to_string());
        }
        if cfg!(feature = "glusterfs") {
            configure_args.push("--enable-glusterfs".to_string());
        }
        if cfg!(feature = "gnutls") {
            configure_args.push("--enable-gnutls".to_string());
        }
        if cfg!(feature = "gtk") {
            configure_args.push("--enable-gtk".to_string());
        }
        if cfg!(feature = "guest-agent") {
            configure_args.push("--enable-guest-agent".to_string());
        }
        if cfg!(feature = "guest-agent-msi") {
            configure_args.push("--enable-guest-agent-msi".to_string());
        }
        if cfg!(feature = "hax") {
            configure_args.push("--enable-hax".to_string());
        }
        if cfg!(feature = "hvf") {
            configure_args.push("--enable-hvf".to_string());
        }
        if cfg!(feature = "iconv") {
            configure_args.push("--enable-iconv".to_string());
        }
        if cfg!(feature = "jack") {
            configure_args.push("--enable-jack".to_string());
        }
        if cfg!(feature = "keyring") {
            configure_args.push("--enable-keyring".to_string());
        }
        if cfg!(feature = "kvm") {
            configure_args.push("--enable-kvm".to_string());
        }
        if cfg!(feature = "l2tpv3") {
            configure_args.push("--enable-l2tpv3".to_string());
        }
        if cfg!(feature = "libdaxctl") {
            configure_args.push("--enable-libdaxctl".to_string());
        }
        if cfg!(feature = "libiscsi") {
            configure_args.push("--enable-libiscsi".to_string());
        }
        if cfg!(feature = "libnfs") {
            configure_args.push("--enable-libnfs".to_string());
        }
        if cfg!(feature = "libpmem") {
            configure_args.push("--enable-libpmem".to_string());
        }
        if cfg!(feature = "libssh") {
            configure_args.push("--enable-libssh".to_string());
        }
        if cfg!(feature = "libudev") {
            configure_args.push("--enable-libudev".to_string());
        }
        if cfg!(feature = "libusb") {
            configure_args.push("--enable-libusb".to_string());
        }
        if cfg!(feature = "libvduse") {
            configure_args.push("--enable-libvduse".to_string());
        }
        if cfg!(feature = "linux-aio") {
            configure_args.push("--enable-linux-aio".to_string());
        }
        if cfg!(feature = "linux-io-uring") {
            configure_args.push("--enable-linux-io-uring".to_string());
        }
        if cfg!(feature = "live-block-migration") {
            configure_args.push("--enable-live-block-migration".to_string());
        }
        if cfg!(feature = "lzfse") {
            configure_args.push("--enable-lzfse".to_string());
        }
        if cfg!(feature = "lzo") {
            configure_args.push("--enable-lzo".to_string());
        }
        if cfg!(feature = "malloc-trim") {
            configure_args.push("--enable-malloc-trim".to_string());
        }
        if cfg!(feature = "membarrier") {
            configure_args.push("--enable-membarrier".to_string());
        }
        if cfg!(feature = "mpath") {
            configure_args.push("--enable-mpath".to_string());
        }
        if cfg!(feature = "multiprocess") {
            configure_args.push("--enable-multiprocess".to_string());
        }
        if cfg!(feature = "netmap") {
            configure_args.push("--enable-netmap".to_string());
        }
        if cfg!(feature = "nettle") {
            configure_args.push("--enable-nettle".to_string());
        }
        if cfg!(feature = "numa") {
            configure_args.push("--enable-numa".to_string());
        }
        if cfg!(feature = "nvmm") {
            configure_args.push("--enable-nvmm".to_string());
        }
        if cfg!(feature = "opengl") {
            configure_args.push("--enable-opengl".to_string());
        }
        if cfg!(feature = "oss") {
            configure_args.push("--enable-oss".to_string());
        }
        if cfg!(feature = "pa") {
            configure_args.push("--enable-pa".to_string());
        }
        if cfg!(feature = "parallels") {
            configure_args.push("--enable-parallels".to_string());
        }
        if cfg!(feature = "png") {
            configure_args.push("--enable-png".to_string());
        }
        if cfg!(feature = "pvrdma") {
            configure_args.push("--enable-pvrdma".to_string());
        }
        if cfg!(feature = "qcow1") {
            configure_args.push("--enable-qcow1".to_string());
        }
        if cfg!(feature = "qed") {
            configure_args.push("--enable-qed".to_string());
        }
        if cfg!(feature = "qga-vss") {
            configure_args.push("--enable-qga-vss".to_string());
        }
        if cfg!(feature = "rbd") {
            configure_args.push("--enable-rbd".to_string());
        }
        if cfg!(feature = "rdma") {
            configure_args.push("--enable-rdma".to_string());
        }
        if cfg!(feature = "replication") {
            configure_args.push("--enable-replication".to_string());
        }
        if cfg!(feature = "sdl") {
            configure_args.push("--enable-sdl".to_string());
        }
        if cfg!(feature = "sdl-image") {
            configure_args.push("--enable-sdl-image".to_string());
        }
        if cfg!(feature = "seccomp") {
            configure_args.push("--enable-seccomp".to_string());
        }
        if cfg!(feature = "selinux") {
            configure_args.push("--enable-selinux".to_string());
        }
        if cfg!(feature = "slirp") {
            configure_args.push("--enable-slirp".to_string());
        }
        if cfg!(feature = "slirp-smbd") {
            configure_args.push("--enable-slirp-smbd".to_string());
        }
        if cfg!(feature = "smartcard") {
            configure_args.push("--enable-smartcard".to_string());
        }
        if cfg!(feature = "snappy") {
            configure_args.push("--enable-snappy".to_string());
        }
        if cfg!(feature = "sndio") {
            configure_args.push("--enable-sndio".to_string());
        }
        if cfg!(feature = "sparse") {
            configure_args.push("--enable-sparse".to_string());
        }
        if cfg!(feature = "spice") {
            configure_args.push("--enable-spice".to_string());
        }
        if cfg!(feature = "spice-protocol") {
            configure_args.push("--enable-spice-protocol".to_string());
        }
        if cfg!(feature = "tcg") {
            configure_args.push("--enable-tcg".to_string());
        }
        if cfg!(feature = "tools") {
            configure_args.push("--enable-tools".to_string());
        }
        if cfg!(feature = "tpm") {
            configure_args.push("--enable-tpm".to_string());
        }
        if cfg!(feature = "u2f") {
            configure_args.push("--enable-u2f".to_string());
        }
        if cfg!(feature = "usb-redir") {
            configure_args.push("--enable-usb-redir".to_string());
        }
        if cfg!(feature = "vde") {
            configure_args.push("--enable-vde".to_string());
        }
        if cfg!(feature = "vdi") {
            configure_args.push("--enable-vdi".to_string());
        }
        if cfg!(feature = "vfio-user-server") {
            configure_args.push("--enable-vfio-user-server".to_string());
        }
        if cfg!(feature = "vhost-crypto") {
            configure_args.push("--enable-vhost-crypto".to_string());
        }
        if cfg!(feature = "vhost-kernel") {
            configure_args.push("--enable-vhost-kernel".to_string());
        }
        if cfg!(feature = "vhost-net") {
            configure_args.push("--enable-vhost-net".to_string());
        }
        if cfg!(feature = "vhost-user") {
            configure_args.push("--enable-vhost-user".to_string());
        }
        if cfg!(feature = "vhost-user-blk-server") {
            configure_args.push("--enable-vhost-user-blk-server".to_string());
        }
        if cfg!(feature = "vduse-blk-export") {
            configure_args.push("--enable-vduse-blk-export".to_string());
        }
        if cfg!(feature = "vhost-vdpa") {
            configure_args.push("--enable-vhost-vdpa".to_string());
        }
        if cfg!(feature = "virglrenderer") {
            configure_args.push("--enable-virglrenderer".to_string());
        }
        if cfg!(feature = "virtfs") {
            configure_args.push("--enable-virtfs".to_string());
        }
        if cfg!(feature = "virtiofsd") {
            configure_args.push("--enable-virtiofsd".to_string());
        }
        if cfg!(feature = "vmnet") {
            configure_args.push("--enable-vmnet".to_string());
        }
        if cfg!(feature = "vnc") {
            configure_args.push("--enable-vnc".to_string());
        }
        if cfg!(feature = "vnc-jpeg") {
            configure_args.push("--enable-vnc-jpeg".to_string());
        }
        if cfg!(feature = "vnc-sasl") {
            configure_args.push("--enable-vnc-sasl".to_string());
        }
        if cfg!(feature = "vte") {
            configure_args.push("--enable-vte".to_string());
        }
        if cfg!(feature = "vvfat") {
            configure_args.push("--enable-vvfat".to_string());
        }
        if cfg!(feature = "whpx") {
            configure_args.push("--enable-whpx".to_string());
        }
        if cfg!(feature = "xen") {
            configure_args.push("--enable-xen".to_string());
        }
        if cfg!(feature = "xen-pci-passthrough") {
            configure_args.push("--enable-xen-pci-passthrough".to_string());
        }
        if cfg!(feature = "xkbcommon") {
            configure_args.push("--enable-xkbcommon".to_string());
        }
        if cfg!(feature = "zstd") {
            configure_args.push("--enable-zstd".to_string());
        }
        if cfg!(feature = "pie") {
            configure_args.push("--enable-pie".to_string());
        }
        if cfg!(feature = "modules") {
            configure_args.push("--enable-modules".to_string());
        }
        if cfg!(feature = "debug-tcg") {
            configure_args.push("--enable-debug-tcg".to_string());
        }
        if cfg!(feature = "debug-info") {
            configure_args.push("--enable-debug-info".to_string());
        }
        if cfg!(feature = "safe-stack") {
            configure_args.push("--enable-safe-stack".to_string());
        }
    }

    // Unconditional arguments used to configure installation via cargo
    configure_args.push(format!("--prefix={}", build_path.as_ref().to_string_lossy()));

    configure_args
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let outdir_path = PathBuf::from(
        var("OUT_DIR").expect("OUT_DIR not set. Is build.rs being run correctly by cargo build?"),
    );

    let qemu_repo_path = outdir_path.join("qemu");
    let qemu_build_path = outdir_path.join("build");
    let qemu_install_path = outdir_path.join("install");

    let repo = if !qemu_repo_path.exists() {
        if let Ok(repo) = Repository::clone(QEMU_GIT_URL, &qemu_repo_path) {
            repo
        } else {
            let output = Command::new("git")
                .arg("clone")
                .arg(QEMU_GIT_URL)
                .arg(&qemu_repo_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run git clone")
            .wait_with_output()
            .expect("Failed to wait for git clone");

            if !output.status.success() {
                panic!(
                    "Failed to configure qemu:\nstdout:{}\nstderr:{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Repository::open(&qemu_repo_path).expect("Failed to open cli cloned repository")
        }
    } else {
        Repository::open(&qemu_repo_path).expect("Failed to open repository")
    };

    repo.checkout_head(Some(CheckoutBuilder::default().force()))
        .expect("Failed to checkout repository");

    let configure_args = build_qemu_configure_args(&qemu_install_path);

    let configure_prog = qemu_repo_path
        .join("configure")
        .canonicalize()
        .expect("Failed to canonicalize configure script path");

    if !configure_prog.exists() {
        panic!(
            "Failed to find configure script at {}",
            configure_prog.display()
        );
    }

    if !qemu_build_path.exists() {
        create_dir(&qemu_build_path).expect("Failed to create build directory");
    }

    if !qemu_install_path.exists() {
        create_dir(&qemu_install_path).expect("Failed to create install directory");
    }

    eprintln!(
        "Running configure script {:?} with args: {:?}",
        configure_prog, configure_args
    );

    let building_docs = var("DOCS_RS").is_ok();

    if !building_docs {
        let output = Command::new(configure_prog)
            .current_dir(&qemu_build_path)
            .args(&configure_args)
            .arg(
                repo
                    .path()
                    .parent()
                    .expect("Could not find parent of repo path"),
            )
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run make install")
            .wait_with_output()
            .expect("Failed to wait for make install");

        if !output.status.success() {
            panic!(
                "Failed to configure qemu:\nstdout:{}\nstderr:{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let output = Command::new("make")
            .current_dir(&qemu_build_path)
            .arg("-j")
            .arg(num_cpus::get().to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run make install")
            .wait_with_output()
            .expect("Failed to wait for make install");

        if !output.status.success() {
            panic!(
                "Failed to build qemu:\nstdout:{}\nstderr:{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }


        let output = Command::new("make")
            .current_dir(&qemu_build_path)
            .arg("install")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run make install")
            .wait_with_output()
            .expect("Failed to wait for make install");

        if !output.status.success() {
            panic!(
                "Failed to install qemu:\nstdout:{}\nstderr:{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let enabled_targets = get_target_list();

        for enabled_target in enabled_targets {
            let target_name = match enabled_target.contains("softmmu") {
                true => "qemu-system-".to_string() + &enabled_target.replace("-softmmu", ""),
                false => "qemu-".to_string() + &enabled_target.replace("-linux-user", ""),
            };
            let target_bin = qemu_install_path.join("bin").join(&target_name);
            if !target_bin.exists() {
                panic!("Failed to build target {:?}", target_bin);
            }
        }
    }
}
