//! Build script for QEMU binaries. Configures QEMU by converting crate features to configure
//! arguments, then builds it into the crate OUT_DIR.

use anyhow::{anyhow, Error, Result};
use command_ext::CommandExtCheck;
use reqwest::blocking::get;
use std::{
    env::var,
    fs::{create_dir_all, File, OpenOptions},
    path::{Path, PathBuf},
    process::Command,
};
use tar::Archive;
use xz2::read::XzDecoder;

const QEMU_SRC_URL_BASE: &str = "https://download.qemu.org/";
const QEMU_VERSION: &str = "9.0.0";

pub struct ConfigureArgs(Vec<String>);

impl ConfigureArgs {
    fn targets_from_features() -> String {
        let mut targets = Vec::new();

        if cfg!(feature = "aarch64-softmmu") {
            targets.push("aarch64-softmmu".to_string());
        }
        if cfg!(feature = "alpha-softmmu") {
            targets.push("alpha-softmmu".to_string());
        }
        if cfg!(feature = "arm-softmmu") {
            targets.push("arm-softmmu".to_string());
        }
        if cfg!(feature = "avr-softmmu") {
            targets.push("avr-softmmu".to_string());
        }
        if cfg!(feature = "cris-softmmu") {
            targets.push("cris-softmmu".to_string());
        }
        if cfg!(feature = "hppa-softmmu") {
            targets.push("hppa-softmmu".to_string());
        }
        if cfg!(feature = "i386-softmmu") {
            targets.push("i386-softmmu".to_string());
        }
        if cfg!(feature = "loongarch64-softmmu") {
            targets.push("loongarch64-softmmu".to_string());
        }
        if cfg!(feature = "m68k-softmmu") {
            targets.push("m68k-softmmu".to_string());
        }
        if cfg!(feature = "microblazeel-softmmu") {
            targets.push("microblazeel-softmmu".to_string());
        }
        if cfg!(feature = "microblaze-softmmu") {
            targets.push("microblaze-softmmu".to_string());
        }
        if cfg!(feature = "mips64el-softmmu") {
            targets.push("mips64el-softmmu".to_string());
        }
        if cfg!(feature = "mips64-softmmu") {
            targets.push("mips64-softmmu".to_string());
        }
        if cfg!(feature = "mipsel-softmmu") {
            targets.push("mipsel-softmmu".to_string());
        }
        if cfg!(feature = "mips-softmmu") {
            targets.push("mips-softmmu".to_string());
        }
        if cfg!(feature = "nios2-softmmu") {
            targets.push("nios2-softmmu".to_string());
        }
        if cfg!(feature = "or1k-softmmu") {
            targets.push("or1k-softmmu".to_string());
        }
        if cfg!(feature = "ppc64-softmmu") {
            targets.push("ppc64-softmmu".to_string());
        }
        if cfg!(feature = "ppc-softmmu") {
            targets.push("ppc-softmmu".to_string());
        }
        if cfg!(feature = "riscv32-softmmu") {
            targets.push("riscv32-softmmu".to_string());
        }
        if cfg!(feature = "riscv64-softmmu") {
            targets.push("riscv64-softmmu".to_string());
        }
        if cfg!(feature = "rx-softmmu") {
            targets.push("rx-softmmu".to_string());
        }
        if cfg!(feature = "s390x-softmmu") {
            targets.push("s390x-softmmu".to_string());
        }
        if cfg!(feature = "sh4eb-softmmu") {
            targets.push("sh4eb-softmmu".to_string());
        }
        if cfg!(feature = "sh4-softmmu") {
            targets.push("sh4-softmmu".to_string());
        }
        if cfg!(feature = "sparc64-softmmu") {
            targets.push("sparc64-softmmu".to_string());
        }
        if cfg!(feature = "sparc-softmmu") {
            targets.push("sparc-softmmu".to_string());
        }
        if cfg!(feature = "tricore-softmmu") {
            targets.push("tricore-softmmu".to_string());
        }
        if cfg!(feature = "x86_64-softmmu") {
            targets.push("x86_64-softmmu".to_string());
        }
        if cfg!(feature = "xtensaeb-softmmu") {
            targets.push("xtensaeb-softmmu".to_string());
        }
        if cfg!(feature = "xtensa-softmmu") {
            targets.push("xtensa-softmmu".to_string());
        }
        if cfg!(feature = "aarch64_be-linux-user") {
            targets.push("aarch64_be-linux-user".to_string());
        }
        if cfg!(feature = "aarch64-linux-user") {
            targets.push("aarch64-linux-user".to_string());
        }
        if cfg!(feature = "alpha-linux-user") {
            targets.push("alpha-linux-user".to_string());
        }
        if cfg!(feature = "armeb-linux-user") {
            targets.push("armeb-linux-user".to_string());
        }
        if cfg!(feature = "arm-linux-user") {
            targets.push("arm-linux-user".to_string());
        }
        if cfg!(feature = "cris-linux-user") {
            targets.push("cris-linux-user".to_string());
        }
        if cfg!(feature = "hexagon-linux-user") {
            targets.push("hexagon-linux-user".to_string());
        }
        if cfg!(feature = "hppa-linux-user") {
            targets.push("hppa-linux-user".to_string());
        }
        if cfg!(feature = "i386-linux-user") {
            targets.push("i386-linux-user".to_string());
        }
        if cfg!(feature = "loongarch64-linux-user") {
            targets.push("loongarch64-linux-user".to_string());
        }
        if cfg!(feature = "m68k-linux-user") {
            targets.push("m68k-linux-user".to_string());
        }
        if cfg!(feature = "microblazeel-linux-user") {
            targets.push("microblazeel-linux-user".to_string());
        }
        if cfg!(feature = "microblaze-linux-user") {
            targets.push("microblaze-linux-user".to_string());
        }
        if cfg!(feature = "mips64el-linux-user") {
            targets.push("mips64el-linux-user".to_string());
        }
        if cfg!(feature = "mips64-linux-user") {
            targets.push("mips64-linux-user".to_string());
        }
        if cfg!(feature = "mipsel-linux-user") {
            targets.push("mipsel-linux-user".to_string());
        }
        if cfg!(feature = "mips-linux-user") {
            targets.push("mips-linux-user".to_string());
        }
        if cfg!(feature = "mipsn32el-linux-user") {
            targets.push("mipsn32el-linux-user".to_string());
        }
        if cfg!(feature = "mipsn32-linux-user") {
            targets.push("mipsn32-linux-user".to_string());
        }
        if cfg!(feature = "nios2-linux-user") {
            targets.push("nios2-linux-user".to_string());
        }
        if cfg!(feature = "or1k-linux-user") {
            targets.push("or1k-linux-user".to_string());
        }
        if cfg!(feature = "ppc64le-linux-user") {
            targets.push("ppc64le-linux-user".to_string());
        }
        if cfg!(feature = "ppc64-linux-user") {
            targets.push("ppc64-linux-user".to_string());
        }
        if cfg!(feature = "ppc-linux-user") {
            targets.push("ppc-linux-user".to_string());
        }
        if cfg!(feature = "riscv32-linux-user") {
            targets.push("riscv32-linux-user".to_string());
        }
        if cfg!(feature = "riscv64-linux-user") {
            targets.push("riscv64-linux-user".to_string());
        }
        if cfg!(feature = "s390x-linux-user") {
            targets.push("s390x-linux-user".to_string());
        }
        if cfg!(feature = "sh4eb-linux-user") {
            targets.push("sh4eb-linux-user".to_string());
        }
        if cfg!(feature = "sh4-linux-user") {
            targets.push("sh4-linux-user".to_string());
        }
        if cfg!(feature = "sparc32plus-linux-user") {
            targets.push("sparc32plus-linux-user".to_string());
        }
        if cfg!(feature = "sparc64-linux-user") {
            targets.push("sparc64-linux-user".to_string());
        }
        if cfg!(feature = "sparc-linux-user") {
            targets.push("sparc-linux-user".to_string());
        }
        if cfg!(feature = "x86_64-linux-user") {
            targets.push("x86_64-linux-user".to_string());
        }
        if cfg!(feature = "xtensaeb-linux-user") {
            targets.push("xtensaeb-linux-user".to_string());
        }
        if cfg!(feature = "xtensa-linux-user") {
            targets.push("xtensa-linux-user".to_string());
        }

        format!("--target-list={}", targets.join(","))
    }

