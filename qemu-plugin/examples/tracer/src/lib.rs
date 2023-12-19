use anyhow::{anyhow, Error, Result};
use ctor::ctor;
use qemu_plugin::{
    install::{Args, Info, Value},
    plugin::{HasCallbacks, Plugin, Register, PLUGIN},
    Instruction, MemRW, MemoryInfo, PluginId, TranslationBlock, VCPUIndex,
};
use serde::{Deserialize, Serialize};
use serde_cbor::to_writer;
use std::{
    collections::HashMap,
    os::unix::net::UnixStream,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone, Debug, Deserialize, Serialize)]
pub struct InstructionEvent {
    pub vaddr: u64,
    pub haddr: u64,
    pub disas: String,
    pub symbol: Option<String>,
    pub data: Vec<u8>,
}

impl TryFrom<&Instruction<'_>> for InstructionEvent {
    type Error = Error;

    fn try_from(value: &Instruction) -> Result<Self> {
        Ok(Self::builder()
            .vaddr(value.vaddr())
            .haddr(value.haddr())
            .disas(value.disas()?)
            .symbol(value.symbol()?)
            .data(value.data())
            .build())
    }
}

#[derive(TypedBuilder, Clone, Debug, Deserialize, Serialize)]
pub struct MemoryEvent {
    pub vaddr: u64,
    pub haddr: Option<u64>,
    pub haddr_is_io: Option<bool>,
    pub haddr_device_name: Option<String>,
    pub size_shift: usize,
    pub size_bytes: usize,
    pub sign_extended: bool,
    pub is_store: bool,
    pub big_endian: bool,
}

impl MemoryEvent {
    fn try_from(value: &MemoryInfo, vaddr: u64) -> Result<Self> {
        let haddr = value.hwaddr(vaddr);
        Ok(Self::builder()
            .vaddr(vaddr)
            .haddr(haddr.as_ref().map(|h| h.hwaddr()))
            .haddr_is_io(haddr.as_ref().map(|h| h.is_io()))
            .haddr_device_name(haddr.and_then(|h| h.device_name().ok().flatten()))
            .size_shift(value.size_shift())
            .size_bytes(match value.size_shift() {
                0 => 1,
                1 => 2,
                2 => 4,
                3 => 8,
                _ => 0,
            })
            .sign_extended(value.sign_extended())
            .is_store(value.is_store())
            .big_endian(value.big_endian())
            .build())
    }
}

#[derive(TypedBuilder, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SyscallSource {
    plugin_id: PluginId,
    vcpu_index: VCPUIndex,
}

#[derive(TypedBuilder, Clone, Debug, Deserialize, Serialize)]
pub struct SyscallEvent {
    pub num: i64,
    pub return_value: i64,
    pub args: [u64; 8],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Event {
    Instruction(InstructionEvent),
    Memory(MemoryEvent),
    Syscall(SyscallEvent),
}

#[derive(TypedBuilder, Clone, Debug)]
struct Tracer {
    pub syscalls: Arc<Mutex<HashMap<SyscallSource, SyscallEvent>>>,
    #[builder(default)]
    pub tx: Arc<Mutex<Option<UnixStream>>>,
    #[builder(default)]
    pub log_insns: bool,
    #[builder(default)]
    pub log_mem: bool,
    #[builder(default)]
    pub log_syscalls: bool,
}

impl Tracer {
    pub fn new() -> Self {
        Self::builder()
            .syscalls(Arc::new(Mutex::new(HashMap::new())))
            .build()
    }
}

impl HasCallbacks for Tracer {
    fn on_translation_block_translate(
        &mut self,
        _id: PluginId,
        tb: TranslationBlock,
    ) -> Result<()> {
        tb.instructions().try_for_each(|insn| {
            let event = InstructionEvent::try_from(&insn)?;

            if self.log_insns {
                let tx = self.tx.clone();

                insn.register_execute_callback(move |_| {
                    tx.lock()
                        .map_err(|e| anyhow!("Failed to lock tx: {}", e))
                        .and_then(|tx| {
                            to_writer(
                                tx.as_ref().ok_or_else(|| anyhow!("No tx"))?,
                                &Event::Instruction(event.clone()),
                            )
                            .map_err(|e| anyhow!(e))
                        })
                        .expect("Failed to send instruction event");
                });
            }

            if self.log_mem {
                let tx = self.tx.clone();

                insn.register_memory_access_callback(
                    move |_, info, vaddr| {
                        tx.lock()
                            .map_err(|e| anyhow!("Failed to lock tx: {}", e))
                            .and_then(|tx| {
                                to_writer(
                                    tx.as_ref().ok_or_else(|| anyhow!("No tx"))?,
                                    &Event::Memory(MemoryEvent::try_from(&info, vaddr)?),
                                )
                                .map_err(|e| anyhow!(e))
                            })
                            .expect("Failed to send memory event");
                    },
                    MemRW::QEMU_PLUGIN_MEM_RW,
                );
            }

            Ok::<(), Error>(())
        })?;

        Ok(())
    }

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
    ) -> Result<()> {
        if !self.log_syscalls {
            return Ok(());
        }

        let event = SyscallEvent::builder()
            .num(num)
            .return_value(-1)
            .args([a1, a2, a3, a4, a5, a6, a7, a8])
            .build();

        let mut syscalls = self
            .syscalls
            .lock()
            .map_err(|e| anyhow!("Failed to lock syscalls: {e}"))?;

        syscalls.insert(
            SyscallSource::builder()
                .plugin_id(id)
                .vcpu_index(vcpu_index)
                .build(),
            event,
        );

        Ok(())
    }

