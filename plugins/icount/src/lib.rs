use anyhow::Result;
use qemu_plugin::{
    PluginId, TranslationBlock, VCPUIndex,
    plugin::{HasCallbacks, Register},
    register,
};

#[derive(Default)]
struct ICount;

impl Register for ICount {}

impl HasCallbacks for ICount {
    fn on_vcpu_init(&mut self, _id: PluginId, _vcpu_id: VCPUIndex) -> Result<()> {
        Ok(())
    }

    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        tb.instructions().try_for_each(|insn| {
            let vaddr = insn.vaddr();
            let disas = insn.disas()?;

            insn.register_execute_callback(move |_idx| {
                println!("{vaddr:08x}: {disas}");
            });

            Ok(())
        })
    }
}

register!(ICount);