    fn audio_drv_from_features() -> String {
        let mut audio_drvs = Vec::new();

        if cfg!(feature = "audio-drv-alsa") {
            audio_drvs.push("alsa".to_string());
        }
        if cfg!(feature = "audio-drv-coreaudio") {
            audio_drvs.push("coreaudio".to_string());
        }
        if cfg!(feature = "audio-drv-dsound") {
            audio_drvs.push("dsound".to_string());
        }
        if cfg!(feature = "audio-drv-jack") {
            audio_drvs.push("jack".to_string());
        }
        if cfg!(feature = "audio-drv-oss") {
            audio_drvs.push("oss".to_string());
        }
        if cfg!(feature = "audio-drv-pa") {
            audio_drvs.push("pa".to_string());
        }
        if cfg!(feature = "audio-drv-pipewire") {
            audio_drvs.push("pipewire".to_string());
        }
        if cfg!(feature = "audio-drv-sdl") {
            audio_drvs.push("sdl".to_string());
        }
        if cfg!(feature = "audio-drv-sndio") {
            audio_drvs.push("sndio".to_string());
        }
        if cfg!(feature = "audio-drv-default") {
            audio_drvs.push("default".to_string());
        }

        format!("--audio-drv-list={}", audio_drvs.join(","))
    }

