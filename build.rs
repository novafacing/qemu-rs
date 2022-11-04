use std::{env::var, path::PathBuf, process::Command};

use git2::{
    build::{self, CheckoutBuilder},
    Repository,
};

const QEMU_GIT_URL: &str = "https://github.com/qemu/qemu.git";

fn get_target_list() -> Vec<String> {
    let mut target_architectures = Vec::new();

    if cfg!(feature = "aarch64-softmmu") {
        target_architectures.push("aarch64-softmmu".to_string());
    }
    if cfg!(feature = "alpha-softmmu") {
        target_architectures.push("alpha-softmmu".to_string());
    }
    if cfg!(feature = "arm-softmmu") {
        target_architectures.push("arm-softmmu".to_string());
    }
    if cfg!(feature = "avr-softmmu") {
        target_architectures.push("avr-softmmu".to_string());
    }
    if cfg!(feature = "cris-softmmu") {
        target_architectures.push("cris-softmmu".to_string());
    }
    if cfg!(feature = "hppa-softmmu") {
        target_architectures.push("hppa-softmmu".to_string());
    }
    if cfg!(feature = "i386-softmmu") {
        target_architectures.push("i386-softmmu".to_string());
    }
    if cfg!(feature = "loongarch-softmmu") {
        target_architectures.push("loongarch-softmmu".to_string());
    }
    if cfg!(feature = "m68k-softmmu") {
        target_architectures.push("m68k-softmmu".to_string());
    }
    if cfg!(feature = "microblazeel-softmmu") {
        target_architectures.push("microblazeel-softmmu".to_string());
    }
    if cfg!(feature = "microblaze-softmmu") {
        target_architectures.push("microblaze-softmmu".to_string());
    }
    if cfg!(feature = "mips64el-softmmu") {
        target_architectures.push("mips64el-softmmu".to_string());
    }
    if cfg!(feature = "mips64-softmmu") {
        target_architectures.push("mips64-softmmu".to_string());
    }
    if cfg!(feature = "mipsel-softmmu") {
        target_architectures.push("mipsel-softmmu".to_string());
    }
    if cfg!(feature = "mips-softmmu") {
        target_architectures.push("mips-softmmu".to_string());
    }
    if cfg!(feature = "nios2-softmmu") {
        target_architectures.push("nios2-softmmu".to_string());
    }
    if cfg!(feature = "or1k-softmmu") {
        target_architectures.push("or1k-softmmu".to_string());
    }
    if cfg!(feature = "ppc64-softmmu") {
        target_architectures.push("ppc64-softmmu".to_string());
    }
    if cfg!(feature = "ppc-softmmu") {
        target_architectures.push("ppc-softmmu".to_string());
    }
    if cfg!(feature = "riscv32-softmmu") {
        target_architectures.push("riscv32-softmmu".to_string());
    }
    if cfg!(feature = "riscv64-softmmu") {
        target_architectures.push("riscv64-softmmu".to_string());
    }
    if cfg!(feature = "rx-softmmu") {
        target_architectures.push("rx-softmmu".to_string());
    }
    if cfg!(feature = "s390x-softmmu") {
        target_architectures.push("s390x-softmmu".to_string());
    }
    if cfg!(feature = "sh4eb-softmmu") {
        target_architectures.push("sh4eb-softmmu".to_string());
    }
    if cfg!(feature = "sh4-softmmu") {
        target_architectures.push("sh4-softmmu".to_string());
    }
    if cfg!(feature = "sparc64-softmmu") {
        target_architectures.push("sparc64-softmmu".to_string());
    }
    if cfg!(feature = "sparc-softmmu") {
        target_architectures.push("sparc-softmmu".to_string());
    }
    if cfg!(feature = "tricore-softmmu") {
        target_architectures.push("tricore-softmmu".to_string());
    }
    if cfg!(feature = "x86_64-softmmu") {
        target_architectures.push("x86_64-softmmu".to_string());
    }
    if cfg!(feature = "xtensaeb-softmmu") {
        target_architectures.push("xtensaeb-softmmu".to_string());
    }
    if cfg!(feature = "xtensa-softmmu") {
        target_architectures.push("xtensa-softmmu".to_string());
    }

    if cfg!(feature = "aarch64_be-linux-user") {
        target_architectures.push("aarch64_be-linux-user".to_string());
    }
    if cfg!(feature = "aarch64-linux-user") {
        target_architectures.push("aarch64-linux-user".to_string());
    }
    if cfg!(feature = "alpha-linux-user") {
        target_architectures.push("alpha-linux-user".to_string());
    }
    if cfg!(feature = "armeb-linux-user") {
        target_architectures.push("armeb-linux-user".to_string());
    }
    if cfg!(feature = "arm-linux-user") {
        target_architectures.push("arm-linux-user".to_string());
    }
    if cfg!(feature = "cris-linux-user") {
        target_architectures.push("cris-linux-user".to_string());
    }
    if cfg!(feature = "hexagon-linux-user") {
        target_architectures.push("hexagon-linux-user".to_string());
    }
    if cfg!(feature = "hppa-linux-user") {
        target_architectures.push("hppa-linux-user".to_string());
    }
    if cfg!(feature = "i386-linux-user") {
        target_architectures.push("i386-linux-user".to_string());
    }
    if cfg!(feature = "loongarch64-linux-user") {
        target_architectures.push("loongarch64-linux-user".to_string());
    }
    if cfg!(feature = "m68k-linux-user") {
        target_architectures.push("m68k-linux-user".to_string());
    }
    if cfg!(feature = "microblazeel-linux-user") {
        target_architectures.push("microblazeel-linux-user".to_string());
    }
    if cfg!(feature = "microblaze-linux-user") {
        target_architectures.push("microblaze-linux-user".to_string());
    }
    if cfg!(feature = "mips64el-linux-user") {
        target_architectures.push("mips64el-linux-user".to_string());
    }
    if cfg!(feature = "mips64-linux-user") {
        target_architectures.push("mips64-linux-user".to_string());
    }
    if cfg!(feature = "mipsel-linux-user") {
        target_architectures.push("mipsel-linux-user".to_string());
    }
    if cfg!(feature = "mips-linux-user") {
        target_architectures.push("mips-linux-user".to_string());
    }
    if cfg!(feature = "mipsn32el-linux-user") {
        target_architectures.push("mipsn32el-linux-user".to_string());
    }
    if cfg!(feature = "mipsn32-linux-user") {
        target_architectures.push("mipsn32-linux-user".to_string());
    }
    if cfg!(feature = "nios2-linux-user") {
        target_architectures.push("nios2-linux-user".to_string());
    }
    if cfg!(feature = "or1k-linux-user") {
        target_architectures.push("or1k-linux-user".to_string());
    }
    if cfg!(feature = "ppc64le-linux-user") {
        target_architectures.push("ppc64le-linux-user".to_string());
    }
    if cfg!(feature = "ppc64-linux-user") {
        target_architectures.push("ppc64-linux-user".to_string());
    }
    if cfg!(feature = "ppc-linux-user") {
        target_architectures.push("ppc-linux-user".to_string());
    }
    if cfg!(feature = "riscv32-linux-user") {
        target_architectures.push("riscv32-linux-user".to_string());
    }
    if cfg!(feature = "riscv64-linux-user") {
        target_architectures.push("riscv64-linux-user".to_string());
    }
    if cfg!(feature = "s390x-linux-user") {
        target_architectures.push("s390x-linux-user".to_string());
    }
    if cfg!(feature = "sh4eb-linux-user") {
        target_architectures.push("sh4eb-linux-user".to_string());
    }
    if cfg!(feature = "sh4-linux-user") {
        target_architectures.push("sh4-linux-user".to_string());
    }
    if cfg!(feature = "sparc32plus-linux-user") {
        target_architectures.push("sparc32plus-linux-user".to_string());
    }
    if cfg!(feature = "sparc64-linux-user") {
        target_architectures.push("sparc64-linux-user".to_string());
    }
    if cfg!(feature = "sparc-linux-user") {
        target_architectures.push("sparc-linux-user".to_string());
    }
    if cfg!(feature = "x86_64-linux-user") {
        target_architectures.push("x86_64-linux-user".to_string());
    }
    if cfg!(feature = "xtensaeb-linux-user") {
        target_architectures.push("xtensaeb-linux-user".to_string());
    }
    if cfg!(feature = "xtensa-linux-user") {
        target_architectures.push("xtensa-linux-user".to_string());
    }
    target_architectures
}

