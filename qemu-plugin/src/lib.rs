//! High level idiomatic Rust bindings to the QEMU plugin API
//!
//! `qemu-plugin` makes it easy to write QEMU plugins in Rust.
//!
//! # Example
//!
//! Below is a minimal plugin example for a plugin which prints the execution trace of the
//! program running in QEMU. Notice that all we do is register a struct which implements
//! `Plugin`, and the library takes care of the rest.
//!
//! ```rust,ignore
//! use anyhow::Result;
//! use ctor::ctor;
//! use qemu_plugin::{
//!     plugin::{HasCallbacks, Plugin, Register, PLUGIN},
//!     PluginId, TranslationBlock,
//! };
//! use std::sync::Mutex;
//!
//! struct TinyTrace {}
//!
//! impl Register for TinyTrace {}
//!
//! impl HasCallbacks for TinyTrace {
//!     fn on_translation_block_translate(
//!         &mut self,
//!         _id: PluginId,
//!         tb: TranslationBlock,
//!     ) -> Result<()> {
//!         tb.instructions().enumerate().try_for_each(|(idx, insn)| {
//!             if idx == 0 {
//!                 println!("====TB: {:08x}", insn.vaddr());
//!             }
//!
//!             println!("{:08x}: {}", insn.vaddr(), insn.disas()?);
//!             Ok::<(), anyhow::Error>(())
//!         })?;
//!
//!         Ok(())
//!     }
//! }
//!
//! impl Plugin for TinyTrace {}
//!
//! #[ctor]
//! fn init() {
//!     PLUGIN
//!         .set(Mutex::new(Box::new(TinyTrace {})))
//!         .map_err(|_| anyhow::anyhow!("Failed to set plugin"))
//!         .expect("Failed to set plugin");
//! }
//! ```
//!
//! The above `src/lib.rs` in a Cargo project with the following `Cargo.toml` will compile to
//! `libtiny.so`, which can be loaded in QEMU by running `qemu-system-ARCH -plugin ./libtiny.so`.
//!
//! ```toml
//! [package]
//! name = "tiny"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! qemu-plugin = "8.1.3-v1"
//! anyhow = "1.0.75"
//! ffi = "0.1.0"
//! ctor = "0.2.6"
//! ```

#![deny(missing_docs)]
#![cfg_attr(all(unix, feature = "unix-weak-link"), feature(linkage))]

#[cfg(all(unix, feature = "unix-weak-link"))]
mod unix_weak_link;

#[cfg(windows)]
mod win_link_hook;