    fn options_from_features() -> Vec<String> {
        let mut options = Vec::new();

        if cfg!(feature = "static") {
            options.push("--static".to_string());
        }
        if cfg!(feature = "debug") {
            options.push("--enable-debug".to_string());
        }
        if cfg!(feature = "werror") {
            options.push("--enable-werror".to_string());
        }
        if cfg!(feature = "plugins") {
            options.push("--enable-plugins".to_string());
        }
        if !cfg!(feature = "coroutine-pool") {
            options.push("--disable-coroutine-pool".to_string());
        }
        if !cfg!(feature = "debug-info") {
            options.push("--disable-debug-info".to_string());
        }
        if !cfg!(feature = "hexagon-idef-parser") {
            options.push("--disable-hexagon-idef-parser".to_string());
        }
        if !cfg!(feature = "install-blobs") {
            options.push("--disable-install-blobs".to_string());
        }
        if !cfg!(feature = "qom-cast-debug") {
            options.push("--disable-qom-cast-debug".to_string());
        }
        if cfg!(feature = "cfi") {
            options.push("--enable-cfi".to_string());
        }
        if cfg!(feature = "cfi-debug") {
            options.push("--enable-cfi-debug".to_string());
        }
        if cfg!(feature = "debug-graph-lock") {
            options.push("--enable-debug-graph-lock".to_string());
        }
        if cfg!(feature = "debug-mutex") {
            options.push("--enable-debug-mutex".to_string());
        }
        if cfg!(feature = "debug-stack-usage") {
            options.push("--enable-debug-stack-usage".to_string());
        }
        if cfg!(feature = "fuzzing") {
            options.push("--enable-fuzzing".to_string());
        }
        if cfg!(feature = "gcov") {
            options.push("--enable-gcov".to_string());
        }
        if cfg!(feature = "gprof") {
            options.push("--enable-gprof".to_string());
        }
        if cfg!(feature = "lto") {
            options.push("--enable-lto".to_string());
        }
        if cfg!(feature = "module-upgrades") {
            options.push("--enable-module-upgrades".to_string());
        }
        if cfg!(feature = "rng-none") {
            options.push("--enable-rng-none".to_string());
        }
        if cfg!(feature = "safe-stack") {
            options.push("--enable-safe-stack".to_string());
        }
        if cfg!(feature = "sanitizers") {
            options.push("--enable-sanitizers".to_string());
        }
        if cfg!(feature = "strip") {
            options.push("--enable-strip".to_string());
        }
        if cfg!(feature = "tcg-interpreter") {
            options.push("--enable-tcg-interpreter".to_string());
        }
        if cfg!(feature = "tsan") {
            options.push("--enable-tsan".to_string());
        }

        options
    }

