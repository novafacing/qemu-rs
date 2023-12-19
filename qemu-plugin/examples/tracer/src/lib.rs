use anyhow::Result;
use ctor::ctor;
use qemu_plugin::{
    install::{Args, Info},
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    qemu_plugin_register_atexit_cb, PluginId, TranslationBlock,
};
use std::sync::{Arc, Mutex};

struct Tracer {
    tbs: Arc<Mutex<usize>>,
    insns: Arc<Mutex<usize>>,
}

impl Register for Tracer {
    fn register(&mut self, id: PluginId, _args: &Args, _info: &Info) -> Result<()> {
        let tbs = self.tbs.clone();
        let insns = self.insns.clone();

        qemu_plugin_register_atexit_cb(id, move |_| {
            println!("tbs: {}", tbs.lock().unwrap());
            println!("insns: {}", insns.lock().unwrap());
        })?;

        Ok(())
    }
}

impl HasCallbacks for Tracer {
    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        let tbs = self.tbs.clone();
        tb.register_execute_callback(move |_| {
            tbs.lock()
                .map(|mut tbs| {
                    *tbs += 1;
                })
                .map_err(|e| eprintln!("Failed to lock tbs: {:?}", e))
                .ok();
        });
        tb.instructions().enumerate().try_for_each(|(idx, insn)| {
            let insns = self.insns.clone();
            insn.register_execute_callback(move |_| {
                insns
                    .lock()
                    .map(|mut insns| {
                        *insns += 1;
                    })
                    .map_err(|e| eprintln!("Failed to lock insns: {:?}", e))
                    .ok();
            });

            if idx == 0 {
                println!("====TB: {:08x}", insn.vaddr());
            }

            println!("{:08x}: {}", insn.vaddr(), insn.disas()?);
            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }
}

impl Plugin for Tracer {}

#[ctor]
fn init() {
    PLUGIN
        .set(Mutex::new(Box::new(Tracer {
            tbs: Arc::new(Mutex::new(0)),
            insns: Arc::new(Mutex::new(0)),
        })))
        .map_err(|_| anyhow::anyhow!("Failed to set plugin"))
        .expect("Failed to set plugin");
}
