use anyhow::{anyhow, Result};
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
    fn on_syscall(
        &mut self,
        id: PluginId,
        vcpu_index: qemu_plugin::VCPUIndex,
        num: i64,
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ) -> Result<(), anyhow::Error> {
        println!(
            "on_syscall: id: {:?}, vcpu_index: {:?}, num: {:?}, a1: {:?}, a2: {:?}, a3: {:?}, a4: {:?}, a5: {:?}, a6: {:?}, a7: {:?}, a8: {:?}",
            id, vcpu_index, num, a1, a2, a3, a4, a5, a6, a7, a8
        );
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