    fn trace_backends_from_features() -> String {
        let mut trace_backends = Vec::new();
        if cfg!(feature = "trace-backend-dtrace") {
            trace_backends.push("dtrace".to_string());
        }
        if cfg!(feature = "trace-backend-ftrace") {
            trace_backends.push("ftrace".to_string());
        }
        if cfg!(feature = "trace-backend-log") {
            trace_backends.push("log".to_string());
        }
        if cfg!(feature = "trace-backend-nop") {
            trace_backends.push("nop".to_string());
        }
        if cfg!(feature = "trace-backend-simple") {
            trace_backends.push("simple".to_string());
        }
        if cfg!(feature = "trace-backend-syslog") {
            trace_backends.push("syslog".to_string());
        }
        if cfg!(feature = "trace-backend-ust") {
            trace_backends.push("ust".to_string());
        }

        format!("--enable-trace-backends={}", trace_backends.join(","))
    }

    fn coroutine_backend_from_features() -> String {
        if cfg!(feature = "coroutine-backend-auto") {
            "auto"
        } else if cfg!(feature = "coroutine-backend-sigaltstack") {
            "sigaltstack"
        } else if cfg!(feature = "coroutine-backend-ucontext") {
            "ucontext"
        } else if cfg!(feature = "coroutine-backend-windows") {
            "windows"
        } else {
            panic!("No coroutine backend selected");
        }
        .to_string()
    }