use crate::error::{Error, Result};
use qemu_plugin_sys::{
    qemu_plugin_cb_flags, qemu_plugin_hwaddr, qemu_plugin_id_t, qemu_plugin_insn,
    qemu_plugin_mem_rw, qemu_plugin_meminfo_t, qemu_plugin_simple_cb_t, qemu_plugin_tb,
    qemu_plugin_vcpu_simple_cb_t, qemu_plugin_vcpu_syscall_cb_t, qemu_plugin_vcpu_syscall_ret_cb_t,
    qemu_plugin_vcpu_tb_trans_cb_t,
};
use std::{
    ffi::{c_uint, c_void, CStr, CString},
    marker::PhantomData,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

pub mod error;
pub mod install;
pub mod plugin;
pub mod sys;

#[cfg(not(windows))]
extern "C" {
    /// glib g_free is provided by the QEMU program we are being linked into
    fn g_free(mem: *mut c_void);
}

#[cfg(windows)]
unsafe fn g_free(_mem: *mut c_void) {
    //TODO: We would really like to call g_free in the qemu binary here
    //but we can't, because windows doesn't export symbols unless you explicitly export them
    //and g_free isn't so exported.

    //For now, we're just going to leak.
}

/// The index of a vCPU
pub type VCPUIndex = c_uint;
/// Flags for callbacks
pub type CallbackFlags = qemu_plugin_cb_flags;
/// Memory read/write flags
pub type MemRW = qemu_plugin_mem_rw;
/// A plugin ID
pub type PluginId = qemu_plugin_id_t;

/// A callback that can be called many times, each time a vCPU is initialized
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `vcpu_index`: The index of the vCPU that was initialized
pub type VCPUInitCallback = qemu_plugin_vcpu_simple_cb_t;

/// A callback that can be called many times, each time a vCPU exits
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `vcpu_index`: The index of the vCPU that exited
pub type VCPUExitCallback = qemu_plugin_vcpu_simple_cb_t;

/// A callback that can be called many times, each time a vCPU idles
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `vcpu_index`: The index of the vCPU that idled
pub type VCPUIdleCallback = qemu_plugin_vcpu_simple_cb_t;

/// A callback that can be called many times, each time a vCPU is resumed
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `vcpu_index`: The index of the vCPU that resumed
pub type VCPUResumeCallback = qemu_plugin_vcpu_simple_cb_t;

/// A callback that can be called many times, each time a translation occurs.  The
/// callback is passed an opaque `qemu_plugin_tb` pointer, which can be queried for
/// additional information including the list of translated instructions.  The callback
/// can register further callbacks to be triggered when the block or individual
/// instructions execute.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `tb`: The translated block
pub type VCPUTranslationBlockTranslationCallback = qemu_plugin_vcpu_tb_trans_cb_t;

/// A callback called on flush.
///
/// # Arguments
///
/// - `id`: The plugin ID
pub type FlushCallback = qemu_plugin_simple_cb_t;

/// A callback called on Syscall entry
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `vcpu_index`: The index of the vCPU that executed the instruction
/// - `num`: The syscall number
/// - `a1`: The first syscall argument
/// - `a2`: The second syscall argument
/// - `a3`: The third syscall argument
/// - `a4`: The fourth syscall argument
/// - `a5`: The fifth syscall argument
/// - `a6`: The sixth syscall argument
/// - `a7`: The seventh syscall argument
/// - `a8`: The eighth syscall argument
pub type SyscallCallback = qemu_plugin_vcpu_syscall_cb_t;

/// A callback called on Syscall return
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `vcpu_index`: The index of the vCPU that executed the instruction
/// - `num`: The syscall number
/// - `ret`: The syscall return value
pub type SyscallReturnCallback = qemu_plugin_vcpu_syscall_ret_cb_t;

/// Wrapper structure for a `qemu_plugin_tb *`
///
/// # Safety
///
/// This structure is safe to use as long as the pointer is valid. The pointer is
/// always opaque, and therefore may not be dereferenced.
pub struct TranslationBlock<'a> {
    translation_block: usize,
    marker: PhantomData<&'a ()>,
}

impl<'a> From<*mut qemu_plugin_tb> for TranslationBlock<'a> {
    fn from(tb: *mut qemu_plugin_tb) -> Self {
        Self {
            translation_block: tb as usize,
            marker: PhantomData,
        }
    }
}

impl<'a> TranslationBlock<'a> {
    /// Returns the number of instructions in the translation block
    pub fn size(&self) -> usize {
        unsafe { crate::sys::qemu_plugin_tb_n_insns(self.translation_block as *mut qemu_plugin_tb) }
    }

    /// Returns the virtual address for the start of a translation block
    pub fn vaddr(&self) -> u64 {
        unsafe { crate::sys::qemu_plugin_tb_vaddr(self.translation_block as *mut qemu_plugin_tb) }
    }

