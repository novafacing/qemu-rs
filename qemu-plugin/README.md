# QEMU-PLUGIN

High level idiomatic Rust bindings to the QEMU Plugin API, including tools to build
QEMU plugins in Rust.

## Example

Below is a minimal plugin example for a plugin which prints the execution trace of the
program running in QEMU. Notice that all we do is register a struct which implements
`Plugin` in a library constructor, and the library takes care of the rest.

```rust,ignore
use anyhow::Result;
use ctor::ctor;
use qemu_plugin::{
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    PluginId, TranslationBlock,
};
use std::sync::Mutex;

struct TinyTrace {}

impl Register for TinyTrace {}

impl HasCallbacks for TinyTrace {
    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        tb.instructions().enumerate().try_for_each(|(idx, insn)| {
            if idx == 0 {
                println!("====TB: {:08x}", insn.vaddr());
            }

            println!("{:08x}: {}", insn.vaddr(), insn.disas()?);
            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }
}

impl Plugin for TinyTrace {}

#[ctor]
fn init() {
    PLUGIN
        .set(Mutex::new(Box::new(TinyTrace {})))
        .map_err(|_| anyhow::anyhow!("Failed to set plugin"))
        .expect("Failed to set plugin");
}
```

The above `src/lib.rs` in a Cargo project with the following `Cargo.toml` will compile to
`libtiny.so`, which can be loaded in QEMU by running `qemu-system-ARCH -plugin ./libtiny.so`.

```toml
[package]
name = "tiny"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
qemu-plugin = "8.2.2-v0"
anyhow = "1.0.75"
ffi = "0.1.0"
ctor = "0.2.6"
```
