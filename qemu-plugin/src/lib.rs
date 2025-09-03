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
//!     plugin::{HasCallbacks, Plugin, Register, init_plugin},
//!     PluginId, TranslationBlock,
//! };
//! use std::sync::Mutex;
//!
//! struct TinyTrace;
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
//! #[ctor]
//! fn init() {
//!     init_plugin(TinyTrace).expect("Failed to initialize plugin");
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
//! edition = "2024"
//!
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! qemu-plugin = "9.2.0-v0"
//! anyhow = "1.0.75"
//! ffi = "0.1.0"
//! ctor = "0.2.6"
//! ```

#![deny(missing_docs)]
#![cfg_attr(feature = "num-traits", feature(generic_const_exprs))]

#[cfg(windows)]
mod win_link_hook;

/// Used by the init_plugin! macro, should not be used directly
pub mod __ctor_export {
    pub use ctor::*;
}

use crate::error::{Error, Result};
#[cfg(all(
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")),
    feature = "num-traits"
))]
use num_traits::{FromBytes, PrimInt};
#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2"
)))]
use qemu_plugin_sys::qemu_plugin_cond;
#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
use qemu_plugin_sys::{
    GArray, GByteArray, qemu_plugin_read_register, qemu_plugin_reg_descriptor,
    qemu_plugin_register, qemu_plugin_scoreboard, qemu_plugin_u64,
};
use qemu_plugin_sys::{
    qemu_plugin_cb_flags, qemu_plugin_hwaddr, qemu_plugin_id_t, qemu_plugin_insn,
    qemu_plugin_mem_rw, qemu_plugin_meminfo_t, qemu_plugin_op, qemu_plugin_simple_cb_t,
    qemu_plugin_tb, qemu_plugin_vcpu_simple_cb_t, qemu_plugin_vcpu_syscall_cb_t,
    qemu_plugin_vcpu_syscall_ret_cb_t, qemu_plugin_vcpu_tb_trans_cb_t,
};
#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2",
    feature = "plugin-api-v3"
)))]
use qemu_plugin_sys::{qemu_plugin_mem_value, qemu_plugin_mem_value_type};
#[cfg(not(feature = "plugin-api-v0"))]
use std::path::PathBuf;
use std::{
    ffi::{CStr, CString, c_uint, c_void},
    marker::PhantomData,
    sync::{Mutex, OnceLock},
};
#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
use std::{
    fmt::{Debug, Formatter},
    mem::MaybeUninit,
};

pub mod error;
pub mod install;
pub mod plugin;
pub mod sys;

#[cfg(not(windows))]
unsafe extern "C" {
    /// glib g_free is provided by the QEMU program we are being linked into
    fn g_free(mem: *mut c_void);
}

#[cfg(all(
    not(windows),
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
unsafe extern "C" {
    /// glib g_byte_array_new is provided by the QEMU program we are being linked into
    fn g_byte_array_new() -> *mut GByteArray;
    /// glib g_byte_array_free is provided by the QEMU program we are being linked into
    fn g_byte_array_free(array: *mut GByteArray, free_segment: bool) -> *mut u8;
    /// glib g_array_free is provided byt he QEMU program we are being linked into
    fn g_array_free(array: *mut GArray, free_segment: bool) -> *mut u8;
}

#[cfg(windows)]
lazy_static::lazy_static! {
    static ref G_FREE : libloading::os::windows::Symbol<unsafe extern "C" fn(*mut c_void)> = {
        let lib =
            libloading::os::windows::Library::open_already_loaded("libglib-2.0-0.dll")
                .expect("libglib-2.0-0.dll should already be loaded");
        // SAFETY
        // "Users of `Library::get` should specify the correct type of the function loaded".
        // We are specifying the correct type of g_free above (`void g_free(void*)`)
        unsafe{lib.get(b"g_free").expect("find g_free")}
    };
}

#[cfg(all(
    windows,
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
lazy_static::lazy_static! {
    static ref G_BYTE_ARRAY_NEW: libloading::os::windows::Symbol<unsafe extern "C" fn() -> *mut GByteArray> = {
        let lib =
            libloading::os::windows::Library::open_already_loaded("libglib-2.0-0.dll")
                .expect("libglib-2.0-0.dll should already be loaded");
        // SAFETY
        // "Users of `Library::get` should specify the correct type of the function loaded".
        // We are specifying the correct type of g_free above (`void g_free(void*)`)
        unsafe{lib.get(b"g_byte_array_new").expect("find g_byte_array_new")}
    };
}

#[cfg(all(
    windows,
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
lazy_static::lazy_static! {
    static ref G_BYTE_ARRAY_FREE: libloading::os::windows::Symbol<unsafe extern "C" fn(*mut c_void, bool) -> *mut u8> = {
        let lib =
            libloading::os::windows::Library::open_already_loaded("libglib-2.0-0.dll")
                .expect("libglib-2.0-0.dll should already be loaded");
        // SAFETY
        // "Users of `Library::get` should specify the correct type of the function loaded".
        // We are specifying the correct type of g_free above (`void g_free(void*)`)
        unsafe{lib.get(b"g_byte_array_free").expect("find g_byte_array_free")}
    };
}