    pub fn features_from_features() -> Vec<String> {
        let mut features = Vec::new();
        if !cfg!(feature = "without-default-features") {
            return features;
        }
        if cfg!(feature = "enable-feature-alsa") {
            features.push("--enable-alsa".to_string())
        }
        if cfg!(feature = "enable-feature-attr") {
            features.push("--enable-attr".to_string())
        }
        if cfg!(feature = "enable-feature-auth-pam") {
            features.push("--enable-auth-pam".to_string())
        }
        if cfg!(feature = "enable-feature-avx2") {
            features.push("--enable-avx2".to_string())
        }
        if cfg!(feature = "enable-feature-avx512bw") {
            features.push("--enable-avx512bw".to_string())
        }
        if cfg!(feature = "enable-feature-avx512f") {
            features.push("--enable-avx512f".to_string())
        }
        if cfg!(feature = "enable-feature-blkio") {
            features.push("--enable-blkio".to_string())
        }
        if cfg!(feature = "enable-feature-bochs") {
            features.push("--enable-bochs".to_string())
        }
        if cfg!(feature = "enable-feature-bpf") {
            features.push("--enable-bpf".to_string())
        }
        if cfg!(feature = "enable-feature-brlapi") {
            features.push("--enable-brlapi".to_string())
        }
        if cfg!(feature = "enable-feature-bzip2") {
            features.push("--enable-bzip2".to_string())
        }
        if cfg!(feature = "enable-feature-canokey") {
            features.push("--enable-canokey".to_string())
        }
        if cfg!(feature = "enable-feature-cap-ng") {
            features.push("--enable-cap-ng".to_string())
        }
        if cfg!(feature = "enable-feature-capstone") {
            features.push("--enable-capstone".to_string())
        }
        if cfg!(feature = "enable-feature-cloop") {
            features.push("--enable-cloop".to_string())
        }
        if cfg!(feature = "enable-feature-cocoa") {
            features.push("--enable-cocoa".to_string())
        }
        if cfg!(feature = "enable-feature-colo-proxy") {
            features.push("--enable-colo-proxy".to_string())
        }
        if cfg!(feature = "enable-feature-coreaudio") {
            features.push("--enable-coreaudio".to_string())
        }
        if cfg!(feature = "enable-feature-crypto-afalg") {
            features.push("--enable-crypto-afalg".to_string())
        }
        if cfg!(feature = "enable-feature-curl") {
            features.push("--enable-curl".to_string())
        }
        if cfg!(feature = "enable-feature-curses") {
            features.push("--enable-curses".to_string())
        }
        if cfg!(feature = "enable-feature-dbus-display") {
            features.push("--enable-dbus-display".to_string())
        }
        if cfg!(feature = "enable-feature-dmg") {
            features.push("--enable-dmg".to_string())
        }
        if cfg!(feature = "enable-feature-docs") {
            features.push("--enable-docs".to_string())
        }
        if cfg!(feature = "enable-feature-dsound") {
            features.push("--enable-dsound".to_string())
        }
        if cfg!(feature = "enable-feature-fuse") {
            features.push("--enable-fuse".to_string())
        }
        if cfg!(feature = "enable-feature-fuse-lseek") {
            features.push("--enable-fuse-lseek".to_string())
        }
        if cfg!(feature = "enable-feature-gcrypt") {
            features.push("--enable-gcrypt".to_string())
        }
        if cfg!(feature = "enable-feature-gettext") {
            features.push("--enable-gettext".to_string())
        }
        if cfg!(feature = "enable-feature-gio") {
            features.push("--enable-gio".to_string())
        }
        if cfg!(feature = "enable-feature-glusterfs") {
            features.push("--enable-glusterfs".to_string())
        }
        if cfg!(feature = "enable-feature-gnutls") {
            features.push("--enable-gnutls".to_string())
        }
        if cfg!(feature = "enable-feature-gtk") {
            features.push("--enable-gtk".to_string())
        }
        if cfg!(feature = "enable-feature-gtk-clipboard") {
            features.push("--enable-gtk-clipboard".to_string())
        }
        if cfg!(feature = "enable-feature-guest-agent") {
            features.push("--enable-guest-agent".to_string())
        }
        if cfg!(feature = "enable-feature-guest-agent-msi") {
            features.push("--enable-guest-agent-msi".to_string())
        }
        if cfg!(feature = "enable-feature-hax") {
            features.push("--enable-hax".to_string())
        }
        if cfg!(feature = "enable-feature-hvf") {
            features.push("--enable-hvf".to_string())
        }
        if cfg!(feature = "enable-feature-iconv") {
            features.push("--enable-iconv".to_string())
        }
        if cfg!(feature = "enable-feature-jack") {
            features.push("--enable-jack".to_string())
        }
        if cfg!(feature = "enable-feature-keyring") {
            features.push("--enable-keyring".to_string())
        }
        if cfg!(feature = "enable-feature-kvm") {
            features.push("--enable-kvm".to_string())
        }
        if cfg!(feature = "enable-feature-l2tpv3") {
            features.push("--enable-l2tpv3".to_string())
        }
        if cfg!(feature = "enable-feature-libdaxctl") {
            features.push("--enable-libdaxctl".to_string())
        }
        if cfg!(feature = "enable-feature-libdw") {
            features.push("--enable-libdw".to_string())
        }
        if cfg!(feature = "enable-feature-libiscsi") {
            features.push("--enable-libiscsi".to_string())
        }
        if cfg!(feature = "enable-feature-libkeyutils") {
            features.push("--enable-libkeyutils".to_string())
        }
        if cfg!(feature = "enable-feature-libnfs") {
            features.push("--enable-libnfs".to_string())
        }
        if cfg!(feature = "enable-feature-libpmem") {
            features.push("--enable-libpmem".to_string())
        }
        if cfg!(feature = "enable-feature-libssh") {
            features.push("--enable-libssh".to_string())
        }
        if cfg!(feature = "enable-feature-libudev") {
            features.push("--enable-libudev".to_string())
        }
        if cfg!(feature = "enable-feature-libusb") {
            features.push("--enable-libusb".to_string())
        }
        if cfg!(feature = "enable-feature-libvduse") {
            features.push("--enable-libvduse".to_string())
        }
        if cfg!(feature = "enable-feature-linux-aio") {
            features.push("--enable-linux-aio".to_string())
        }
        if cfg!(feature = "enable-feature-linux-io-uring") {
            features.push("--enable-linux-io-uring".to_string())
        }
        if cfg!(feature = "enable-feature-live-block-migration") {
            features.push("--enable-live-block-migration".to_string())
        }
        if cfg!(feature = "enable-feature-lzfse") {
            features.push("--enable-lzfse".to_string())
        }
        if cfg!(feature = "enable-feature-lzo") {
            features.push("--enable-lzo".to_string())
        }
        if cfg!(feature = "enable-feature-malloc-trim") {
            features.push("--enable-malloc-trim".to_string())
        }
        if cfg!(feature = "enable-feature-membarrier") {
            features.push("--enable-membarrier".to_string())
        }
        if cfg!(feature = "enable-feature-modules") {
            features.push("--enable-modules".to_string())
        }
        if cfg!(feature = "enable-feature-mpath") {
            features.push("--enable-mpath".to_string())
        }
        if cfg!(feature = "enable-feature-multiprocess") {
            features.push("--enable-multiprocess".to_string())
        }
        if cfg!(feature = "enable-feature-netmap") {
            features.push("--enable-netmap".to_string())
        }
        if cfg!(feature = "enable-feature-nettle") {
            features.push("--enable-nettle".to_string())
        }
        if cfg!(feature = "enable-feature-numa") {
            features.push("--enable-numa".to_string())
        }
        if cfg!(feature = "enable-feature-nvmm") {
            features.push("--enable-nvmm".to_string())
        }
        if cfg!(feature = "enable-feature-opengl") {
            features.push("--enable-opengl".to_string())
        }
        if cfg!(feature = "enable-feature-oss") {
            features.push("--enable-oss".to_string())
        }
        if cfg!(feature = "enable-feature-pa") {
            features.push("--enable-pa".to_string())
        }
        if cfg!(feature = "enable-feature-parallels") {
            features.push("--enable-parallels".to_string())
        }
        if cfg!(feature = "enable-feature-pipewire") {
            features.push("--enable-pipewire".to_string())
        }
        if cfg!(feature = "enable-feature-png") {
            features.push("--enable-png".to_string())
        }
        if cfg!(feature = "enable-feature-pvrdma") {
            features.push("--enable-pvrdma".to_string())
        }
        if cfg!(feature = "enable-feature-qcow1") {
            features.push("--enable-qcow1".to_string())
        }
        if cfg!(feature = "enable-feature-qed") {
            features.push("--enable-qed".to_string())
        }
        if cfg!(feature = "enable-feature-qga-vss") {
            features.push("--enable-qga-vss".to_string())
        }
        if cfg!(feature = "enable-feature-rbd") {
            features.push("--enable-rbd".to_string())
        }
        if cfg!(feature = "enable-feature-rdma") {
            features.push("--enable-rdma".to_string())
        }
        if cfg!(feature = "enable-feature-replication") {
            features.push("--enable-replication".to_string())
        }
        if cfg!(feature = "enable-feature-sdl") {
            features.push("--enable-sdl".to_string())
        }
        if cfg!(feature = "enable-feature-sdl-image") {
            features.push("--enable-sdl-image".to_string())
        }
        if cfg!(feature = "enable-feature-seccomp") {
            features.push("--enable-seccomp".to_string())
        }
        if cfg!(feature = "enable-feature-selinux") {
            features.push("--enable-selinux".to_string())
        }
        if cfg!(feature = "enable-feature-slirp") {
            features.push("--enable-slirp".to_string())
        }
        if cfg!(feature = "enable-feature-slirp-smbd") {
            features.push("--enable-slirp-smbd".to_string())
        }
        if cfg!(feature = "enable-feature-smartcard") {
            features.push("--enable-smartcard".to_string())
        }
        if cfg!(feature = "enable-feature-snappy") {
            features.push("--enable-snappy".to_string())
        }
        if cfg!(feature = "enable-feature-sndio") {
            features.push("--enable-sndio".to_string())
        }
        if cfg!(feature = "enable-feature-sparse") {
            features.push("--enable-sparse".to_string())
        }
        if cfg!(feature = "enable-feature-spice") {
            features.push("--enable-spice".to_string())
        }
        if cfg!(feature = "enable-feature-spice-protocol") {
            features.push("--enable-spice-protocol".to_string())
        }
        if cfg!(feature = "enable-feature-stack-protector") {
            features.push("--enable-stack-protector".to_string())
        }
        if cfg!(feature = "enable-feature-tcg") {
            features.push("--enable-tcg".to_string())
        }
        if cfg!(feature = "enable-feature-tools") {
            features.push("--enable-tools".to_string())
        }
        if cfg!(feature = "enable-feature-tpm") {
            features.push("--enable-tpm".to_string())
        }
        if cfg!(feature = "enable-feature-u2f") {
            features.push("--enable-u2f".to_string())
        }
        if cfg!(feature = "enable-feature-usb-redir") {
            features.push("--enable-usb-redir".to_string())
        }
        if cfg!(feature = "enable-feature-vde") {
            features.push("--enable-vde".to_string())
        }
        if cfg!(feature = "enable-feature-vdi") {
            features.push("--enable-vdi".to_string())
        }
        if cfg!(feature = "enable-feature-vduse-blk-export") {
            features.push("--enable-vduse-blk-export".to_string())
        }
        if cfg!(feature = "enable-feature-vfio-user-server") {
            features.push("--enable-vfio-user-server".to_string())
        }
        if cfg!(feature = "enable-feature-vhdx") {
            features.push("--enable-vhdx".to_string())
        }
        if cfg!(feature = "enable-feature-vhost-crypto") {
            features.push("--enable-vhost-crypto".to_string())
        }
        if cfg!(feature = "enable-feature-vhost-kernel") {
            features.push("--enable-vhost-kernel".to_string())
        }
        if cfg!(feature = "enable-feature-vhost-net") {
            features.push("--enable-vhost-net".to_string())
        }
        if cfg!(feature = "enable-feature-vhost-user") {
            features.push("--enable-vhost-user".to_string())
        }
        if cfg!(feature = "enable-feature-vhost-user-blk-server") {
            features.push("--enable-vhost-user-blk-server".to_string())
        }
        if cfg!(feature = "enable-feature-vhost-vdpa") {
            features.push("--enable-vhost-vdpa".to_string())
        }
        if cfg!(feature = "enable-feature-virglrenderer") {
            features.push("--enable-virglrenderer".to_string())
        }
        if cfg!(feature = "enable-feature-virtfs") {
            features.push("--enable-virtfs".to_string())
        }
        if cfg!(feature = "enable-feature-virtfs-proxy-helper") {
            features.push("--enable-virtfs-proxy-helper".to_string())
        }
        if cfg!(feature = "enable-feature-vmdk") {
            features.push("--enable-vmdk".to_string())
        }
        if cfg!(feature = "enable-feature-vmnet") {
            features.push("--enable-vmnet".to_string())
        }
        if cfg!(feature = "enable-feature-vnc") {
            features.push("--enable-vnc".to_string())
        }
        if cfg!(feature = "enable-feature-vnc-jpeg") {
            features.push("--enable-vnc-jpeg".to_string())
        }
        if cfg!(feature = "enable-feature-vnc-sasl") {
            features.push("--enable-vnc-sasl".to_string())
        }
        if cfg!(feature = "enable-feature-vpc") {
            features.push("--enable-vpc".to_string())
        }
        if cfg!(feature = "enable-feature-vte") {
            features.push("--enable-vte".to_string())
        }
        if cfg!(feature = "enable-feature-vvfat") {
            features.push("--enable-vvfat".to_string())
        }
        if cfg!(feature = "enable-feature-whpx") {
            features.push("--enable-whpx".to_string())
        }
        if cfg!(feature = "enable-feature-xen") {
            features.push("--enable-xen".to_string())
        }
        if cfg!(feature = "enable-feature-xen-pci-passthrough") {
            features.push("--enable-xen-pci-passthrough".to_string())
        }
        if cfg!(feature = "enable-feature-xkbcommon") {
            features.push("--enable-xkbcommon".to_string())
        }
        if cfg!(feature = "enable-feature-zstd") {
            features.push("--enable-zstd".to_string())
        }
        if cfg!(feature = "enable-feature-system") {
            features.push("--enable-system".to_string())
        }
        if cfg!(feature = "enable-feature-user") {
            features.push("--enable-user".to_string())
        }
        if cfg!(feature = "enable-feature-linux-user") {
            features.push("--enable-linux-user".to_string())
        }
        if cfg!(feature = "enable-feature-bsd-user") {
            features.push("--enable-bsd-user".to_string())
        }
        if cfg!(feature = "enable-feature-pie") {
            features.push("--enable-pie".to_string())
        }
        if cfg!(feature = "enable-feature-debug-tcg") {
            features.push("--enable-debug-tcg".to_string())
        }

        features
    }

