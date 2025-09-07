use qemu_plugin::{HasCallbacks, PluginId, Register, Result, register};

struct TinyTrace;

impl Register for TinyTrace {}

impl HasCallbacks for TinyTrace {
    fn on_vcpu_init(&mut self, id: PluginId, vcpu_id: qemu_plugin::VCPUIndex) -> Result<()> {
        println!("on_vcpu_init: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }

    fn on_vcpu_idle(&mut self, id: PluginId, vcpu_id: qemu_plugin::VCPUIndex) -> Result<()> {
        println!("on_vcpu_idle: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }

    fn on_vcpu_exit(&mut self, id: PluginId, vcpu_id: qemu_plugin::VCPUIndex) -> Result<()> {
        println!("on_vcpu_exit: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }

    fn on_vcpu_resume(&mut self, id: PluginId, vcpu_id: qemu_plugin::VCPUIndex) -> Result<()> {
        println!("on_vcpu_resume: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }
}

register!(TinyTrace);