#[cfg(all(
    windows,
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
lazy_static::lazy_static! {
    static ref G_ARRAY_FREE: libloading::os::windows::Symbol<unsafe extern "C" fn(*mut c_void, bool) -> *mut u8> = {
        let lib =
            libloading::os::windows::Library::open_already_loaded("libglib-2.0-0.dll")
                .expect("libglib-2.0-0.dll should already be loaded");
        // SAFETY
        // "Users of `Library::get` should specify the correct type of the function loaded".
        // We are specifying the correct type of g_free above (`void g_free(void*)`)
        unsafe{lib.get(b"g_array_free").expect("find g_array_free")}
    };
}

#[cfg(windows)]
/// Define g_free, because on Windows we cannot delay link it
///
/// # Safety
///
/// `g_free` must *only* be used to deallocate values allocated with `g_malloc`, regardless of
/// its documented guarantees about wrapping the system allocator. QEMU plugin APIs which return
/// such values are documented to do so, and it is safe to call `g_free` on these values
/// provided they are not used afterward.
unsafe fn g_free(mem: *mut c_void) {
    unsafe { G_FREE(mem) }
}

#[cfg(all(
    windows,
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
/// Define g_byte_array_new, because on Windows we cannot delay link it
///
/// # Safety
///
/// `g_byte_array_new` must be used to allocate a new `GByteArray` which can be used to store
/// arbitrary data. The returned `GByteArray` must be freed with `g_byte_array_free` when it is
/// no longer needed.
unsafe fn g_byte_array_new() -> *mut GByteArray {
    unsafe { G_BYTE_ARRAY_NEW() }
}

#[cfg(all(
    windows,
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
/// Define g_byte_array_free, because on Windows we cannot delay link it
///
/// # Safety
///
/// `g_byte_array_free` must be used to free a `GByteArray` allocated with `g_byte_array_new`.
/// The `free_segment` argument should be `true` if the data stored in the `GByteArray` should
/// also be freed. If `false`, the data will not be freed, and the caller is responsible for
/// freeing it with `g_free`.
unsafe fn g_byte_array_free(array: *mut GByteArray, free_segment: bool) -> *mut u8 {
    unsafe { G_BYTE_ARRAY_FREE(array as *mut c_void, free_segment) }
}

#[cfg(all(
    windows,
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
/// Define g_array_free, because on Windows we cannot delay link it
///
/// # Safety
///
/// `g_array_free` must be used to free a `GArray` allocated with `g_array_new`. The `free_segment`
/// argument should be `true` if the data stored in the `GArray` should also be freed. If `false`,
/// the data will not be freed, and the caller is responsible for freeing it with `g_free`.
unsafe fn g_array_free(array: *mut GArray, free_segment: bool) -> *mut u8 {
    unsafe { G_ARRAY_FREE(array as *mut c_void, free_segment) }
}

