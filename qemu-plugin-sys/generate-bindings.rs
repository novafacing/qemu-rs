#!/usr/bin/env -S cargo +nightly-gnu -Z script
```
[package]
edition = "2021"
[dependencies]
anyhow = "*"
bindgen = "*"
cargo_metadata = "*"
reqwest = { version = "*", features = ["blocking"] }
tar = "*"
xz2 = "*"
[lints.rust]
non_snake_case = "allow"
```

use anyhow::{anyhow, Result};
use bindgen::{
    builder, AliasVariation, EnumVariation, FieldVisibilityKind, MacroTypeVariation,
    NonCopyUnionStyle,
};
use cargo_metadata::MetadataCommand;
use reqwest::blocking::get;
#[cfg(windows)]
use std::fs::{read_to_string, write};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    path::Path,
};
use tar::Archive;
use xz2::read::XzDecoder;

const QEMU_SRC_URL_BASE: &str = "https://download.qemu.org/";
const QEMU_VERSION: &str = "8.1.3";

fn qemu_src_url() -> String {
    format!("{}qemu-{}.tar.xz", QEMU_SRC_URL_BASE, QEMU_VERSION)
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

#[cfg(windows)]
fn generate_windows_delaylink_library(qemu_plugin_symbols: &Path, out_dir: &Path) -> Result<()> {
    let def_file = out_dir.join("qemu_plugin_api.def");
    let all_commands = read_to_string(qemu_plugin_symbols)?;
    let all_commands = all_commands.replace(|x| "{};".contains(x), "");
    write(&def_file, format!("EXPORTS\n{all_commands}"))?;

    Ok(())
}

fn generate_bindings(qemu_plugin_header: &Path, destination: &Path) -> Result<()> {
    let rust_bindings = builder()
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        // We don't care at all what warnings QEMU generates
        .clang_arg("-Wno-everything")
        .default_visibility(FieldVisibilityKind::Public)
        .default_alias_style(AliasVariation::TypeAlias)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .default_macro_constant_type(MacroTypeVariation::Unsigned)
        .default_non_copy_union_style(NonCopyUnionStyle::BindgenWrapper)
        .derive_default(true)
        .derive_hash(true)
        .derive_partialord(true)
        .derive_ord(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .generate_comments(true)
        .header(qemu_plugin_header.to_str().unwrap())
        // Blocklist because we will define these items
        .blocklist_function("qemu_plugin_install")
        .blocklist_item("qemu_plugin_version")
        // Blocklist because these types are not necessary
        .blocklist_item("_INTTYPES_H")
        .blocklist_item("_FEATURES_H")
        .blocklist_item("_DEFAULT_SOURCE")
        .blocklist_item("__GLIBC_USE_ISOC2X")
        .blocklist_item("__USE_ISOC11")
        .blocklist_item("__USE_ISOC99")
        .blocklist_item("__USE_ISOC95")
        .blocklist_item("__USE_POSIX_IMPLICITLY")
        .blocklist_item("_POSIX_SOURCE")
        .blocklist_item("_POSIX_C_SOURCE")
        .blocklist_item("__USE_POSIX")
        .blocklist_item("__USE_POSIX2")
        .blocklist_item("__USE_POSIX199309")
        .blocklist_item("__USE_POSIX199506")
        .blocklist_item("__USE_XOPEN2K")
        .blocklist_item("__USE_XOPEN2K8")
        .blocklist_item("_ATFILE_SOURCE")
        .blocklist_item("__WORDSIZE")
        .blocklist_item("__WORDSIZE_TIME64_COMPAT32")
        .blocklist_item("__TIMESIZE")
        .blocklist_item("__USE_MISC")
        .blocklist_item("__USE_ATFILE")
        .blocklist_item("__USE_FORTIFY_LEVEL")
        .blocklist_item("__GLIBC_USE_DEPRECATED_GETS")
        .blocklist_item("__GLIBC_USE_DEPRECATED_SCANF")
        .blocklist_item("__GLIBC_USE_C2X_STRTOL")
        .blocklist_item("_STDC_PREDEF_H")
        .blocklist_item("__STDC_IEC_559__")
        .blocklist_item("__STDC_IEC_60559_BFP__")
        .blocklist_item("__STDC_IEC_559_COMPLEX__")
        .blocklist_item("__STDC_IEC_60559_COMPLEX__")
        .blocklist_item("__STDC_ISO_10646__")
        .blocklist_item("__GNU_LIBRARY__")
        .blocklist_item("__GLIBC__")
        .blocklist_item("__GLIBC_MINOR__")
        .blocklist_item("_SYS_CDEFS_H")
        .blocklist_item("__glibc_c99_flexarr_available")
        .blocklist_item("__LDOUBLE_REDIRECTS_TO_FLOAT128_ABI")
        .blocklist_item("__HAVE_GENERIC_SELECTION")
        .blocklist_item("_STDINT_H")
        .blocklist_item("__GLIBC_USE_LIB_EXT2")
        .blocklist_item("__GLIBC_USE_IEC_60559_BFP_EXT")
        .blocklist_item("__GLIBC_USE_IEC_60559_BFP_EXT_C2X")
        .blocklist_item("__GLIBC_USE_IEC_60559_EXT")
        .blocklist_item("__GLIBC_USE_IEC_60559_FUNCS_EXT")
        .blocklist_item("__GLIBC_USE_IEC_60559_FUNCS_EXT_C2X")
        .blocklist_item("__GLIBC_USE_IEC_60559_TYPES_EXT")
        .blocklist_item("_BITS_TYPES_H")
        .blocklist_item("_BITS_TYPESIZES_H")
        .blocklist_item("__OFF_T_MATCHES_OFF64_T")
        .blocklist_item("__INO_T_MATCHES_INO64_T")
        .blocklist_item("__RLIM_T_MATCHES_RLIM64_T")
        .blocklist_item("__STATFS_MATCHES_STATFS64")
        .blocklist_item("__FD_SETSIZE")
        .blocklist_item("_BITS_TIME64_H")
        .blocklist_item("_BITS_WCHAR_H")
        .blocklist_item("_BITS_STDINT_INTN_H")
        .blocklist_item("_BITS_STDINT_UINTN_H")
        .blocklist_item("INT8_MIN")
        .blocklist_item("INT16_MIN")
        .blocklist_item("INT32_MIN")
        .blocklist_item("INT8_MAX")
        .blocklist_item("INT16_MAX")
        .blocklist_item("INT32_MAX")
        .blocklist_item("UINT8_MAX")
        .blocklist_item("UINT16_MAX")
        .blocklist_item("UINT32_MAX")
        .blocklist_item("INT_LEAST8_MIN")
        .blocklist_item("INT_LEAST16_MIN")
        .blocklist_item("INT_LEAST32_MIN")
        .blocklist_item("INT_LEAST8_MAX")
        .blocklist_item("INT_LEAST16_MAX")
        .blocklist_item("INT_LEAST32_MAX")
        .blocklist_item("UINT_LEAST8_MAX")
        .blocklist_item("UINT_LEAST16_MAX")
        .blocklist_item("UINT_LEAST32_MAX")
        .blocklist_item("INT_FAST8_MIN")
        .blocklist_item("INT_FAST16_MIN")
        .blocklist_item("INT_FAST32_MIN")
        .blocklist_item("INT_FAST8_MAX")
        .blocklist_item("INT_FAST16_MAX")
        .blocklist_item("INT_FAST32_MAX")
        .blocklist_item("UINT_FAST8_MAX")
        .blocklist_item("UINT_FAST16_MAX")
        .blocklist_item("UINT_FAST32_MAX")
        .blocklist_item("INTPTR_MIN")
        .blocklist_item("INTPTR_MAX")
        .blocklist_item("UINTPTR_MAX")
        .blocklist_item("PTRDIFF_MIN")
        .blocklist_item("PTRDIFF_MAX")
        .blocklist_item("SIG_ATOMIC_MIN")
        .blocklist_item("SIG_ATOMIC_MAX")
        .blocklist_item("SIZE_MAX")
        .blocklist_item("WINT_MIN")
        .blocklist_item("WINT_MAX")
        .blocklist_item("____gwchar_t_defined")
        .blocklist_item("__PRI64_PREFIX")
        .blocklist_item("__PRIPTR_PREFIX")
        .blocklist_item("PRId8")
        .blocklist_item("PRId16")
        .blocklist_item("PRId32")
        .blocklist_item("PRId64")
        .blocklist_item("PRIdLEAST8")
        .blocklist_item("PRIdLEAST16")
        .blocklist_item("PRIdLEAST32")
        .blocklist_item("PRIdLEAST64")
        .blocklist_item("PRIdFAST8")
        .blocklist_item("PRIdFAST16")
        .blocklist_item("PRIdFAST32")
        .blocklist_item("PRIdFAST64")
        .blocklist_item("PRIi8")
        .blocklist_item("PRIi16")
        .blocklist_item("PRIi32")
        .blocklist_item("PRIi64")
        .blocklist_item("PRIiLEAST8")
        .blocklist_item("PRIiLEAST16")
        .blocklist_item("PRIiLEAST32")
        .blocklist_item("PRIiLEAST64")
        .blocklist_item("PRIiFAST8")
        .blocklist_item("PRIiFAST16")
        .blocklist_item("PRIiFAST32")
        .blocklist_item("PRIiFAST64")
        .blocklist_item("PRIo8")
        .blocklist_item("PRIo16")
        .blocklist_item("PRIo32")
        .blocklist_item("PRIo64")
        .blocklist_item("PRIoLEAST8")
        .blocklist_item("PRIoLEAST16")
        .blocklist_item("PRIoLEAST32")
        .blocklist_item("PRIoLEAST64")
        .blocklist_item("PRIoFAST8")
        .blocklist_item("PRIoFAST16")
        .blocklist_item("PRIoFAST32")
        .blocklist_item("PRIoFAST64")
        .blocklist_item("PRIu8")
        .blocklist_item("PRIu16")
        .blocklist_item("PRIu32")
        .blocklist_item("PRIu64")
        .blocklist_item("PRIuLEAST8")
        .blocklist_item("PRIuLEAST16")
        .blocklist_item("PRIuLEAST32")
        .blocklist_item("PRIuLEAST64")
        .blocklist_item("PRIuFAST8")
        .blocklist_item("PRIuFAST16")
        .blocklist_item("PRIuFAST32")
        .blocklist_item("PRIuFAST64")
        .blocklist_item("PRIx8")
        .blocklist_item("PRIx16")
        .blocklist_item("PRIx32")
        .blocklist_item("PRIx64")
        .blocklist_item("PRIxLEAST8")
        .blocklist_item("PRIxLEAST16")
        .blocklist_item("PRIxLEAST32")
        .blocklist_item("PRIxLEAST64")
        .blocklist_item("PRIxFAST8")
        .blocklist_item("PRIxFAST16")
        .blocklist_item("PRIxFAST32")
        .blocklist_item("PRIxFAST64")
        .blocklist_item("PRIX8")
        .blocklist_item("PRIX16")
        .blocklist_item("PRIX32")
        .blocklist_item("PRIX64")
        .blocklist_item("PRIXLEAST8")
        .blocklist_item("PRIXLEAST16")
        .blocklist_item("PRIXLEAST32")
        .blocklist_item("PRIXLEAST64")
        .blocklist_item("PRIXFAST8")
        .blocklist_item("PRIXFAST16")
        .blocklist_item("PRIXFAST32")
        .blocklist_item("PRIXFAST64")
        .blocklist_item("PRIdMAX")
        .blocklist_item("PRIiMAX")
        .blocklist_item("PRIoMAX")
        .blocklist_item("PRIuMAX")
        .blocklist_item("PRIxMAX")
        .blocklist_item("PRIXMAX")
        .blocklist_item("PRIdPTR")
        .blocklist_item("PRIiPTR")
        .blocklist_item("PRIoPTR")
        .blocklist_item("PRIuPTR")
        .blocklist_item("PRIxPTR")
        .blocklist_item("PRIXPTR")
        .blocklist_item("SCNd8")
        .blocklist_item("SCNd16")
        .blocklist_item("SCNd32")
        .blocklist_item("SCNd64")
        .blocklist_item("SCNdLEAST8")
        .blocklist_item("SCNdLEAST16")
        .blocklist_item("SCNdLEAST32")
        .blocklist_item("SCNdLEAST64")
        .blocklist_item("SCNdFAST8")
        .blocklist_item("SCNdFAST16")
        .blocklist_item("SCNdFAST32")
        .blocklist_item("SCNdFAST64")
        .blocklist_item("SCNi8")
        .blocklist_item("SCNi16")
        .blocklist_item("SCNi32")
        .blocklist_item("SCNi64")
        .blocklist_item("SCNiLEAST8")
        .blocklist_item("SCNiLEAST16")
        .blocklist_item("SCNiLEAST32")
        .blocklist_item("SCNiLEAST64")
        .blocklist_item("SCNiFAST8")
        .blocklist_item("SCNiFAST16")
        .blocklist_item("SCNiFAST32")
        .blocklist_item("SCNiFAST64")
        .blocklist_item("SCNu8")
        .blocklist_item("SCNu16")
        .blocklist_item("SCNu32")
        .blocklist_item("SCNu64")
        .blocklist_item("SCNuLEAST8")
        .blocklist_item("SCNuLEAST16")
        .blocklist_item("SCNuLEAST32")
        .blocklist_item("SCNuLEAST64")
        .blocklist_item("SCNuFAST8")
        .blocklist_item("SCNuFAST16")
        .blocklist_item("SCNuFAST32")
        .blocklist_item("SCNuFAST64")
        .blocklist_item("SCNo8")
        .blocklist_item("SCNo16")
        .blocklist_item("SCNo32")
        .blocklist_item("SCNo64")
        .blocklist_item("SCNoLEAST8")
        .blocklist_item("SCNoLEAST16")
        .blocklist_item("SCNoLEAST32")
        .blocklist_item("SCNoLEAST64")
        .blocklist_item("SCNoFAST8")
        .blocklist_item("SCNoFAST16")
        .blocklist_item("SCNoFAST32")
        .blocklist_item("SCNoFAST64")
        .blocklist_item("SCNx8")
        .blocklist_item("SCNx16")
        .blocklist_item("SCNx32")
        .blocklist_item("SCNx64")
        .blocklist_item("SCNxLEAST8")
        .blocklist_item("SCNxLEAST16")
        .blocklist_item("SCNxLEAST32")
        .blocklist_item("SCNxLEAST64")
        .blocklist_item("SCNxFAST8")
        .blocklist_item("SCNxFAST16")
        .blocklist_item("SCNxFAST32")
        .blocklist_item("SCNxFAST64")
        .blocklist_item("SCNdMAX")
        .blocklist_item("SCNiMAX")
        .blocklist_item("SCNoMAX")
        .blocklist_item("SCNuMAX")
        .blocklist_item("SCNxMAX")
        .blocklist_item("SCNdPTR")
        .blocklist_item("SCNiPTR")
        .blocklist_item("SCNoPTR")
        .blocklist_item("SCNuPTR")
        .blocklist_item("SCNxPTR")
        .blocklist_item("__bool_true_false_are_defined")
        .blocklist_item("true_")
        .blocklist_item("false_")
        .blocklist_item("__u_char")
        .blocklist_item("__u_short")
        .blocklist_item("__u_int")
        .blocklist_item("__u_long")
        .blocklist_item("__int8_t")
        .blocklist_item("__uint8_t")
        .blocklist_item("__int16_t")
        .blocklist_item("__uint16_t")
        .blocklist_item("__int32_t")
        .blocklist_item("__uint32_t")
        .blocklist_item("__int64_t")
        .blocklist_item("__uint64_t")
        .blocklist_item("__int_least8_t")
        .blocklist_item("__uint_least8_t")
        .blocklist_item("__int_least16_t")
        .blocklist_item("__uint_least16_t")
        .blocklist_item("__int_least32_t")
        .blocklist_item("__uint_least32_t")
        .blocklist_item("__int_least64_t")
        .blocklist_item("__uint_least64_t")
        .blocklist_item("__quad_t")
        .blocklist_item("__u_quad_t")
        .blocklist_item("__intmax_t")
        .blocklist_item("__uintmax_t")
        .blocklist_item("__dev_t")
        .blocklist_item("__uid_t")
        .blocklist_item("__gid_t")
        .blocklist_item("__ino_t")
        .blocklist_item("__ino64_t")
        .blocklist_item("__mode_t")
        .blocklist_item("__nlink_t")
        .blocklist_item("__off_t")
        .blocklist_item("__off64_t")
        .blocklist_item("__pid_t")
        .blocklist_item("__fsid_t")
        .blocklist_item("__clock_t")
        .blocklist_item("__rlim_t")
        .blocklist_item("__rlim64_t")
        .blocklist_item("__id_t")
        .blocklist_item("__time_t")
        .blocklist_item("__useconds_t")
        .blocklist_item("__suseconds_t")
        .blocklist_item("__suseconds64_t")
        .blocklist_item("__daddr_t")
        .blocklist_item("__key_t")
        .blocklist_item("__clockid_t")
        .blocklist_item("__timer_t")
        .blocklist_item("__blksize_t")
        .blocklist_item("__blkcnt_t")
        .blocklist_item("__blkcnt64_t")
        .blocklist_item("__fsblkcnt_t")
        .blocklist_item("__fsblkcnt64_t")
        .blocklist_item("__fsfilcnt_t")
        .blocklist_item("__fsfilcnt64_t")
        .blocklist_item("__fsword_t")
        .blocklist_item("__ssize_t")
        .blocklist_item("__syscall_slong_t")
        .blocklist_item("__syscall_ulong_t")
        .blocklist_item("__loff_t")
        .blocklist_item("__caddr_t")
        .blocklist_item("__intptr_t")
        .blocklist_item("__socklen_t")
        .blocklist_item("__sig_atomic_t")
        .blocklist_item("int_least8_t")
        .blocklist_item("int_least16_t")
        .blocklist_item("int_least32_t")
        .blocklist_item("int_least64_t")
        .blocklist_item("uint_least8_t")
        .blocklist_item("uint_least16_t")
        .blocklist_item("uint_least32_t")
        .blocklist_item("uint_least64_t")
        .blocklist_item("int_fast8_t")
        .blocklist_item("int_fast16_t")
        .blocklist_item("int_fast32_t")
        .blocklist_item("int_fast64_t")
        .blocklist_item("uint_fast8_t")
        .blocklist_item("uint_fast16_t")
        .blocklist_item("uint_fast32_t")
        .blocklist_item("uint_fast64_t")
        .blocklist_item("intmax_t")
        .blocklist_item("uintmax_t")
        .blocklist_item("__gwchar_t")
        .blocklist_item("imaxdiv_t")
        .blocklist_item("imaxabs")
        .blocklist_item("imaxdiv")
        .blocklist_item("strtoimax")
        .blocklist_item("strtoumax")
        .blocklist_item("wcstoimax")
        .blocklist_item("wcstoumax")
        .blocklist_item("max_align_t")
        .blocklist_item("wchar_t")
        .generate()?;

    rust_bindings.write_to_file(destination)?;
    Ok(())
}

fn main() -> Result<()> {
    let metadata = MetadataCommand::new().no_deps().exec()?;

    let package = metadata
        .packages
        .iter()
        .find(|p| p.name == "qemu-plugin-sys")
        .ok_or_else(|| anyhow!("Failed to find package"))?;

    let out_dir = package
        .manifest_path
        .parent()
        .ok_or_else(|| anyhow!("Failed to get manifest path"))?
        .join("src")
        .into_std_path_buf();

    println!("out_dir: {:?}", out_dir);

    let tmp_dir = metadata.target_directory.join("tmp").into_std_path_buf();

    if !tmp_dir.exists() {
        create_dir_all(&tmp_dir)?;
    }

    let src_archive = tmp_dir.join(format!("qemu-{}.tar.xz", QEMU_VERSION));
    let src_dir = tmp_dir.join(format!("qemu-{}", QEMU_VERSION));

    if !src_archive.exists() {
        download(&qemu_src_url(), &src_archive)?;
    }

    if !src_dir.exists() {
        extract_txz(&src_archive, &src_dir)?;
    }

    #[cfg(windows)]
    generate_windows_delaylink_library(
        &src_dir.join("plugins").join("qemu-plugins.symbols"),
        &out_dir,
    )?;

    generate_bindings(
        &src_dir.join("include").join("qemu").join("qemu-plugin.h"),
        &out_dir.join("bindings.rs"),
    )?;

    Ok(())
}
