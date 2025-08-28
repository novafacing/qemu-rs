use anyhow::anyhow;
use ctor::ctor;
use qemu_plugin::{
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    PluginId,
};
use std::sync::Mutex;

struct TinyTrace {}

impl Plugin for TinyTrace {}
impl Register for TinyTrace {}

impl HasCallbacks for TinyTrace {
    fn on_vcpu_init(
        &mut self,
        id: PluginId,
        vcpu_id: qemu_plugin::VCPUIndex,
    ) -> std::prelude::v1::Result<(), anyhow::Error> {
        println!("on_vcpu_init: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }

    fn on_vcpu_idle(
        &mut self,
        id: PluginId,
        vcpu_id: qemu_plugin::VCPUIndex,
    ) -> std::prelude::v1::Result<(), anyhow::Error> {
        println!("on_vcpu_idle: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }

    fn on_vcpu_exit(
        &mut self,
        id: PluginId,
        vcpu_id: qemu_plugin::VCPUIndex,
    ) -> std::prelude::v1::Result<(), anyhow::Error> {
        println!("on_vcpu_exit: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }

    fn on_vcpu_resume(
        &mut self,
        id: PluginId,
        vcpu_id: qemu_plugin::VCPUIndex,
    ) -> std::prelude::v1::Result<(), anyhow::Error> {
        println!("on_vcpu_resume: id: {id:?}, vcpu_id: {vcpu_id:?}");
        Ok(())
    }
}

#[ctor]
fn init() {
    PLUGIN
        .set(Mutex::new(Box::new(TinyTrace {})))
        .map_err(|_| anyhow!("Failed to set plugin"))
        .expect("Failed to set plugin");
}