/// The index of a vCPU
pub type VCPUIndex = c_uint;
#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// u64 member of an entry in a scoreboard, allows access to a specific u64 member in
/// one given entry, located at a specified offset. Inline operations expect this as an
/// entry.
pub type PluginU64 = qemu_plugin_u64;
/// Flags for callbacks
pub type CallbackFlags = qemu_plugin_cb_flags;
/// Memory read/write flags
pub type MemRW = qemu_plugin_mem_rw;
#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2"
)))]
/// A condition for a callback to be run
pub type PluginCondition = qemu_plugin_cond;
/// Plugin operations for inline operations
pub type PluginOp = qemu_plugin_op;
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
///
/// # Example
///
/// ```
/// struct MyPlugin;
///
/// impl qemu_plugin::plugin::Register for MyPlugin {}
///
/// impl qemu_plugin::plugin::HasCallbacks for MyPlugin {
///     fn on_translation_block_translate(
///         &mut self,
///         id: qemu_plugin::PluginId,
///         tb: qemu_plugin::TranslationBlock,
///     ) -> anyhow::Result<()> {
///         for insn in tb.instructions() {
///             println!("{:08x}: {}", insn.vaddr(), insn.disas()?);
///         }   
///         Ok(())
///     }
/// }
/// ```
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
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    pub fn register_execute_callback<F>(&self, cb: F)
    where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        self.register_execute_callback_flags(cb, CallbackFlags::QEMU_PLUGIN_CB_NO_REGS);
    }

    /// Register a callback to be run on execution of this translation block
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    pub fn register_execute_callback_flags<F>(&self, cb: F, flags: CallbackFlags)
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
                flags,
                userdata,
            )
        };
    }

    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    )))]
    /// Register a callback to be conditionally run on execution of this translation
    /// block
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `cond`: The condition for the callback to be run
    /// - `entry` The entry to increment the scoreboard for
    /// - `immediate`: The immediate value to use for the callback
    pub fn register_conditional_execute_callback<F>(
        &self,
        cb: F,
        cond: PluginCondition,
        entry: PluginU64,
        immediate: u64,
    ) where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        self.register_conditional_execute_callback_flags(
            cb,
            CallbackFlags::QEMU_PLUGIN_CB_NO_REGS,
            cond,
            entry,
            immediate,
        )
    }

    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    )))]
    /// Register a callback to be conditionally run on execution of this translation
    /// block
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `flags`: The flags for the callback
    /// - `cond`: The condition for the callback to be run
    /// - `entry`: The entry to increment the scoreboard for
    /// - `immediate`: The immediate value to use for the callback
    pub fn register_conditional_execute_callback_flags<F>(
        &self,
        cb: F,
        flags: CallbackFlags,
        cond: PluginCondition,
        entry: PluginU64,
        immediate: u64,
    ) where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        let callback = Box::new(cb);
        let callback_box = Box::new(callback);
        let userdata = Box::into_raw(callback_box) as *mut c_void;

        unsafe {
            crate::sys::qemu_plugin_register_vcpu_tb_exec_cond_cb(
                self.translation_block as *mut qemu_plugin_tb,
                Some(handle_qemu_plugin_register_vcpu_tb_exec_cb::<F>),
                flags,
                cond,
                entry,
                immediate,
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
///
/// # Example
///
/// ```
/// struct MyPlugin;
///
/// impl qemu_plugin::plugin::Register for MyPlugin {}
///
/// impl qemu_plugin::plugin::HasCallbacks for MyPlugin {
///     fn on_translation_block_translate(
///         &mut self,
///         id: qemu_plugin::PluginId,
///         tb: qemu_plugin::TranslationBlock,
///     ) -> anyhow::Result<()> {
///         for insn in tb.instructions() {
///             let vaddr = insn.vaddr();
///             let disas = insn.disas()?;
///             // Register a callback to be run on execution of this instruction
///             insn.register_execute_callback(move |vcpu_index| {
///                 println!("{vcpu_index}@{vaddr:#x}: {disas}");
///             });
///         }
///         Ok(())
///     }
/// }
/// ```
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
    #[cfg(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    ))]
    /// Returns the data for this instruction. This method may only be called inside the
    /// callback in which the instruction is obtained, but the resulting data is owned.
    pub fn data(&self) -> Vec<u8> {
        let size = self.size();
        let mut data = Vec::with_capacity(size);

        // NOTE: The name of this API doesn't change, but its parameters and return value *do*
        let insn_data =
            unsafe { crate::sys::qemu_plugin_insn_data(self.instruction as *mut qemu_plugin_insn) }
                as *mut u8;

        unsafe {
            data.set_len(size);
            std::ptr::copy_nonoverlapping(insn_data, data.as_mut_ptr(), size);
        }

        data
    }

    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    )))]
    /// Reads the data for this instruction returning number of bytes read. This method may only be
    /// called inside the callback in which the instruction is obtained.
    pub fn read_data(&self, data: &mut [u8]) -> usize {
        // NOTE: The name of this API doesn't change, but its parameters and return value *do*
        unsafe {
            crate::sys::qemu_plugin_insn_data(
                self.instruction as *mut qemu_plugin_insn,
                data.as_mut_ptr() as *mut _,
                data.len(),
            )
        }
    }

    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    )))]
    /// Returns the data for this instruction. This method may only be called inside the
    /// callback in which the instruction is obtained, but the resulting data is owned.
    pub fn data(&self) -> Vec<u8> {
        let size = self.size();
        let mut data = vec![0; size];

        let size = self.read_data(&mut data);

        data.truncate(size);

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

    #[cfg(not(feature = "plugin-api-v0"))]
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

    /// Register a callback to be run on execution of this instruction with no
    /// capability to inspect registers
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    pub fn register_execute_callback<F>(&self, cb: F)
    where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        self.register_execute_callback_flags(cb, CallbackFlags::QEMU_PLUGIN_CB_NO_REGS)
    }

    /// Register a callback to be run on execution of this instruction with a choice of
    /// capability whether to inspect or modify registers or not
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `flags`: The flags for the callback specifying whether the callback needs
    ///   permission to read or write registers
    pub fn register_execute_callback_flags<F>(&self, cb: F, flags: CallbackFlags)
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
                flags,
                userdata,
            )
        };
    }

    /// Register a callback to be conditionally run on execution of this instruction
    /// with no capability to inspect registers
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `cond`: The condition for the callback to be run
    /// - `entry`: The entry to increment the scoreboard for
    /// - `immediate`: The immediate value to use for the callback
    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    )))]
    pub fn register_conditional_execute_callback<F>(
        &self,
        cb: F,
        cond: PluginCondition,
        entry: PluginU64,
        immediate: u64,
    ) where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        self.register_conditional_execute_callback_flags(
            cb,
            CallbackFlags::QEMU_PLUGIN_CB_NO_REGS,
            cond,
            entry,
            immediate,
        )
    }

    /// Register a callback to be conditionally run on execution of this instruction
    /// with a choice of capability whether to inspect or modify registers or not
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `flags`: The flags for the callback specifying whether the callback needs
    ///   permission to read or write registers
    /// - `cond`: The condition for the callback to be run
    /// - `entry`: The entry to increment the scoreboard for
    /// - `immediate`: The immediate value to use for the callback
    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2"
    )))]
    pub fn register_conditional_execute_callback_flags<F>(
        &self,
        cb: F,
        flags: CallbackFlags,
        cond: PluginCondition,
        entry: PluginU64,
        immediate: u64,
    ) where
        F: FnMut(VCPUIndex) + Send + Sync + 'static,
    {
        let callback = Box::new(cb);
        let callback_box = Box::new(callback);
        let userdata = Box::into_raw(callback_box) as *mut c_void;

        unsafe {
            crate::sys::qemu_plugin_register_vcpu_insn_exec_cond_cb(
                self.instruction as *mut qemu_plugin_insn,
                Some(handle_qemu_plugin_register_vcpu_insn_exec_cb::<F>),
                flags,
                cond,
                entry,
                immediate,
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
        F: for<'b> FnMut(VCPUIndex, MemoryInfo<'b>, u64) + Send + Sync + 'static,
    {
        self.register_memory_access_callback_flags(cb, rw, CallbackFlags::QEMU_PLUGIN_CB_NO_REGS)
    }

    /// Register a callback to be run on memory access of this instruction
    ///
    /// # Arguments
    ///
    /// - `cb`: The callback to be run
    /// - `rw`: The type of memory access to trigger the callback on
    pub fn register_memory_access_callback_flags<F>(&self, cb: F, rw: MemRW, flags: CallbackFlags)
    where
        F: for<'b> FnMut(VCPUIndex, MemoryInfo<'b>, u64) + Send + Sync + 'static,
    {
        let callback = Box::new(cb);
        let callback_box = Box::new(callback);
        let userdata = Box::into_raw(callback_box) as *mut c_void;

        unsafe {
            crate::sys::qemu_plugin_register_vcpu_mem_cb(
                self.instruction as *mut qemu_plugin_insn,
                Some(handle_qemu_plugin_register_vcpu_mem_cb::<F>),
                flags,
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
    pub fn hwaddr(&'a self, vaddr: u64) -> Option<HwAddr<'a>> {
        let hwaddr = unsafe { crate::sys::qemu_plugin_get_hwaddr(self.memory_info, vaddr) };
        if hwaddr.is_null() {
            None
        } else {
            Some(HwAddr::from(hwaddr))
        }
    }

    /// Return last value loaded/stored
    #[cfg(not(any(
        feature = "plugin-api-v0",
        feature = "plugin-api-v1",
        feature = "plugin-api-v2",
        feature = "plugin-api-v3"
    )))]
    pub fn value(&self) -> MemValue {
        let qemu_mem_value = unsafe { crate::sys::qemu_plugin_mem_get_value(self.memory_info) };
        MemValue::from(qemu_mem_value)
    }
}