    /// Returns the instruction in the translation block at `index`. If the index is out of bounds,
    /// an error is returned.
    ///
    /// # Arguments
    ///
    /// - `index`: The index of the instruction to return
    pub fn instruction(&'a self, index: usize) -> Result<Instruction<'a>> {
        let size = self.size();

        if index >= size {
            Err(Error::InvalidInstructionIndex { index, size })
        } else {
            Ok(Instruction::new(self, unsafe {
                crate::sys::qemu_plugin_tb_get_insn(
                    self.translation_block as *mut qemu_plugin_tb,
                    index,
                )
            }))
        }
    }

    /// Returns an iterator over the instructions in the translation block
    pub fn instructions(&'a self) -> TranslationBlockIterator<'a> {
        TranslationBlockIterator { tb: self, index: 0 }
    }

    /// Register a callback to be run on execution of this translation block
    pub fn register_execute_callback<F>(&self, cb: F)
    where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        let callback = Box::new(cb);
        let callback_box = Box::new(callback);
        let userdata = Box::into_raw(callback_box) as *mut c_void;

        unsafe {
            crate::sys::qemu_plugin_register_vcpu_tb_exec_cb(
                self.translation_block as *mut qemu_plugin_tb,
                Some(handle_qemu_plugin_register_vcpu_tb_exec_cb::<F>),
                // NOTE: Not checked, and is an error to specify any other value
                CallbackFlags::QEMU_PLUGIN_CB_NO_REGS,
                userdata,
            )
        };
    }
}

/// An iterator over the instructions of a translation block
pub struct TranslationBlockIterator<'a> {
    tb: &'a TranslationBlock<'a>,
    index: usize,
}

impl<'a> Iterator for TranslationBlockIterator<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.tb.size();

        if self.index >= size {
            None
        } else {
            let insn = self.tb.instruction(self.index).ok();
            self.index += 1;
            insn
        }
    }
}

/// Wrapper structure for a `qemu_plugin_insn *`
///
/// # Safety
///
/// This structure is safe to use as long as the pointer is valid. The pointer is
/// always opaque, and therefore may not be dereferenced.
pub struct Instruction<'a> {
    #[allow(unused)]
    // NOTE: This field may be useful in the future
    translation_block: &'a TranslationBlock<'a>,
    instruction: usize,
    marker: PhantomData<&'a ()>,
}

impl<'a> Instruction<'a> {
    fn new(translation_block: &'a TranslationBlock<'a>, insn: *mut qemu_plugin_insn) -> Self {
        Self {
            translation_block,
            instruction: insn as usize,
            marker: PhantomData,
        }
    }
}

impl<'a> Instruction<'a> {
    /// Returns the data for this instruction. This method may only be called inside the
    /// callback in which the instruction is obtained, but the resulting data is owned.
    pub fn data(&self) -> Vec<u8> {
        let size = self.size();
        let mut data = Vec::with_capacity(size);

        let insn_data =
            unsafe { crate::sys::qemu_plugin_insn_data(self.instruction as *mut qemu_plugin_insn) }
                as *mut u8;

        unsafe {
            data.set_len(size);
            std::ptr::copy_nonoverlapping(insn_data, data.as_mut_ptr(), size);
        }

        data
    }

    /// Returns the size of the data for this instruction
    pub fn size(&self) -> usize {
        unsafe { crate::sys::qemu_plugin_insn_size(self.instruction as *mut qemu_plugin_insn) }
    }

    /// Returns the virtual address of this instruction
    pub fn vaddr(&self) -> u64 {
        unsafe { crate::sys::qemu_plugin_insn_vaddr(self.instruction as *mut qemu_plugin_insn) }
    }

    /// Returns the hardware (physical) address of this instruction
    pub fn haddr(&self) -> u64 {
        (unsafe { crate::sys::qemu_plugin_insn_haddr(self.instruction as *mut qemu_plugin_insn) })
            as usize as u64
    }

    /// Returns the textual disassembly of this instruction
    pub fn disas(&self) -> Result<String> {
        let disas = unsafe {
            crate::sys::qemu_plugin_insn_disas(self.instruction as *mut qemu_plugin_insn)
        };
        if disas.is_null() {
            Err(Error::NoDisassemblyString)
        } else {
            let disas_string = unsafe { CStr::from_ptr(disas) }.to_str()?.to_string();

            // NOTE: The string is allocated, so we free it
            unsafe { g_free(disas as *mut _) };

            Ok(disas_string)
        }
    }

