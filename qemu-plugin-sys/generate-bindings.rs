#!/usr/bin/env -S cargo +nightly-gnu -Z script

//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! anyhow = "*"
//! bindgen = "*"
//! cargo_metadata = "*"
//! reqwest = { version = "*", features = ["blocking"] }
//! tar = "*"
//! xz2 = "*"
//![lints.rust]
//!non_snake_case = "allow"
//! ```

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
        .blocklist_function("qemu_plugin_install")
        .blocklist_item("qemu_plugin_version")
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
