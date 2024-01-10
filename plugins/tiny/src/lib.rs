use anyhow::{anyhow, Result};
use ctor::ctor;
use qemu_plugin::{
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    PluginId, TranslationBlock,
};
use std::sync::Mutex;

struct TinyTrace {}

impl Plugin for TinyTrace {}
impl Register for TinyTrace {}

impl HasCallbacks for TinyTrace {
    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        tb.instructions().try_for_each(|insn| {
            println!("{:08x}: {}", insn.vaddr(), insn.disas()?);
            Ok(())
        })
    }
}


#[ctor]
fn init() {
    PLUGIN
        .set(Mutex::new(Box::new(TinyTrace {})))
        .map_err(|_| anyhow!("Failed to set plugin"))
        .expect("Failed to set plugin");
}