fn build_qemu_configure_args(build_path: &PathBuf) -> Vec<String> {
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
    configure_args.push(format!("--prefix={}", build_path.to_string_lossy()));

    configure_args
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let outdir_path = PathBuf::from(
        var("OUT_DIR").expect("OUT_DIR not set. Is build.rs being run correctly by cargo build?"),
    );

    let qemu_repo_path = outdir_path.join("qemu");
    let qemu_build_path = qemu_repo_path.join("build");
    let qemu_install_path = qemu_repo_path.join("install");

    let repo = match Repository::clone(QEMU_GIT_URL, &qemu_repo_path) {
        Err(_) => Repository::open(&qemu_repo_path).expect("Failed to open repository"),
        Ok(repo) => repo,
    };

    repo.checkout_head(Some(CheckoutBuilder::default().force()))
        .expect("Failed to checkout repository");

    let configure_args = build_qemu_configure_args(&qemu_install_path);

    let configure_prog = qemu_repo_path.join("configure");

    if !configure_prog.exists() {
        panic!(
            "Failed to find configure script at {}",
            configure_prog.display()
        );
    }

    Command::new(configure_prog)
        .current_dir(&qemu_build_path)
        .args(&configure_args)
        .arg(
            &repo
                .path()
                .parent()
                .expect("Could not find parent of repo path"),
        )
        .status()
        .expect("Failed to run configure");

    Command::new("make")
        .current_dir(&qemu_build_path)
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .status()
        .expect("Failed to run make");

    let enabled_targets = get_target_list();

    for enabled_target in enabled_targets {
        let target_bin = outdir_path.join(&enabled_target);
        if !target_bin.exists() {
            panic!("Failed to build target {}", enabled_target);
        }
    }
}