    pub fn from_features() -> Self {
        let mut features = Vec::new();

        features.push(Self::targets_from_features());
        features.push(Self::audio_drv_from_features());
        features.extend(Self::options_from_features());
        features.push(Self::trace_backends_from_features());
        features.push(Self::coroutine_backend_from_features());
        features.extend(Self::features_from_features());

        Self(features)
    }

    pub fn with_prefix(&mut self, prefix: &Path) -> &mut Self {
        self.0.push(format!("--prefix={}", prefix.display()));
        self
    }

    pub fn as_args(&self) -> Vec<String> {
        self.0.clone()
    }
}

fn qemu_src_url() -> String {
    format!("{}qemu-{}.tar.xz", QEMU_SRC_URL_BASE, QEMU_VERSION)
}

fn out_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(
        var("OUT_DIR").map_err(|e| anyhow!("OUT_DIR not set: {e}"))?,
    ))
}

/// Download a URL to a destination, using a blocking request
fn download(url: &str, destination: &Path) -> Result<()> {
    let mut response = get(url)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(destination)?;
    response.copy_to(&mut file)?;
    Ok(())
}

/// Extract a tar.xz archive at a path to a destination
fn extract_txz(archive: &Path, destination: &Path) -> Result<()> {
    let mut archive = File::open(archive)?;
    let mut archive = XzDecoder::new(&mut archive);
    let mut archive = Archive::new(&mut archive);
    // Unpack archive, removing 1 leading path component
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .try_for_each(|mut e| {
            let Ok(path) = e.path() else {
                return Err(anyhow!("Failed to get path from archive entry"));
            };
            let Some(prefix) = path.components().next() else {
                return Err(anyhow!("Failed to get prefix from archive entry {path:?}"));
            };
            let Ok(suffix) = path.strip_prefix(prefix) else {
                return Err(anyhow!(
                    "Failed to strip prefix {prefix:?} from archive entry {path:?}"
                ));
            };
            e.unpack(destination.join(suffix))
                .map(|_| ())
                .map_err(|e| anyhow!(e))
        })?;
    Ok(())
}