    /// Returns the symbol associated with this instruction, if one exists and the
    /// binary contains a symbol table
    pub fn symbol(&self) -> Result<Option<String>> {
        let symbol = unsafe {
            crate::sys::qemu_plugin_insn_symbol(self.instruction as *mut qemu_plugin_insn)
        };
        if symbol.is_null() {
            Ok(None)
        } else {
            let symbol_string = unsafe { CStr::from_ptr(symbol) }.to_str()?.to_string();
            // NOTE: The string is static, so we do not free it
            Ok(Some(symbol_string))
        }
    }

    /// Register a callback to be run on execution of this instruction
    pub fn register_execute_callback<F>(&self, cb: F)
    where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        let callback = Box::new(cb);
        let callback_box = Box::new(callback);
        let userdata = Box::into_raw(callback_box) as *mut c_void;

        unsafe {
            crate::sys::qemu_plugin_register_vcpu_insn_exec_cb(
                self.instruction as *mut qemu_plugin_insn,
                Some(handle_qemu_plugin_register_vcpu_insn_exec_cb::<F>),
                // NOTE: Not checked, and is an error to specify any other value
                CallbackFlags::QEMU_PLUGIN_CB_NO_REGS,
                userdata,
            )
        };
    }

    /// Register a callback to be run on memory access of this instruction
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `rw`: The type of memory access to trigger the callback on
    pub fn register_memory_access_callback<F>(&self, cb: F, rw: MemRW)
    where
        F: FnMut(VCPUIndex, MemoryInfo, u64) + Send + Sync + 'static,
    {
        let callback = Box::new(cb);
        let callback_box = Box::new(callback);
        let userdata = Box::into_raw(callback_box) as *mut c_void;

        unsafe {
            crate::sys::qemu_plugin_register_vcpu_mem_cb(
                self.instruction as *mut qemu_plugin_insn,
                Some(handle_qemu_plugin_register_vcpu_mem_cb::<F>),
                CallbackFlags::QEMU_PLUGIN_CB_NO_REGS,
                rw,
                userdata,
            )
        };
    }
}

/// Wrapper structure for a `qemu_plugin_meminfo_t`
///
/// # Safety
///
/// This structure is safe to use during the invocation of the callback which receives it as an
/// argument. The structure is always opaque, and therefore may not be accessed directly.
pub struct MemoryInfo<'a> {
    memory_info: qemu_plugin_meminfo_t,
    marker: PhantomData<&'a ()>,
}

impl<'a> From<qemu_plugin_meminfo_t> for MemoryInfo<'a> {
    fn from(info: qemu_plugin_meminfo_t) -> Self {
        Self {
            memory_info: info,
            marker: PhantomData,
        }
    }
}

impl<'a> MemoryInfo<'a> {
    /// Returns the size of the access in base-2, e.g. 0 for byte, 1 for 16-bit, 2 for
    /// 32-bit, etc.
    pub fn size_shift(&self) -> usize {
        (unsafe { crate::sys::qemu_plugin_mem_size_shift(self.memory_info) }) as usize
    }

    /// Returns whether the access was sign extended
    pub fn sign_extended(&self) -> bool {
        unsafe { crate::sys::qemu_plugin_mem_is_sign_extended(self.memory_info) }
    }

    /// Returns whether the access was big-endian
    pub fn big_endian(&self) -> bool {
        unsafe { crate::sys::qemu_plugin_mem_is_big_endian(self.memory_info) }
    }

    /// Returns whether the access was a store
    pub fn is_store(&self) -> bool {
        unsafe { crate::sys::qemu_plugin_mem_is_store(self.memory_info) }
    }

    /// Return a handle to query details about the physical address backing the virtual address
    /// in system emulation. In user-mode, this method always returns `None`.
    pub fn hwaddr(&self, vaddr: u64) -> Option<HwAddr> {
        let hwaddr = unsafe { crate::sys::qemu_plugin_get_hwaddr(self.memory_info, vaddr) };
        if hwaddr.is_null() {
            None
        } else {
            Some(HwAddr::from(hwaddr))
        }
    }
}

