use anyhow::Result;
use ctor::ctor;
use qemu_plugin::{
    plugin::{init_plugin, HasCallbacks, Register},
    PluginId, TranslationBlock,
};
#[cfg(not(feature = "plugin-api-v1"))]
use qemu_plugin::{qemu_plugin_get_registers, RegisterDescriptor, VCPUIndex};

#[derive(Default)]
struct TinyTrace {
    #[cfg(not(feature = "plugin-api-v1"))]
    registers: Vec<RegisterDescriptor<'static>>,
}

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
    init_plugin(TinyTrace::default()).expect("Failed to initialize plugin");
}
