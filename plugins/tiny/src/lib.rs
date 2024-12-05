use anyhow::{anyhow, Result};
use ctor::ctor;
use qemu_plugin::{
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    PluginId, TranslationBlock,
};
#[cfg(not(feature = "plugin-api-v1"))]
use qemu_plugin::{qemu_plugin_get_registers, RegisterDescriptor, VCPUIndex};
use std::sync::Mutex;

#[derive(Default)]
struct TinyTrace {
    #[cfg(not(feature = "plugin-api-v1"))]
    registers: Vec<RegisterDescriptor<'static>>,
}

impl Plugin for TinyTrace {}
impl Register for TinyTrace {}

impl HasCallbacks for TinyTrace {
    #[cfg(not(feature = "plugin-api-v1"))]
    fn on_vcpu_init(&mut self, _id: PluginId, _vcpu_id: VCPUIndex) -> Result<()> {
        self.registers = qemu_plugin_get_registers()?;
        Ok(())
    }
    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        #[cfg(any(feature = "plugin-api-v2", feature = "plugin-api-v3"))]
        let registers = self.registers.clone();

        tb.instructions().try_for_each(|insn| {
            println!("{:08x}: {}", insn.vaddr(), insn.disas()?);

            #[cfg(any(feature = "plugin-api-v2", feature = "plugin-api-v3"))]
            {
                for register in &registers {
                    let value = register.read()?;
                    println!("    {}: {:?}", register.name, value);
                }
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