/// Wrapper structure for a `qemu_plugin_hwaddr *`
///
/// # Safety
///
/// This structure is safe to use as long as the pointer is valid. The pointer is
/// always opaque, and therefore may not be dereferenced.
pub struct HwAddr<'a> {
    hwaddr: usize,
    marker: PhantomData<&'a ()>,
}

impl<'a> From<*mut qemu_plugin_hwaddr> for HwAddr<'a> {
    fn from(hwaddr: *mut qemu_plugin_hwaddr) -> Self {
        Self {
            hwaddr: hwaddr as usize,
            marker: PhantomData,
        }
    }
}

impl<'a> HwAddr<'a> {
    /// Returns whether the memory operation is to MMIO. Returns false if the operation is to
    /// RAM.
    pub fn is_io(&self) -> bool {
        unsafe { crate::sys::qemu_plugin_hwaddr_is_io(self.hwaddr as *mut qemu_plugin_hwaddr) }
    }

    /// Returns the physical address for the memory operation
    pub fn hwaddr(&self) -> u64 {
        unsafe { crate::sys::qemu_plugin_hwaddr_phys_addr(self.hwaddr as *mut qemu_plugin_hwaddr) }
    }

    /// Returns a string representing the device
    pub fn device_name(&self) -> Result<Option<String>> {
        let device_name = unsafe {
            crate::sys::qemu_plugin_hwaddr_device_name(self.hwaddr as *mut qemu_plugin_hwaddr)
        };

        if device_name.is_null() {
            Ok(None)
        } else {
            let device_name_string = unsafe { CStr::from_ptr(device_name) }.to_str()?.to_string();
            // NOTE: The string is static, so we do not free it
            Ok(Some(device_name_string))
        }
    }
}

// NOTE: Box<Box< is not strictly necessary here because the pointer is never sent via
// FFI which means we never downcast to an 8-byte pointer from fat, but it is best not
// to rely on that.

#[allow(clippy::type_complexity)]
/// A callback which will run once removal and uninstallation of the plugin is finalized. This callback
/// can only be set once, by calling the `qemu_plugin_uninstall` function.
static UNINSTALL_CALLBACK: OnceLock<
    Mutex<Option<Box<Box<dyn FnOnce(qemu_plugin_id_t) + Send + Sync + 'static>>>>,
> = OnceLock::new();
#[allow(clippy::type_complexity)]
/// A callback which will run once the plugin is reset. This callback is set by calling the
/// `qemu_plugin_reset` function.
static RESET_CALLBACK: OnceLock<
    Mutex<Option<Box<Box<dyn FnOnce(qemu_plugin_id_t) + Send + Sync + 'static>>>>,
> = OnceLock::new();

/// Handle the invocation of the uninstall callback by calling the stored
/// callback closure, if one exists.
extern "C" fn handle_qemu_plugin_uninstall_callback(id: qemu_plugin_id_t) {
    if let Some(callback) = UNINSTALL_CALLBACK.get() {
        if let Ok(mut callback) = callback.lock() {
            if let Some(callback) = callback.take() {
                callback(id);
            }
        }
    }
    // NOTE: An error here is ignored, and exceedingly fatal
}

/// Handle the invocation of the reset callback by calling the stored
/// callback closure, if one exists.
extern "C" fn handle_qemu_plugin_reset_callback(id: qemu_plugin_id_t) {
    if let Some(callback) = UNINSTALL_CALLBACK.get() {
        if let Ok(mut callback) = callback.lock() {
            if let Some(callback) = callback.take() {
                callback(id);
            }
        }
    }
    // NOTE: An error here is ignored, and exceedingly fatal
}

