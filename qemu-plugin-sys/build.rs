#[cfg(windows)]
use anyhow::anyhow;
use anyhow::Result;
#[cfg(windows)]
use std::{env::var, path::PathBuf, process::Command, str::FromStr};

#[cfg(windows)]
fn out_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(
        var("OUT_DIR").map_err(|e| anyhow!("OUT_DIR not set: {e}"))?,
    ))
}

fn main() -> Result<()> {
    #[cfg(windows)]
    {
        let out_dir = out_dir()?;
        let def_file = PathBuf::from_str("src/qemu_plugin_api.def")?;
        let def_file_str = def_file.to_string_lossy();
        let lib_file = out_dir.join("qemu_plugin_api.lib");
        let lib_file_str = lib_file.to_string_lossy();
        let ch = Command::new("dlltool")
            .args([
                "--input-def",
                &def_file_str,
                "--output-delaylib",
                &lib_file_str,
                "--dllname",
                "qemu.exe",
            ])
            .spawn()?
            .wait()?;
        if !ch.success() {
            return Err(anyhow!("dlltool failed"));
        }
        println!("cargo:rustc-link-search={}", out_dir.display());
        println!("cargo:rustc-link-lib=qemu_plugin_api");
    }
    Ok(())
}
