//! Scoreboard-related functionality for QEMU plugins

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
use crate::VCPUIndex;
#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
use crate::sys::qemu_plugin_scoreboard;
#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
use std::{marker::PhantomData, mem::MaybeUninit};

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