/// Uninstall a plugin.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: A callback function that will be called when the plugin has been
/// uninstalled.
///
/// # Safety
///
/// Do NOT assume that the plugin has been uninstalled once this function returns.
/// Plugins are uninstalled asynchronously, and therefore the given plugin receives
/// callbacks until cb is called. This function must not be called from
/// `qemu_plugin_install`.
pub fn qemu_plugin_uninstall<F>(id: qemu_plugin_id_t, cb: F) -> Result<()>
where
    F: FnOnce(qemu_plugin_id_t) + Send + Sync + 'static,
{
    UNINSTALL_CALLBACK
        .set(Mutex::new(Some(Box::new(Box::new(cb)))))
        .map_err(|_| Error::ConcurrentPluginUninstallCallbackSet)?;

    unsafe { crate::sys::qemu_plugin_uninstall(id, Some(handle_qemu_plugin_uninstall_callback)) };

    Ok(())
}

/// Reset a plugin
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: A callback function that will be called when the plugin has been reset.
///
/// # Safety
///
/// Do NOT assume that the plugin has been reset once this function returns. Plugins
/// are reset asynchronously, and therefore the given plugin receives callbacks until
/// cb is called.
pub fn qemu_plugin_reset<F>(id: qemu_plugin_id_t, cb: F) -> Result<()>
where
    F: FnOnce(qemu_plugin_id_t) + Send + Sync + 'static,
{
    if let Some(callback) = RESET_CALLBACK.get() {
        let Ok(mut callback) = callback.lock() else {
            return Err(Error::PluginResetCallbackState);
        };
        let _ = callback.replace(Box::new(Box::new(cb)));
    } else {
        RESET_CALLBACK
            .set(Mutex::new(Some(Box::new(Box::new(cb)))))
            .map_err(|_| Error::ConcurrentPluginResetCallbackSet)?;
    }

    unsafe { crate::sys::qemu_plugin_reset(id, Some(handle_qemu_plugin_reset_callback)) };

    Ok(())
}

/// Register a callback to be called when a vCPU is initialized. The callback does not receive
/// user data, so it is not possible to register it via closure.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_init_cb(id: qemu_plugin_id_t, cb: VCPUInitCallback) -> Result<()> {
    unsafe { crate::sys::qemu_plugin_register_vcpu_init_cb(id, cb) };
    Ok(())
}

/// Register a callback to be called when a vCPU exits. The callback does not receive
/// user data, so it is not possible to register it via closure.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_exit_cb(id: qemu_plugin_id_t, cb: VCPUExitCallback) -> Result<()> {
    unsafe { crate::sys::qemu_plugin_register_vcpu_exit_cb(id, cb) };
    Ok(())
}

/// Register a callback to be called when a vCPU idles. The callback does not receive
/// user data, so it is not possible to register it via closure.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_idle_cb(id: qemu_plugin_id_t, cb: VCPUIdleCallback) -> Result<()> {
    unsafe { crate::sys::qemu_plugin_register_vcpu_idle_cb(id, cb) };
    Ok(())
}

/// Register a callback to be called when a vCPU resumes. The callback does not receive
/// user data, so it is not possible to register it via closure.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_resume_cb(
    id: qemu_plugin_id_t,
    cb: VCPUResumeCallback,
) -> Result<()> {
    unsafe { crate::sys::qemu_plugin_register_vcpu_resume_cb(id, cb) };
    Ok(())
}

/// Register a callback to be called when a translation block is translated. The callback
/// receives a pointer to a `qemu_plugin_tb` structure, which can be queried for additional
/// information including the list of translated instructions. The callback can register
/// further callbacks to be triggered when the block or individual instructions execute.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_tb_trans_cb(
    id: qemu_plugin_id_t,
    cb: VCPUTranslationBlockTranslationCallback,
) -> Result<()> {
    unsafe { crate::sys::qemu_plugin_register_vcpu_tb_trans_cb(id, cb) };
    Ok(())
}