#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2",
    feature = "plugin-api-v3"
)))]
#[derive(Clone)]
/// Memory value loaded/stored (in memory callback)
///
/// Wrapper structure for a `qemu_plugin_mem_value`
pub enum MemValue {
    /// 8-bit value
    U8(u8),
    /// 16-bit value
    U16(u16),
    /// 32-bit value
    U32(u32),
    /// 64-bit value
    U64(u64),
    /// 128-bit value
    U128(u128),
}

#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2",
    feature = "plugin-api-v3"
)))]
impl From<qemu_plugin_mem_value> for MemValue {
    fn from(value: qemu_plugin_mem_value) -> Self {
        unsafe {
            match value.type_ {
                qemu_plugin_mem_value_type::QEMU_PLUGIN_MEM_VALUE_U8 => Self::U8(value.data.u8_),
                qemu_plugin_mem_value_type::QEMU_PLUGIN_MEM_VALUE_U16 => Self::U16(value.data.u16_),
                qemu_plugin_mem_value_type::QEMU_PLUGIN_MEM_VALUE_U32 => Self::U32(value.data.u32_),
                qemu_plugin_mem_value_type::QEMU_PLUGIN_MEM_VALUE_U64 => Self::U64(value.data.u64_),
                qemu_plugin_mem_value_type::QEMU_PLUGIN_MEM_VALUE_U128 => {
                    let high = value.data.u128_.high as u128;
                    let low = value.data.u128_.low as u128;
                    Self::U128(high << 64 | low)
                }
            }
        }
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
#[derive(Clone)]
/// Wrapper structure for a `qemu_plugin_register_descriptor`
///
/// # Safety
///
/// This structure is safe to use as long as the pointer is valid. The pointer is
/// always opaque, and therefore may not be dereferenced.
pub struct RegisterDescriptor<'a> {
    /// Opaque handle to the register for retrieving the value with
    /// qemu_plugin_read_register
    handle: usize,
    /// The register name
    pub name: String,
    /// Optional feature descriptor
    pub feature: Option<String>,
    marker: PhantomData<&'a ()>,
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
impl<'a> From<qemu_plugin_reg_descriptor> for RegisterDescriptor<'a> {
    fn from(descriptor: qemu_plugin_reg_descriptor) -> Self {
        let name = unsafe { CStr::from_ptr(descriptor.name) }
            .to_str()
            .expect("Register name is not valid UTF-8")
            .to_string();

        let feature = if descriptor.feature.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(descriptor.feature) }
                    .to_str()
                    .expect("Register feature is not valid UTF-8")
                    .to_string(),
            )
        };

