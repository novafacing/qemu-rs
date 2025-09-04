//! GLib FFI bindings

#[cfg(not(any(feature = "plugin-api-v0", feature = "plugin-api-v1")))]
use crate::sys::{GArray, GByteArray};
#[cfg(not(windows))]
use std::ffi::c_void;

#[cfg(not(windows))]
unsafe extern "C" {
    /// glib g_free is provided by the QEMU program we are being linked into
    pub(crate) fn g_free(mem: *mut c_void);
}

#[cfg(all(
    not(windows),
    not(any(feature = "plugin-api-v0", feature = "plugin-api-v1"))
))]
unsafe extern "C" {
    /// glib g_byte_array_new is provided by the QEMU program we are being linked into
    pub(crate) fn g_byte_array_new() -> *mut GByteArray;
    /// glib g_byte_array_free is provided by the QEMU program we are being linked into
    pub(crate) fn g_byte_array_free(array: *mut GByteArray, free_segment: bool) -> *mut u8;
    /// glib g_array_free is provided byt he QEMU program we are being linked into
    pub(crate) fn g_array_free(array: *mut GArray, free_segment: bool) -> *mut u8;
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
pub(crate) unsafe fn g_free(mem: *mut c_void) {
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
pub(crate) unsafe fn g_byte_array_new() -> *mut GByteArray {
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
pub(crate) unsafe fn g_byte_array_free(array: *mut GByteArray, free_segment: bool) -> *mut u8 {
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
pub(crate) unsafe fn g_array_free(array: *mut GArray, free_segment: bool) -> *mut u8 {
    unsafe { G_ARRAY_FREE(array as *mut c_void, free_segment) }
}