extern "C" fn handle_qemu_plugin_register_vcpu_tb_exec_cb<F>(
    vcpu_index: VCPUIndex,
    userdata: *mut c_void,
) where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    let mut cb: Box<Box<F>> = unsafe { Box::from_raw(userdata as *mut _) };
    cb(vcpu_index);
    Box::leak(cb);
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register a callback to be called when a translation block is executed.
///
/// # Arguments
///
/// - `tb`: The translated block to register the execution callback for
/// - `cb`: The callback to be called when the block `tb` is executed
///
/// # Safety
///
/// This function is safe when the pointer `tb` is a valid pointer to a `qemu_plugin_tb`
/// structure, which is always opaque.
pub fn qemu_plugin_register_vcpu_tb_exec_cb<F>(tb: TranslationBlock, cb: F)
where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    tb.register_execute_callback(cb);
}

extern "C" fn handle_qemu_plugin_register_vcpu_insn_exec_cb<F>(
    vcpu_index: VCPUIndex,
    userdata: *mut c_void,
) where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    let mut cb: Box<Box<F>> = unsafe { Box::from_raw(userdata as *mut _) };
    cb(vcpu_index);
    // NOTE: This memory will be freed on plugin exit
    Box::leak(cb);
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register a callback to be called when an instruction is executed.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_insn_exec_cb<F>(insn: Instruction, cb: F)
where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    insn.register_execute_callback(cb);
}

extern "C" fn handle_qemu_plugin_register_vcpu_mem_cb<F>(
    vcpu_index: VCPUIndex,
    meminfo: qemu_plugin_meminfo_t,
    vaddr: u64,
    userdata: *mut c_void,
) where
    F: FnMut(VCPUIndex, MemoryInfo, u64) + Send + Sync + 'static,
{
    let mut cb: Box<Box<F>> = unsafe { Box::from_raw(userdata as *mut _) };
    let meminfo = MemoryInfo::from(meminfo);
    cb(vcpu_index, meminfo, vaddr);
    // NOTE: This memory will be freed on plugin exit
    Box::leak(cb);
}

/// Register a callback for every memory transaction of a particular instruction. If the
/// instruction is executed multiple times, the callback will be called multiple times.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `cb`: The callback to be called
/// - `rw`: Whether the callback should be called for reads, writes, or both
pub fn qemu_plugin_register_vcpu_mem_cb<F>(insn: Instruction, cb: F, rw: MemRW)
where
    F: FnMut(VCPUIndex, MemoryInfo, u64) + Send + Sync + 'static,
{
    insn.register_memory_access_callback(cb, rw);
}

extern "C" fn handle_qemu_plugin_register_atexit_cb<F>(id: qemu_plugin_id_t, userdata: *mut c_void)
where
    F: FnOnce(qemu_plugin_id_t) + Send + Sync + 'static,
{
    let cb: Box<Box<F>> = unsafe { Box::from_raw(userdata as *mut _) };
    cb(id);
    // NOTE: This memory is not leaked because this is the last callback to be called
    // and it can only be called once, so we allow it to drop
}

/// Register a callback to run once execution is finished. Plugins should be able to free all
/// their resources at this point.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_atexit_cb<F>(id: qemu_plugin_id_t, cb: F) -> Result<()>
where
    F: FnOnce(qemu_plugin_id_t) + Send + Sync + 'static,
{
    let callback = Box::new(cb);
    let callback_box = Box::new(callback);
    unsafe {
        crate::sys::qemu_plugin_register_atexit_cb(
            id,
            Some(handle_qemu_plugin_register_atexit_cb::<F>),
            Box::into_raw(callback_box) as *mut c_void,
        )
    };
    Ok(())
}

/// Register a callback to run on flush.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_flush_cb(id: qemu_plugin_id_t, cb: FlushCallback) {
    unsafe { crate::sys::qemu_plugin_register_flush_cb(id, cb) };
}

/// Register a callback to run on Syscall
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_syscall_cb(id: qemu_plugin_id_t, cb: SyscallCallback) {
    unsafe { crate::sys::qemu_plugin_register_vcpu_syscall_cb(id, cb) };
}