        Self {
            handle: descriptor.handle as usize,
            name,
            feature,
            marker: PhantomData,
        }
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
impl<'a> Debug for RegisterDescriptor<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterDescriptor")
            .field("name", &self.name)
            .field("feature", &self.feature)
            .finish()
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
impl<'a> RegisterDescriptor<'a> {
    /// Read a register value
    ///
    /// This must only be called in a callback which has been registered with
    /// `CallbackFlags::QEMU_PLUGIN_CB_R_REGS` or
    /// `CallbackFlags::QEMU_PLUGIN_CB_RW_REGS`, otherwise it will fail.
    pub fn read(&self) -> Result<Vec<u8>> {
        let byte_array = unsafe { g_byte_array_new() };

        let result = unsafe {
            qemu_plugin_read_register(self.handle as *mut qemu_plugin_register, byte_array)
        };

        if result == -1 {
            return Err(Error::RegisterReadError {
                name: self.name.clone(),
            });
        }

        let mut data = Vec::new();
        data.extend_from_slice(unsafe {
            std::slice::from_raw_parts((*byte_array).data, (*byte_array).len as usize)
        });

        assert_eq!(
            unsafe { g_byte_array_free(byte_array, true) },
            std::ptr::null_mut(),
            "g_byte_array_free must return NULL"
        );

        Ok(data)
    }

    #[cfg(feature = "num-traits")]
    /// Read a register value into a numeric type in big-endian byte order
    ///
    /// This must only be called in a callback which has been registered with
    /// `CallbackFlags::QEMU_PLUGIN_CB_R_REGS` or
    /// `CallbackFlags::QEMU_PLUGIN_CB_RW_REGS`.
    pub fn read_be<T>(&self) -> Result<T>
    where
        T: PrimInt + FromBytes + Sized,
        T: FromBytes<Bytes = [u8; std::mem::size_of::<T>()]>,
    {
        let data = self.read()?;
        let mut bytes = [0; std::mem::size_of::<T>()];
        bytes.copy_from_slice(&data);
        Ok(T::from_be_bytes(&bytes))
    }