fn configure(build: &Path, src: &Path, prefix: &Path) -> Result<()> {
    Command::new(src.join("configure"))
        .current_dir(build)
        .args(ConfigureArgs::from_features().with_prefix(prefix).as_args())
        .check()
        .map(|_| ())
        .map_err(Error::from)
}

fn make(build: &Path) -> Result<()> {
    #[cfg(unix)]
    Command::new("make")
        .arg(format!("-j{}", num_cpus::get()))
        .current_dir(build)
        .check()
        .map(|_| ())
        .map_err(Error::from)
}

fn install(build: &Path) -> Result<()> {
    #[cfg(unix)]
    Command::new("make")
        .arg("install")
        .current_dir(build)
        .check()
        .map(|_| ())
        .map_err(Error::from)
}

fn main() -> Result<()> {
    if var("DOCS_RS").is_ok() {
        println!("cargo:rustc-cfg=docs_rs");
        return Ok(());
    }

    let out_dir = out_dir()?;
    let src_archive = out_dir.join(format!("qemu-{}.tar.xz", QEMU_VERSION));
    let src_dir = out_dir.join(format!("qemu-{}", QEMU_VERSION));
    let build_dir = out_dir.join("qemu-build");
    let install_dir = out_dir.join("qemu");

    if !src_archive.exists() {
        download(&qemu_src_url(), &src_archive)?;
    }

    if !src_dir.exists() {
        extract_txz(&src_archive, &src_dir)?;
    }

    if !build_dir.exists() {
        create_dir_all(&build_dir)?;
        configure(&build_dir, &src_dir, &install_dir)?;
        make(&build_dir)?;
    }

    if !install_dir.exists() {
        create_dir_all(&install_dir)?;
        install(&build_dir)?;
    }

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
