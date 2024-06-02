use anyhow::{anyhow, Result};
use ctor::ctor;
use qemu_plugin::{
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    PluginId, TranslationBlock, VCPUIndex, qemu_plugin_get_registers,
    RegisterDescriptor,
};
use std::sync::Mutex;

#[derive(Default)]
struct TinyTrace {
    registers: Vec<RegisterDescriptor<'static>>,
}

impl Plugin for TinyTrace {}
impl Register for TinyTrace {}

impl HasCallbacks for TinyTrace {
    fn on_vcpu_init(&mut self, _id: PluginId, _vcpu_id: VCPUIndex) -> Result<()> {
        self.registers = qemu_plugin_get_registers()?;
        Ok(())
    }
    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        let registers = self.registers.clone();
        tb.instructions().try_for_each(|insn| {
            println!("{:08x}: {}", insn.vaddr(), insn.disas()?);
            for register in &registers {
                let value = register.read()?;
                println!("    {}: {:?}", register.name, value);
            }

            Ok(())
        })
    }
}


#[ctor]
fn init() {
    PLUGIN
        .set(Mutex::new(Box::new(TinyTrace::default())))
        .map_err(|_| anyhow!("Failed to set plugin"))
        .expect("Failed to set plugin");
}