    #[cfg(feature = "num-traits")]
    /// Read a register value into a numeric type in little-endian byte order
    ///
    /// This must only be called in a callback which has been registered with
    /// `CallbackFlags::QEMU_PLUGIN_CB_R_REGS` or
    /// `CallbackFlags::QEMU_PLUGIN_CB_RW_REGS`.
    pub fn read_le<T>(&self) -> Result<T>
    where
        T: PrimInt + FromBytes + Sized,
        T: FromBytes<Bytes = [u8; std::mem::size_of::<T>()]>,
    {
        let data = self.read()?;
        let mut bytes = [0; std::mem::size_of::<T>()];
        bytes.copy_from_slice(&data);
        Ok(T::from_le_bytes(&bytes))
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

    #[cfg(not(feature = "plugin-api-v0"))]
    /// Returns the physical address for the memory operation
    pub fn hwaddr(&self) -> u64 {
        unsafe { crate::sys::qemu_plugin_hwaddr_phys_addr(self.hwaddr as *mut qemu_plugin_hwaddr) }
    }

    #[cfg(not(feature = "plugin-api-v0"))]
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

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// A wrapper structure for a `qemu_plugin_scoreboard *`. This is a way of having one
/// entry per VCPU, the count of which is managed automatically by QEMU. Keep in mind
/// that additional entries *and* existing entries will be allocated and reallocated by
/// *qemu*, not by the plugin, so every use of a `T` should include a check for whether
/// it is initialized.
pub struct Scoreboard<'a, T>
where
    T: Sized,
{
    handle: usize,
    marker: PhantomData<&'a T>,
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
impl<'a, T> Scoreboard<'a, T> {
    /// Allocate a new scoreboard object. This must be freed by calling
    /// `qemu_plugin_scoreboard_free` (or by being dropped).
    pub fn new() -> Self {
        let handle =
            unsafe { crate::sys::qemu_plugin_scoreboard_new(std::mem::size_of::<T>()) as usize };

        Self {
            handle,
            marker: PhantomData,
        }
    }

    /// Returns a reference to entry of a scoreboard matching a given vcpu index. This address
    /// is only valid until the next call to `get` or `set`.
    pub fn find<'b>(&mut self, vcpu_index: VCPUIndex) -> &'b mut MaybeUninit<T> {
        unsafe {
            &mut *(crate::sys::qemu_plugin_scoreboard_find(
                self.handle as *mut qemu_plugin_scoreboard,
                vcpu_index,
            ) as *mut MaybeUninit<T>)
        }
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
impl<'a, T> Default for Scoreboard<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
impl<'a, T> Drop for Scoreboard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            crate::sys::qemu_plugin_scoreboard_free(self.handle as *mut qemu_plugin_scoreboard)
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
    if let Some(callback) = UNINSTALL_CALLBACK.get()
        && let Ok(mut callback) = callback.lock()
        && let Some(callback) = callback.take()
    {
        callback(id);
    }
    // NOTE: An error here is ignored, and exceedingly fatal
}

/// Handle the invocation of the reset callback by calling the stored
/// callback closure, if one exists.
extern "C" fn handle_qemu_plugin_reset_callback(id: qemu_plugin_id_t) {
    if let Some(callback) = UNINSTALL_CALLBACK.get()
        && let Ok(mut callback) = callback.lock()
        && let Some(callback) = callback.take()
    {
        callback(id);
    }
    // NOTE: An error here is ignored, and exceedingly fatal
}

/// Uninstall a plugin.
///
/// # Arguments
///
/// - `id`: The plugin ID
/// - `cb`: A callback function that will be called when the plugin has been
///   uninstalled.
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
pub fn qemu_plugin_register_vcpu_tb_exec_cb<F>(tb: TranslationBlock, cb: F, flags: CallbackFlags)
where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    tb.register_execute_callback_flags(cb, flags);
}

#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2"
)))]
/// Register a callback to be conditionally called when a translation block is executed.
///
/// # Arguments
///
/// - `tb`: The translated block to register the execution callback for
/// - `cb`: The callback to be called when the block `tb` is executed
/// - `cond`: The condition to be met for the callback to be called
/// - `entry`: The entry to be passed to the callback
/// - `immediate`: The immediate value to be passed to the callback
///
/// # Safety
///
/// This function is safe when the pointer `tb` is a valid pointer to a `qemu_plugin_tb`
/// structure, which is always opaque.
pub fn qemu_plugin_register_vcpu_tb_exec_cond_cb<F>(
    tb: TranslationBlock,
    cb: F,
    flags: CallbackFlags,
    cond: PluginCondition,
    entry: PluginU64,
    immediate: u64,
) where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    tb.register_conditional_execute_callback_flags(cb, flags, cond, entry, immediate);
}

#[cfg(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register an inline callback to be called when a translation block is executed.
///
/// # Arguments
///
/// - `tb`: The translated block to register the execution callback for
/// - `op`: The operation to be performed
/// - `ptr`: The pointer to the data to be passed to the operation
/// - `imm`: The immediate value to be passed to the operation
pub fn qemu_plugin_register_vcpu_tb_exec_inline(
    tb: TranslationBlock,
    op: PluginOp,
    ptr: *mut c_void,
    imm: u64,
) {
    unsafe {
        crate::sys::qemu_plugin_register_vcpu_tb_exec_inline(
            tb.translation_block as *mut qemu_plugin_tb,
            op,
            ptr,
            imm,
        );
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register an inline callback to be called when a translation block is executed.
///
/// # Arguments
///
/// - `tb`: The translated block to register the execution callback for
/// - `op`: The operation to be performed
/// - `entry`: The entry to be passed to the operation
/// - `imm`: The immediate value to be passed to the operation
pub fn qemu_plugin_register_vcpu_tb_exec_inline_per_vcpu(
    tb: TranslationBlock,
    op: PluginOp,
    entry: PluginU64,
    imm: u64,
) {
    unsafe {
        crate::sys::qemu_plugin_register_vcpu_tb_exec_inline_per_vcpu(
            tb.translation_block as *mut qemu_plugin_tb,
            op,
            entry,
            imm,
        );
    }
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
pub fn qemu_plugin_register_vcpu_insn_exec_cb<F>(insn: Instruction, cb: F, flags: CallbackFlags)
where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    insn.register_execute_callback_flags(cb, flags);
}

#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2"
)))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register a callback to be conditionally called when an instruction is executed.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `cb`: The callback to be called
/// - `cond`: The condition to be met for the callback to be called
/// - `entry`: The entry to be passed to the callback
/// - `immediate`: The immediate value to be passed to the callback
pub fn qemu_plugin_register_vcpu_insn_exec_cond_cb<F>(
    insn: Instruction,
    cb: F,
    flags: CallbackFlags,
    cond: PluginCondition,
    entry: PluginU64,
    immediate: u64,
) where
    F: FnMut(VCPUIndex) + Send + Sync + 'static,
{
    insn.register_conditional_execute_callback_flags(cb, flags, cond, entry, immediate);
}

