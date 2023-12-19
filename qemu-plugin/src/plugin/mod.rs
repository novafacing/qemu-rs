//! Traits and helpers enabling idiomatic QEMU plugin implementation

use std::sync::{Mutex, OnceLock};

use crate::{
    install::{Args, Info},
    PluginId, TranslationBlock, VCPUIndex,
};
use crate::{
    qemu_plugin_register_flush_cb, qemu_plugin_register_vcpu_exit_cb,
    qemu_plugin_register_vcpu_idle_cb, qemu_plugin_register_vcpu_init_cb,
    qemu_plugin_register_vcpu_resume_cb, qemu_plugin_register_vcpu_syscall_cb,
    qemu_plugin_register_vcpu_syscall_ret_cb, qemu_plugin_register_vcpu_tb_trans_cb,
};

extern "C" fn handle_qemu_plugin_register_vcpu_init_cb(id: PluginId, vcpu_id: VCPUIndex) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_vcpu_init(id, vcpu_id)
        .expect("Failed running callback on_vcpu_init");
}

extern "C" fn handle_qemu_plugin_register_vcpu_exit_cb(id: PluginId, vcpu_id: VCPUIndex) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_vcpu_exit(id, vcpu_id)
        .expect("Failed running callback on_vcpu_exit");
}

extern "C" fn handle_qemu_plugin_register_vcpu_idle_cb(id: PluginId, vcpu_id: VCPUIndex) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_vcpu_idle(id, vcpu_id)
        .expect("Failed running callback on_vcpu_idle");
}

extern "C" fn handle_qemu_plugin_register_vcpu_resume_cb(id: PluginId, vcpu_id: VCPUIndex) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_vcpu_resume(id, vcpu_id)
        .expect("Failed running callback on_vcpu_resume");
}

extern "C" fn handle_qemu_plugin_register_vcpu_tb_trans_cb(
    id: PluginId,
    tb: *mut crate::sys::qemu_plugin_tb,
) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    let tb = TranslationBlock::from(tb);

    plugin
        .on_translation_block_translate(id, tb)
        .expect("Failed running callback on_translation_block_translate");
}

extern "C" fn handle_qemu_plugin_register_flush_cb(id: PluginId) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_flush(id)
        .expect("Failed running callback on_flush");
}

extern "C" fn handle_qemu_plugin_register_syscall_cb(
    id: PluginId,
    vcpu_index: VCPUIndex,
    num: i64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_syscall(id, vcpu_index, num, a1, a2, a3, a4, a5, a6, a7, a8)
        .expect("Failed running callback on_syscall");
}

extern "C" fn handle_qemu_plugin_register_syscall_ret_cb(
    id: PluginId,
    vcpu_index: VCPUIndex,
    num: i64,
    ret: i64,
) {
    let Some(plugin) = PLUGIN.get() else {
        panic!("Plugin not set");
    };

    let Ok(mut plugin) = plugin.lock() else {
        panic!("Failed to lock plugin");
    };

    plugin
        .on_syscall_return(id, vcpu_index, num, ret)
        .expect("Failed running callback on_syscall_return");
}

/// Trait which implemenents registering the callbacks implemented on a struct which
/// `HasCallbacks` with QEMU
pub trait Register: HasCallbacks + Send + Sync + 'static {
    #[allow(unused)]
    /// Called by QEMu when registering the plugin. This method should only be overridden if no
    /// default callbacks are desired, and will require re-implementing handlers which is not
    /// recommended.
    fn register_default(
        &mut self,
        id: PluginId,
        args: &Args,
        info: &Info,
    ) -> Result<(), anyhow::Error> {
        qemu_plugin_register_vcpu_init_cb(id, Some(handle_qemu_plugin_register_vcpu_init_cb))?;

        qemu_plugin_register_vcpu_exit_cb(id, Some(handle_qemu_plugin_register_vcpu_exit_cb))?;

        qemu_plugin_register_vcpu_idle_cb(id, Some(handle_qemu_plugin_register_vcpu_idle_cb))?;

        qemu_plugin_register_vcpu_resume_cb(id, Some(handle_qemu_plugin_register_vcpu_resume_cb))?;

        qemu_plugin_register_vcpu_tb_trans_cb(
            id,
            Some(handle_qemu_plugin_register_vcpu_tb_trans_cb),
        )?;

        qemu_plugin_register_flush_cb(id, Some(handle_qemu_plugin_register_flush_cb));

        qemu_plugin_register_vcpu_syscall_cb(id, Some(handle_qemu_plugin_register_syscall_cb));

        qemu_plugin_register_vcpu_syscall_ret_cb(
            id,
            Some(handle_qemu_plugin_register_syscall_ret_cb),
        );

        self.register(id, args, info)?;

        Ok(())
    }

    #[allow(unused)]
    /// Called when registering the plugin. User definition of on-registration behavior should
    /// be implemented here.
    fn register(&mut self, id: PluginId, args: &Args, info: &Info) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

/// Trait implemented by structs which have callbacks which should be registered with QEMU
pub trait HasCallbacks: Send + Sync + 'static {
    #[allow(unused)]
    /// Callback triggered on vCPU init
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `vcpu_id` - The ID of the vCPU
    fn on_vcpu_init(&mut self, id: PluginId, vcpu_id: VCPUIndex) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    /// Callback triggered on vCPU exit
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `vcpu_id` - The ID of the vCPU
    fn on_vcpu_exit(&mut self, id: PluginId, vcpu_id: VCPUIndex) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    /// Callback triggered on vCPU idle
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `vcpu_id` - The ID of the vCPU
    fn on_vcpu_idle(&mut self, id: PluginId, vcpu_id: VCPUIndex) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    /// Callback triggered on vCPU resume
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `vcpu_id` - The ID of the vCPU
    fn on_vcpu_resume(&mut self, id: PluginId, vcpu_id: VCPUIndex) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    /// Callback triggered on translation block translation
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `tb` - The translation block
    fn on_translation_block_translate(
        &mut self,
        id: PluginId,
        tb: TranslationBlock,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    /// Callback triggered on flush
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    fn on_flush(&mut self, id: PluginId) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused, clippy::too_many_arguments)]
    /// Callback triggered on syscall
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `vcpu_index` - The ID of the vCPU
    /// * `num` - The syscall number
    /// * `a1` - The first syscall argument
    /// * `a2` - The second syscall argument
    /// * `a3` - The third syscall argument
    /// * `a4` - The fourth syscall argument
    /// * `a5` - The fifth syscall argument
    /// * `a6` - The sixth syscall argument
    /// * `a7` - The seventh syscall argument
    /// * `a8` - The eighth syscall argument
    fn on_syscall(
        &mut self,
        id: PluginId,
        vcpu_index: VCPUIndex,
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
        Ok(())
    }

    #[allow(unused)]
    /// Callback triggered on syscall return
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the plugin
    /// * `vcpu_index` - The ID of the vCPU
    /// * `num` - The syscall number
    /// * `ret` - The return value of the syscall
    fn on_syscall_return(
        &mut self,
        id: PluginId,
        vcpu_index: VCPUIndex,
        num: i64,
        ret: i64,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

/// Trait implemented by structs which are QEMU plugin contexts
pub trait Plugin: Register + HasCallbacks {}

/// The global plugin item
pub static PLUGIN: OnceLock<Mutex<Box<dyn Plugin>>> = OnceLock::new();
