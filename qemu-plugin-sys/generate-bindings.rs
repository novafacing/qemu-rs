#!/usr/bin/env -S cargo +nightly-gnu -Z script
---
[package]
edition = "2021"
[dependencies]
anyhow = "*"
bindgen = "*"
cargo_metadata = "*"
reqwest = { version = "*", features = ["blocking"] }
zip = "*"
[lints.rust]
non_snake_case = "allow"
---

use anyhow::{anyhow, Result};
use bindgen::{
    builder, AliasVariation, EnumVariation, FieldVisibilityKind, MacroTypeVariation,
    NonCopyUnionStyle,
};
use cargo_metadata::MetadataCommand;
use reqwest::blocking::get;
use std::{
    io::copy,
    fs::{create_dir_all, read_to_string, write, File, OpenOptions},
    path::{Path, PathBuf},
};
use zip::ZipArchive;

const QEMU_GITHUB_URL_BASE: &str = "https://github.com/qemu/qemu/";

const QEMU_VERSIONS: &[&str] = &[
    // Plugin V1 is up until 8.2.4
    "1332b8dd434674480f0feb2cdf3bbaebb85b4240",
    // Plugin V2 is from 9.0.0
    "c25df57ae8f9fe1c72eee2dab37d76d904ac382e",
    // Plugin V3 is from 9.1.0
    "7de77d37880d7267a491cb32a1b2232017d1e545",
    // Plugin V4 is from 9.2.0
    "595cd9ce2ec9330882c991a647d5bc2a5640f380",
];

fn qemu_git_url(hash: &str) -> String {
    format!("{}/archive/{}.zip", QEMU_GITHUB_URL_BASE, hash)
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

/// Extract a zip file at a path to a destination (stripping the root)
fn extract_zip(archive: &Path, destination: &Path) -> Result<()> {
    let archive = File::open(archive)?;
    let mut archive = ZipArchive::new(archive)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = file.mangled_name();
        
        let components: Vec<_> = file_path.components().collect();
        
        if components.len() <= 1 {
            continue;
        }
        
        let new_path = components[1..].iter().collect::<PathBuf>();
        let out_path = destination.join(new_path);
        
        if file.is_dir() {
            create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                create_dir_all(parent)?;
            }
            let mut out_file = File::create(&out_path)?;
            copy(&mut file, &mut out_file)?;
        }
    }
    Ok(())
}

fn generate_windows_delaylink_library(qemu_plugin_symbols: &Path, destination: &Path) -> Result<()> {
    println!("Generating Windows delaylink library from {:?} to {:?}", qemu_plugin_symbols, destination);
    let all_commands = read_to_string(qemu_plugin_symbols)?;
    let all_commands = all_commands.replace(|x| "{};".contains(x), "");
    write(destination, format!("EXPORTS\n{all_commands}"))?;

    Ok(())
}

fn generate_bindings(qemu_plugin_header: &Path, destination: &Path) -> Result<()> {
    let header_contents = read_to_string(qemu_plugin_header)?;
    let header_file_name = qemu_plugin_header.file_name().ok_or_else(|| anyhow!("Failed to get file name"))?.to_str().ok_or_else(|| anyhow!("Failed to convert file name to string"))?;
    let header_contents = header_contents.replace("#include <glib.h>", "");
    // Append `typedef GArray void;` and `typedef GByteArray void;` to the header. Otherwise, we
    // need to use pkg_config to find the glib-2.0 include paths and our bindings will be
    // massive.
    let header_contents = format!(
        "{}\n{}\n{}\n",
        "typedef struct GArray { char *data; unsigned int len; } GArray;",
        "typedef struct GByteArray { unsigned char *data; unsigned int len; } GByteArray;",
        header_contents,
    );

    let rust_bindings = builder()
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
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
        .layout_tests(false)
        .header_contents(header_file_name, &header_contents)
        // Blocklist because we will define these items
        .blocklist_function("qemu_plugin_install")
        .blocklist_item("qemu_plugin_version")
        // ALlowlist all other qemu_plugin.* items
        .allowlist_item("qemu_plugin.*")
        .allowlist_item("QEMU_PLUGIN.*")
        .generate()?;

    rust_bindings.write_to_file(destination)?;
    Ok(())
}

fn generate(tmp_dir: &Path, out_dir: &Path, version: usize) -> Result<()> {
    println!("Generating bindings with tmp={:?} out={:?} version={}", tmp_dir, out_dir, version);
    let src_archive = tmp_dir.join(format!("qemu-{}.zip", QEMU_VERSIONS[version - 1]));
    let src_dir = tmp_dir.join(format!("qemu-{}", QEMU_VERSIONS[version - 1]));

    if !src_archive.exists() {
        let qemu_url = qemu_git_url(QEMU_VERSIONS[version - 1]);
        println!("Downloading {} to {:?}", qemu_url, src_archive);
        download(&qemu_url, &src_archive)?;
    }

    if !src_dir.exists() {
        println!("Extracting {:?} to {:?}", src_archive, src_dir);
        extract_zip(&src_archive, &src_dir)?;
    }

    generate_windows_delaylink_library(
        &src_dir.join("plugins").join("qemu-plugins.symbols"),
        &out_dir.join(&format!("qemu_plugin_api_v{}.def", version)),
    )?;

    generate_bindings(
        &src_dir.join("include").join("qemu").join("qemu-plugin.h"),
        &out_dir.join(&format!("bindings_v{}.rs", version)),
    )?;

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

    let tmp_dir = metadata.target_directory.join("tmp").into_std_path_buf();

    if !tmp_dir.exists() {
        create_dir_all(&tmp_dir)?;
    }

    generate(&tmp_dir, &out_dir, 1)?;
    generate(&tmp_dir, &out_dir, 2)?;
    generate(&tmp_dir, &out_dir, 3)?;
    generate(&tmp_dir, &out_dir, 4)?;

    Ok(())
}