#[cfg(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register an inline callback to be called when an instruction is executed.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `op`: The operation to be performed
/// - `ptr`: The pointer to the data to be passed to the operation
/// - `imm`: The immediate value to be passed to the operation
pub fn qemu_plugin_register_vcpu_insn_exec_inline(
    insn: Instruction,
    op: PluginOp,
    ptr: *mut c_void,
    imm: u64,
) {
    unsafe {
        crate::sys::qemu_plugin_register_vcpu_insn_exec_inline(
            insn.instruction as *mut qemu_plugin_insn,
            op,
            ptr,
            imm,
        );
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register an inline callback to be called when an instruction is executed.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `op`: The operation to be performed
/// - `entry`: The entry to be passed to the operation
/// - `imm`: The immediate value to be passed to the operation
pub fn qemu_plugin_register_vcpu_insn_exec_inline_per_vcpu(
    insn: Instruction,
    op: PluginOp,
    entry: PluginU64,
    imm: u64,
) {
    unsafe {
        crate::sys::qemu_plugin_register_vcpu_insn_exec_inline_per_vcpu(
            insn.instruction as *mut qemu_plugin_insn,
            op,
            entry,
            imm,
        );
    }
}

extern "C" fn handle_qemu_plugin_register_vcpu_mem_cb<F>(
    vcpu_index: VCPUIndex,
    meminfo: qemu_plugin_meminfo_t,
    vaddr: u64,
    userdata: *mut c_void,
) where
    F: for<'a> FnMut(VCPUIndex, MemoryInfo<'a>, u64) + Send + Sync + 'static,
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
pub fn qemu_plugin_register_vcpu_mem_cb<F>(
    insn: Instruction,
    cb: F,
    flags: CallbackFlags,
    rw: MemRW,
) where
    F: for<'a> FnMut(VCPUIndex, MemoryInfo<'a>, u64) + Send + Sync + 'static,
{
    insn.register_memory_access_callback_flags(cb, rw, flags);
}

