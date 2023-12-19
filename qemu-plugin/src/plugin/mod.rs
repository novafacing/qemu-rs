use std::sync::{Mutex, OnceLock};

use crate::{
    install::{Args, Info},
    sys::{qemu_plugin_id_t, qemu_plugin_tb},
    TranslationBlock, VCPUIndex,
};
use crate::{
    qemu_plugin_register_atexit_cb, qemu_plugin_register_flush_cb,
    qemu_plugin_register_vcpu_exit_cb, qemu_plugin_register_vcpu_idle_cb,
    qemu_plugin_register_vcpu_init_cb, qemu_plugin_register_vcpu_resume_cb,
    qemu_plugin_register_vcpu_syscall_cb, qemu_plugin_register_vcpu_syscall_ret_cb,
    qemu_plugin_register_vcpu_tb_trans_cb,
};

extern "C" fn handle_qemu_plugin_register_vcpu_init_cb(id: qemu_plugin_id_t, vcpu_id: VCPUIndex) {
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

extern "C" fn handle_qemu_plugin_register_vcpu_exit_cb(id: qemu_plugin_id_t, vcpu_id: VCPUIndex) {
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

extern "C" fn handle_qemu_plugin_register_vcpu_idle_cb(id: qemu_plugin_id_t, vcpu_id: VCPUIndex) {
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

extern "C" fn handle_qemu_plugin_register_vcpu_resume_cb(id: qemu_plugin_id_t, vcpu_id: VCPUIndex) {
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
    id: qemu_plugin_id_t,
    tb: *mut qemu_plugin_tb,
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

extern "C" fn handle_qemu_plugin_register_flush_cb(id: qemu_plugin_id_t) {
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
    id: qemu_plugin_id_t,
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
    id: qemu_plugin_id_t,
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

pub trait Register: HasCallbacks + Send + Sync + 'static {
    #[allow(unused)]
    /// Called by QEMu when registering the plugin
    fn register(
        &mut self,
        id: qemu_plugin_id_t,
        args: &Args,
        info: &Info,
    ) -> Result<(), anyhow::Error> {
        if self.has_vcpu_init() {
            qemu_plugin_register_vcpu_init_cb(id, Some(handle_qemu_plugin_register_vcpu_init_cb))?;
        }

        if self.has_vcpu_exit() {
            qemu_plugin_register_vcpu_exit_cb(id, Some(handle_qemu_plugin_register_vcpu_exit_cb))?;
        }

        if self.has_vcpu_idle() {
            qemu_plugin_register_vcpu_idle_cb(id, Some(handle_qemu_plugin_register_vcpu_idle_cb))?;
        }

        if self.has_vcpu_resume() {
            qemu_plugin_register_vcpu_resume_cb(
                id,
                Some(handle_qemu_plugin_register_vcpu_resume_cb),
            )?;
        }

        if self.has_translation_block_translate() {
            qemu_plugin_register_vcpu_tb_trans_cb(
                id,
                Some(handle_qemu_plugin_register_vcpu_tb_trans_cb),
            )?;
        }

        if self.has_flush() {
            qemu_plugin_register_flush_cb(id, Some(handle_qemu_plugin_register_flush_cb));
        }

        if self.has_syscall() {
            qemu_plugin_register_vcpu_syscall_cb(id, Some(handle_qemu_plugin_register_syscall_cb));
        }

        if self.has_syscall_return() {
            qemu_plugin_register_vcpu_syscall_ret_cb(
                id,
                Some(handle_qemu_plugin_register_syscall_ret_cb),
            );
        }

        qemu_plugin_register_atexit_cb(id, |_| {});

        self.register_custom(id, args, info)?;

        Ok(())
    }

    #[allow(unused)]
    /// Called by the default `register` implementation when unregistering the plugin
    fn register_custom(
        &mut self,
        id: qemu_plugin_id_t,
        args: &Args,
        info: &Info,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

pub trait HasCallbacks: Send + Sync + 'static {
    fn has_vcpu_init(&mut self) -> bool {
        false
    }

    fn has_vcpu_exit(&mut self) -> bool {
        false
    }

    fn has_vcpu_idle(&mut self) -> bool {
        false
    }

    fn has_vcpu_resume(&mut self) -> bool {
        false
    }

    fn has_translation_block_translate(&mut self) -> bool {
        false
    }

    fn has_flush(&mut self) -> bool {
        false
    }

    fn has_syscall(&mut self) -> bool {
        false
    }

    fn has_syscall_return(&mut self) -> bool {
        false
    }

    #[allow(unused)]
    fn on_vcpu_init(
        &mut self,
        id: qemu_plugin_id_t,
        vcpu_id: VCPUIndex,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    fn on_vcpu_exit(
        &mut self,
        id: qemu_plugin_id_t,
        vcpu_id: VCPUIndex,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    fn on_vcpu_idle(
        &mut self,
        id: qemu_plugin_id_t,
        vcpu_id: VCPUIndex,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    fn on_vcpu_resume(
        &mut self,
        id: qemu_plugin_id_t,
        vcpu_id: VCPUIndex,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    fn on_translation_block_translate(
        &mut self,
        id: qemu_plugin_id_t,
        tb: TranslationBlock,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused)]
    fn on_flush(&mut self, id: qemu_plugin_id_t) -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[allow(unused, clippy::too_many_arguments)]
    fn on_syscall(
        &mut self,
        id: qemu_plugin_id_t,
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
    fn on_syscall_return(
        &mut self,
        id: qemu_plugin_id_t,
        vcpu_index: VCPUIndex,
        num: i64,
        ret: i64,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

pub trait Plugin: Register + HasCallbacks {}

pub static PLUGIN: OnceLock<Mutex<Box<dyn Plugin>>> = OnceLock::new();
