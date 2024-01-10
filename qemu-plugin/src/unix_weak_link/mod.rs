//! Weak linkage for unix hosts allowing this library to be built as an rlib, leaving QEMU
//! symbols unresolved

use std::ptr::{null, null_mut};

use qemu_plugin_sys::*;

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_uninstall(_: qemu_plugin_id_t, _: qemu_plugin_simple_cb_t) {}
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_reset(_: qemu_plugin_id_t, _: qemu_plugin_simple_cb_t) {}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_init_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_simple_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_exit_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_simple_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_idle_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_simple_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_resume_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_simple_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_tb_trans_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_tb_trans_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_tb_exec_cb(
    _: *mut qemu_plugin_tb,
    _: qemu_plugin_vcpu_udata_cb_t,
    _: qemu_plugin_cb_flags,
    _: *mut ::std::os::raw::c_void,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_tb_exec_inline(
    _: *mut qemu_plugin_tb,
    _: qemu_plugin_op,
    _: *mut ::std::os::raw::c_void,
    _: u64,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_insn_exec_cb(
    _: *mut qemu_plugin_insn,
    _: qemu_plugin_vcpu_udata_cb_t,
    _: qemu_plugin_cb_flags,
    _: *mut ::std::os::raw::c_void,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_insn_exec_inline(
    _: *mut qemu_plugin_insn,
    _: qemu_plugin_op,
    _: *mut ::std::os::raw::c_void,
    _: u64,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_tb_n_insns(_: *const qemu_plugin_tb) -> usize {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_tb_vaddr(_: *const qemu_plugin_tb) -> u64 {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_tb_get_insn(
    _: *const qemu_plugin_tb,
    _: usize,
) -> *mut qemu_plugin_insn {
    null_mut()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_insn_data(
    _: *const qemu_plugin_insn,
) -> *const ::std::os::raw::c_void {
    null()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_insn_size(_: *const qemu_plugin_insn) -> usize {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_insn_vaddr(_: *const qemu_plugin_insn) -> u64 {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_insn_haddr(
    _: *const qemu_plugin_insn,
) -> *mut ::std::os::raw::c_void {
    null_mut()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_mem_size_shift(_: qemu_plugin_meminfo_t) -> ::std::os::raw::c_uint {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_mem_is_sign_extended(_: qemu_plugin_meminfo_t) -> bool {
    false
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_mem_is_big_endian(_: qemu_plugin_meminfo_t) -> bool {
    false
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_mem_is_store(_: qemu_plugin_meminfo_t) -> bool {
    false
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_get_hwaddr(
    _: qemu_plugin_meminfo_t,
    _: u64,
) -> *mut qemu_plugin_hwaddr {
    null_mut()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_hwaddr_is_io(_: *const qemu_plugin_hwaddr) -> bool {
    false
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_hwaddr_phys_addr(_: *const qemu_plugin_hwaddr) -> u64 {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_hwaddr_device_name(
    _: *const qemu_plugin_hwaddr,
) -> *const ::std::os::raw::c_char {
    null()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_mem_cb(
    _: *mut qemu_plugin_insn,
    _: qemu_plugin_vcpu_mem_cb_t,
    _: qemu_plugin_cb_flags,
    _: qemu_plugin_mem_rw,
    _: *mut ::std::os::raw::c_void,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_mem_inline(
    _: *mut qemu_plugin_insn,
    _: qemu_plugin_mem_rw,
    _: qemu_plugin_op,
    _: *mut ::std::os::raw::c_void,
    _: u64,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_syscall_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_syscall_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_vcpu_syscall_ret_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_vcpu_syscall_ret_cb_t,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_insn_disas(
    _: *const qemu_plugin_insn,
) -> *mut ::std::os::raw::c_char {
    null_mut()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_insn_symbol(
    _: *const qemu_plugin_insn,
) -> *const ::std::os::raw::c_char {
    null()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_vcpu_for_each(_: qemu_plugin_id_t, _: qemu_plugin_vcpu_simple_cb_t) {}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_flush_cb(_: qemu_plugin_id_t, _: qemu_plugin_simple_cb_t) {}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_register_atexit_cb(
    _: qemu_plugin_id_t,
    _: qemu_plugin_udata_cb_t,
    _: *mut ::std::os::raw::c_void,
) {
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_n_vcpus() -> ::std::os::raw::c_int {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_n_max_vcpus() -> ::std::os::raw::c_int {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_outs(_: *const ::std::os::raw::c_char) {}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_bool_parse(
    _: *const ::std::os::raw::c_char,
    _: *const ::std::os::raw::c_char,
    _: *mut bool,
) -> bool {
    false
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_path_to_binary() -> *const ::std::os::raw::c_char {
    null()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_start_code() -> u64 {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_end_code() -> u64 {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn qemu_plugin_entry_code() -> u64 {
    0
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn g_free(_: *mut ::std::ffi::c_void) {}