#[cfg(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register an inline callback for every memory transaction of a particular instruction.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `rw`: Whether the callback should be called for reads, writes, or both
/// - `op`: The operation to be performed
/// - `ptr`: The pointer to the data to be passed to the operation
/// - `imm`: The immediate value to be passed to the operation
pub fn qemu_plugin_register_vcpu_mem_inline(
    insn: Instruction,
    rw: MemRW,
    op: PluginOp,
    ptr: *mut c_void,
    imm: u64,
) {
    unsafe {
        crate::sys::qemu_plugin_register_vcpu_mem_inline(
            insn.instruction as *mut qemu_plugin_insn,
            rw,
            op,
            ptr,
            imm,
        );
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Register an inline callback for every memory transaction of a particular instruction.
///
/// # Arguments
///
/// - `insn`: The instruction handle to register the callback for
/// - `rw`: Whether the callback should be called for reads, writes, or both
/// - `op`: The operation to be performed
/// - `entry`: The entry to be passed to the operation
/// - `imm`: The immediate value to be passed to the operation
pub fn qemu_plugin_register_vcpu_mem_inline_per_vcpu(
    insn: Instruction,
    rw: MemRW,
    op: PluginOp,
    entry: PluginU64,
    imm: u64,
) {
    unsafe {
        crate::sys::qemu_plugin_register_vcpu_mem_inline_per_vcpu(
            insn.instruction as *mut qemu_plugin_insn,
            rw,
            op,
            entry,
            imm,
        );
    }
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

/// Register a callback to run after QEMU flushes all translation blocks. This is
/// roughly equivalent to a TLB flush, where all instruction caches are invalidated.
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

#[cfg(feature = "plugin-api-v0")]
/// Parse a boolean argument in the form of `=[on|yes|true|off|no|false]`.
/// returns true if the combination @name=@val parses correctly to a boolean
/// argument, and false otherwise. Note that in plugin API v0, this function is
/// not available, so we implement it ourselves. This may ostensibly lead to
/// different behavior.
///
/// # Arguments
///
/// - `name`: argument name, the part before the equals sign @val: argument
///   value, what’s after the equals sign @ret: output return value
/// - `val`: Argument value, what’s after the equals sign
///
pub fn qemu_plugin_bool_parse<S>(name: S, val: S) -> Result<bool>
where
    S: AsRef<str>,
{
    // We can't call sys::qemu_plugin_bool_parse directly because it doesn't exist in plugin-api-v0
    match val.as_ref() {
        "on" | "yes" | "true" => Ok(true),
        "off" | "no" | "false" => Ok(false),
        _ => Err(Error::InvalidBool {
            name: name.as_ref().to_string(),
            val: val.as_ref().to_string(),
        }),
    }
}

#[cfg(not(feature = "plugin-api-v0"))]
/// Parse a boolean argument in the form of `=[on|yes|true|off|no|false]`. returns true
/// if the combination @name=@val parses correctly to a boolean argument, and false
/// otherwise.
///
/// # Arguments
///
/// - `name`: argument name, the part before the equals sign @val: argument value, what’s
///   after the equals sign @ret: output return value
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

#[cfg(not(feature = "plugin-api-v0"))]
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

#[cfg(not(feature = "plugin-api-v0"))]
/// Return the start of the text segment of the binary file being executed if
/// running in user mode, or None if running in System mode. If not running in
/// system mode, `None` may be interpreted as zero by callers, but the caller
/// must take care to ensure the plugin is not running in a system mode context.
pub fn qemu_plugin_start_code() -> Option<u64> {
    let start = unsafe { crate::sys::qemu_plugin_start_code() };

    if start == 0 { None } else { Some(start) }
}

#[cfg(not(feature = "plugin-api-v0"))]
/// Return the end of the text segment of the binary file being executed if
/// running in user mode, or None if running in System mode. If not running in
/// system mode, `None` may be interpreted as zero by callers, but the caller
/// must take care to ensure the plugin is not running in a system mode context.
pub fn qemu_plugin_end_code() -> Option<u64> {
    let end = unsafe { crate::sys::qemu_plugin_end_code() };

    if end == 0 { None } else { Some(end) }
}

#[cfg(not(feature = "plugin-api-v0"))]
/// Return the start address for the module of the binary file being executed if
/// running in user mode, or None if running in System mode. If not running in
/// system mode, `None` may be interpreted as zero by callers, but the caller
/// must take care to ensure the plugin is not running in a system mode context.
pub fn qemu_plugin_entry_code() -> Option<u64> {
    let entry = unsafe { crate::sys::qemu_plugin_entry_code() };

    if entry == 0 { None } else { Some(entry) }
}

#[cfg(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))]
/// Return the number of vCPUs, if running in system mode
pub fn qemu_plugin_n_vcpus() -> Option<i32> {
    let vcpus = unsafe { crate::sys::qemu_plugin_n_vcpus() };

    if vcpus == -1 { None } else { Some(vcpus) }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// Return the number of vCPUs, if running in system mode
pub fn qemu_plugin_num_vcpus() -> Option<i32> {
    let vcpus = unsafe { crate::sys::qemu_plugin_num_vcpus() };

    if vcpus == -1 { None } else { Some(vcpus) }
}

#[cfg(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))]
/// Return the maximum number of vCPUs, if running in system mode
pub fn qemu_plugin_n_max_vcpus() -> Option<i32> {
    let max_cpus = unsafe { crate::sys::qemu_plugin_n_max_vcpus() };

    if max_cpus == -1 { None } else { Some(max_cpus) }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// Returns a potentially empty list of registers. This should be used from a
/// qemu_plugin_register_vcpu_init_cb callback after the vcpu has been initialized.
pub fn qemu_plugin_get_registers<'a>() -> Result<Vec<RegisterDescriptor<'a>>> {
    use std::slice::from_raw_parts;

    let array = unsafe { crate::sys::qemu_plugin_get_registers() };

    let registers = unsafe {
        from_raw_parts(
            (*array).data as *mut qemu_plugin_reg_descriptor,
            (*array).len as usize,
        )
    }
    .iter()
    .map(|desc| RegisterDescriptor::from(*desc))
    .collect::<Vec<_>>();

    // Function notes say caller frees the array but not the strings in each entry
    assert_eq!(
        unsafe { g_array_free(array, true) },
        std::ptr::null_mut(),
        "g_array_free return value must be NULL"
    );

    Ok(registers)
}

#[cfg(not(any(
    feature = "plugin-api-v0",
    feature = "plugin-api-v1",
    feature = "plugin-api-v2",
    feature = "plugin-api-v3"
)))]
/// Returns the contents of virtual memory
///
/// # Arguments
///
/// - `addr`: The virtual address to read from
/// - `len`: The number of bytes to read
pub fn qemu_plugin_read_memory_vaddr(addr: u64, len: usize) -> Result<Vec<u8>> {
    use std::slice::from_raw_parts;

    let data = unsafe { g_byte_array_new() };
    if !unsafe { crate::sys::qemu_plugin_read_memory_vaddr(addr, data, len) } {
        Err(Error::VaddrReadError { addr, len })
    } else {
        Ok(unsafe { from_raw_parts((*data).data, (*data).len as usize) }.to_vec())
    }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// Add a value to a `PluginU64` for a given VCPU
pub fn qemu_plugin_u64_add(entry: PluginU64, vcpu_index: VCPUIndex, added: u64) -> Result<()> {
    unsafe { crate::sys::qemu_plugin_u64_add(entry, vcpu_index, added) };
    Ok(())
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// Get the value of a `PluginU64` for a given VCPU
pub fn qemu_plugin_u64_get(entry: PluginU64, vcpu_index: VCPUIndex) -> u64 {
    unsafe { crate::sys::qemu_plugin_u64_get(entry, vcpu_index) }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// Set the value of a `PluginU64` for a given VCPU
pub fn qemu_plugin_u64_set(entry: PluginU64, vcpu_index: VCPUIndex, value: u64) {
    unsafe { crate::sys::qemu_plugin_u64_set(entry, vcpu_index, value) }
}

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
/// Get the sum of all VCPU entries in a scoreboard
pub fn qemu_plugin_scoreboard_sum(entry: PluginU64) -> u64 {
    unsafe { crate::sys::qemu_plugin_u64_sum(entry) }
}