    fn on_syscall_return(
        &mut self,
        id: PluginId,
        vcpu_index: VCPUIndex,
        _: i64,
        ret: i64,
    ) -> Result<()> {
        if !self.log_syscalls {
            return Ok(());
        }

        let mut syscalls = self
            .syscalls
            .lock()
            .map_err(|e| anyhow!("Failed to lock syscalls: {e}"))?;

        // Remove and return the syscall event
        let mut event = syscalls
            .remove(
                &SyscallSource::builder()
                    .plugin_id(id)
                    .vcpu_index(vcpu_index)
                    .build(),
            )
            .ok_or_else(|| anyhow!("No syscall event found"))?;

        // Update the return value
        event.return_value = ret;

        // Send the event
        let tx = self
            .tx
            .lock()
            .map_err(|e| anyhow!("Failed to lock tx: {e}"))?;
        let tx_stream = tx.as_ref().ok_or_else(|| anyhow!("No tx"))?;

        to_writer(tx_stream, &Event::Syscall(event)).map_err(|e| anyhow!(e))?;

        Ok(())
    }
}

#[derive(TypedBuilder, Clone, Debug)]
pub struct PluginArgs {
    pub log_insns: bool,
    pub log_mem: bool,
    pub log_syscalls: bool,
    pub socket_path: PathBuf,
}

impl TryFrom<&Args> for PluginArgs {
    type Error = Error;

    fn try_from(value: &Args) -> Result<Self> {
        Ok(Self::builder()
            .log_insns(
                value
                    .parsed
                    .get("log_insns")
                    .map(|li| if let Value::Bool(v) = li { *v } else { false })
                    .unwrap_or_default(),
            )
            .log_mem(
                value
                    .parsed
                    .get("log_mem")
                    .map(|lm| if let Value::Bool(v) = lm { *v } else { false })
                    .unwrap_or_default(),
            )
            .log_syscalls(
                value
                    .parsed
                    .get("log_syscalls")
                    .map(|ls| if let Value::Bool(v) = ls { *v } else { false })
                    .unwrap_or_default(),
            )
            .socket_path(
                value
                    .parsed
                    .get("socket_path")
                    .and_then(|sp| {
                        if let Value::String(v) = sp {
                            Some(PathBuf::from(v))
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| anyhow!("No socket path provided"))?,
            )
            .build())
    }
}

impl Register for Tracer {
    fn register(&mut self, _: PluginId, args: &Args, _: &Info) -> Result<()> {
        let plugin_args = PluginArgs::try_from(args)?;

        self.tx = Arc::new(Mutex::new(Some(UnixStream::connect(
            plugin_args.socket_path,
        )?)));

        self.log_insns = plugin_args.log_insns;
        self.log_mem = plugin_args.log_mem;
        self.log_syscalls = plugin_args.log_syscalls;

        Ok(())
    }
}

impl Plugin for Tracer {}

#[ctor]
fn init() {
    PLUGIN
        .set(Mutex::new(Box::new(Tracer::new())))
        .map_err(|_| anyhow::anyhow!("Failed to set plugin"))
        .expect("Failed to set plugin");
}