/// Register a callback to run on Syscall return
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: The callback to be called
pub fn qemu_plugin_register_vcpu_syscall_ret_cb(id: qemu_plugin_id_t, cb: SyscallReturnCallback) {
    unsafe { crate::sys::qemu_plugin_register_vcpu_syscall_ret_cb(id, cb) };
}

/// Output a string via the QEMU logging mechanism
pub fn qemu_plugin_outs<S>(string: S) -> Result<()>
where
    S: AsRef<str>,
{
    unsafe {
        crate::sys::qemu_plugin_outs(CString::new(string.as_ref())?.as_ptr());
    }

    Ok(())
}

/// Parse a boolean argument in the form of `=[on|yes|true|off|no|false]`. returns true
/// if the combination @name=@val parses correctly to a boolean argument, and false
/// otherwise.
///
/// # Arguments
///
/// - `name`: argument name, the part before the equals sign @val: argument value, what’s
/// after the equals sign @ret: output return value
/// - `val`: Argument value, what’s after the equals sign
///
pub fn qemu_plugin_bool_parse<S>(name: S, val: S) -> Result<bool>
where
    S: AsRef<str>,
{
    let mut value = false;
    if unsafe {
        crate::sys::qemu_plugin_bool_parse(
            CString::new(name.as_ref())?.as_ptr(),
            CString::new(val.as_ref())?.as_ptr(),
            &mut value,
        )
    } {
        Ok(value)
    } else {
        Err(Error::InvalidBool {
            name: name.as_ref().to_string(),
            val: val.as_ref().to_string(),
        })
    }
}

/// Return the path to the binary file being executed if running in user mode,
/// or None if running in System mode. Return an error if the path cannot be
/// converted to a string.
pub fn qemu_plugin_path_to_binary() -> Result<Option<PathBuf>> {
    let path_str = unsafe { crate::sys::qemu_plugin_path_to_binary() };
    if path_str.is_null() {
        Ok(None)
    } else {
        let path = unsafe { PathBuf::from(CStr::from_ptr(path_str).to_str()?) };
        unsafe { g_free(path_str as *mut _) };
        Ok(Some(path))
    }
}

/// Return the start of the text segment of the binary file being executed if
/// running in user mode, or None if running in System mode. If not running in
/// system mode, `None` may be interpreted as zero by callers, but the caller
/// must take care to ensure the plugin is not running in a system mode context.
pub fn qemu_plugin_start_code() -> Option<u64> {
    let start = unsafe { crate::sys::qemu_plugin_start_code() };

    if start == 0 {
        None
    } else {
        Some(start)
    }
}

/// Return the end of the text segment of the binary file being executed if
/// running in user mode, or None if running in System mode. If not running in
/// system mode, `None` may be interpreted as zero by callers, but the caller
/// must take care to ensure the plugin is not running in a system mode context.
pub fn qemu_plugin_end_code() -> Option<u64> {
    let end = unsafe { crate::sys::qemu_plugin_end_code() };

    if end == 0 {
        None
    } else {
        Some(end)
    }
}

/// Return the start address for the module of the binary file being executed if
/// running in user mode, or None if running in System mode. If not running in
/// system mode, `None` may be interpreted as zero by callers, but the caller
/// must take care to ensure the plugin is not running in a system mode context.
pub fn qemu_plugin_entry_code() -> Option<u64> {
    let entry = unsafe { crate::sys::qemu_plugin_entry_code() };

    if entry == 0 {
        None
    } else {
        Some(entry)
    }
}

/// Return the maximum number of vCPUs, if running in system mode
pub fn qemu_plugin_n_max_cpus() -> Option<i32> {
    let max_cpus = unsafe { crate::sys::qemu_plugin_n_max_vcpus() };

    if max_cpus == -1 {
        None
    } else {
        Some(max_cpus)
    }
}

/// Return the number of vCPUs, if running in system mode
pub fn qemu_plugin_n_vcpus() -> Option<i32> {
    let vcpus = unsafe { crate::sys::qemu_plugin_n_vcpus() };

    if vcpus == -1 {
        None
    } else {
        Some(vcpus)
    }
}
