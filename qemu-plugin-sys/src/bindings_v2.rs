/* automatically generated by rust-bindgen 0.70.1 */

pub const QEMU_PLUGIN_VERSION: u32 = 2;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct GArray {
    pub data: *mut ::std::os::raw::c_char,
    pub len: ::std::os::raw::c_uint,
}
impl Default for GArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct GByteArray {
    pub data: *mut ::std::os::raw::c_uchar,
    pub len: ::std::os::raw::c_uint,
}
impl Default for GByteArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " typedef qemu_plugin_id_t - Unique plugin ID"]
pub type qemu_plugin_id_t = u64;
#[doc = " struct qemu_info_t - system information for plugins\n\n This structure provides for some limited information about the\n system to allow the plugin to make decisions on how to proceed. For\n example it might only be suitable for running on some guest\n architectures or when under full system emulation."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qemu_info_t {
    #[doc = " @target_name: string describing architecture"]
    pub target_name: *const ::std::os::raw::c_char,
    pub version: qemu_info_t__bindgen_ty_1,
    #[doc = " @system_emulation: is this a full system emulation?"]
    pub system_emulation: bool,
    pub __bindgen_anon_1: qemu_info_t__bindgen_ty_2,
}
#[doc = " @version: minimum and current plugin API level"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct qemu_info_t__bindgen_ty_1 {
    pub min: ::std::os::raw::c_int,
    pub cur: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union qemu_info_t__bindgen_ty_2 {
    pub system: qemu_info_t__bindgen_ty_2__bindgen_ty_1,
}
#[doc = " @system: information relevant to system emulation"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct qemu_info_t__bindgen_ty_2__bindgen_ty_1 {
    #[doc = " @system.smp_vcpus: initial number of vCPUs"]
    pub smp_vcpus: ::std::os::raw::c_int,
    #[doc = " @system.max_vcpus: maximum possible number of vCPUs"]
    pub max_vcpus: ::std::os::raw::c_int,
}
impl Default for qemu_info_t__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for qemu_info_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " typedef qemu_plugin_simple_cb_t - simple callback\n @id: the unique qemu_plugin_id_t\n\n This callback passes no information aside from the unique @id."]
pub type qemu_plugin_simple_cb_t =
    ::std::option::Option<unsafe extern "C" fn(id: qemu_plugin_id_t)>;
#[doc = " typedef qemu_plugin_udata_cb_t - callback with user data\n @id: the unique qemu_plugin_id_t\n @userdata: a pointer to some user data supplied when the callback\n was registered."]
pub type qemu_plugin_udata_cb_t = ::std::option::Option<
    unsafe extern "C" fn(id: qemu_plugin_id_t, userdata: *mut ::std::os::raw::c_void),
>;
#[doc = " typedef qemu_plugin_vcpu_simple_cb_t - vcpu callback\n @id: the unique qemu_plugin_id_t\n @vcpu_index: the current vcpu context"]
pub type qemu_plugin_vcpu_simple_cb_t = ::std::option::Option<
    unsafe extern "C" fn(id: qemu_plugin_id_t, vcpu_index: ::std::os::raw::c_uint),
>;
#[doc = " typedef qemu_plugin_vcpu_udata_cb_t - vcpu callback\n @vcpu_index: the current vcpu context\n @userdata: a pointer to some user data supplied when the callback\n was registered."]
pub type qemu_plugin_vcpu_udata_cb_t = ::std::option::Option<
    unsafe extern "C" fn(vcpu_index: ::std::os::raw::c_uint, userdata: *mut ::std::os::raw::c_void),
>;
extern "C" {
    #[doc = " qemu_plugin_uninstall() - Uninstall a plugin\n @id: this plugin's opaque ID\n @cb: callback to be called once the plugin has been removed\n\n Do NOT assume that the plugin has been uninstalled once this function\n returns. Plugins are uninstalled asynchronously, and therefore the given\n plugin receives callbacks until @cb is called.\n\n Note: Calling this function from qemu_plugin_install() is a bug."]
    pub fn qemu_plugin_uninstall(id: qemu_plugin_id_t, cb: qemu_plugin_simple_cb_t);
}
extern "C" {
    #[doc = " qemu_plugin_reset() - Reset a plugin\n @id: this plugin's opaque ID\n @cb: callback to be called once the plugin has been reset\n\n Unregisters all callbacks for the plugin given by @id.\n\n Do NOT assume that the plugin has been reset once this function returns.\n Plugins are reset asynchronously, and therefore the given plugin receives\n callbacks until @cb is called."]
    pub fn qemu_plugin_reset(id: qemu_plugin_id_t, cb: qemu_plugin_simple_cb_t);
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_init_cb() - register a vCPU initialization callback\n @id: plugin ID\n @cb: callback function\n\n The @cb function is called every time a vCPU is initialized.\n\n See also: qemu_plugin_register_vcpu_exit_cb()"]
    pub fn qemu_plugin_register_vcpu_init_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_simple_cb_t,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_exit_cb() - register a vCPU exit callback\n @id: plugin ID\n @cb: callback function\n\n The @cb function is called every time a vCPU exits.\n\n See also: qemu_plugin_register_vcpu_init_cb()"]
    pub fn qemu_plugin_register_vcpu_exit_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_simple_cb_t,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_idle_cb() - register a vCPU idle callback\n @id: plugin ID\n @cb: callback function\n\n The @cb function is called every time a vCPU idles."]
    pub fn qemu_plugin_register_vcpu_idle_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_simple_cb_t,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_resume_cb() - register a vCPU resume callback\n @id: plugin ID\n @cb: callback function\n\n The @cb function is called every time a vCPU resumes execution."]
    pub fn qemu_plugin_register_vcpu_resume_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_simple_cb_t,
    );
}
#[doc = " struct qemu_plugin_tb - Opaque handle for a translation block"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qemu_plugin_tb {
    _unused: [u8; 0],
}
#[doc = " struct qemu_plugin_insn - Opaque handle for a translated instruction"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qemu_plugin_insn {
    _unused: [u8; 0],
}
#[doc = " struct qemu_plugin_scoreboard - Opaque handle for a scoreboard"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qemu_plugin_scoreboard {
    _unused: [u8; 0],
}
#[doc = " typedef qemu_plugin_u64 - uint64_t member of an entry in a scoreboard\n\n This field allows to access a specific uint64_t member in one given entry,\n located at a specified offset. Inline operations expect this as entry."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct qemu_plugin_u64 {
    pub score: *mut qemu_plugin_scoreboard,
    pub offset: usize,
}
impl Default for qemu_plugin_u64 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(u32)]
#[doc = " enum qemu_plugin_cb_flags - type of callback\n\n @QEMU_PLUGIN_CB_NO_REGS: callback does not access the CPU's regs\n @QEMU_PLUGIN_CB_R_REGS: callback reads the CPU's regs\n @QEMU_PLUGIN_CB_RW_REGS: callback reads and writes the CPU's regs\n\n Note: currently QEMU_PLUGIN_CB_RW_REGS is unused, plugins cannot change\n system register state."]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum qemu_plugin_cb_flags {
    QEMU_PLUGIN_CB_NO_REGS = 0,
    QEMU_PLUGIN_CB_R_REGS = 1,
    QEMU_PLUGIN_CB_RW_REGS = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum qemu_plugin_mem_rw {
    QEMU_PLUGIN_MEM_R = 1,
    QEMU_PLUGIN_MEM_W = 2,
    QEMU_PLUGIN_MEM_RW = 3,
}
#[doc = " typedef qemu_plugin_vcpu_tb_trans_cb_t - translation callback\n @id: unique plugin id\n @tb: opaque handle used for querying and instrumenting a block."]
pub type qemu_plugin_vcpu_tb_trans_cb_t =
    ::std::option::Option<unsafe extern "C" fn(id: qemu_plugin_id_t, tb: *mut qemu_plugin_tb)>;
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_tb_trans_cb() - register a translate cb\n @id: plugin ID\n @cb: callback function\n\n The @cb function is called every time a translation occurs. The @cb\n function is passed an opaque qemu_plugin_type which it can query\n for additional information including the list of translated\n instructions. At this point the plugin can register further\n callbacks to be triggered when the block or individual instruction\n executes."]
    pub fn qemu_plugin_register_vcpu_tb_trans_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_tb_trans_cb_t,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_tb_exec_cb() - register execution callback\n @tb: the opaque qemu_plugin_tb handle for the translation\n @cb: callback function\n @flags: does the plugin read or write the CPU's registers?\n @userdata: any plugin data to pass to the @cb?\n\n The @cb function is called every time a translated unit executes."]
    pub fn qemu_plugin_register_vcpu_tb_exec_cb(
        tb: *mut qemu_plugin_tb,
        cb: qemu_plugin_vcpu_udata_cb_t,
        flags: qemu_plugin_cb_flags,
        userdata: *mut ::std::os::raw::c_void,
    );
}
#[repr(u32)]
#[doc = " enum qemu_plugin_op - describes an inline op\n\n @QEMU_PLUGIN_INLINE_ADD_U64: add an immediate value uint64_t\n\n Note: currently only a single inline op is supported."]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum qemu_plugin_op {
    QEMU_PLUGIN_INLINE_ADD_U64 = 0,
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_tb_exec_inline_per_vcpu() - execution inline op\n @tb: the opaque qemu_plugin_tb handle for the translation\n @op: the type of qemu_plugin_op (e.g. ADD_U64)\n @entry: entry to run op\n @imm: the op data (e.g. 1)\n\n Insert an inline op on a given scoreboard entry."]
    pub fn qemu_plugin_register_vcpu_tb_exec_inline_per_vcpu(
        tb: *mut qemu_plugin_tb,
        op: qemu_plugin_op,
        entry: qemu_plugin_u64,
        imm: u64,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_insn_exec_cb() - register insn execution cb\n @insn: the opaque qemu_plugin_insn handle for an instruction\n @cb: callback function\n @flags: does the plugin read or write the CPU's registers?\n @userdata: any plugin data to pass to the @cb?\n\n The @cb function is called every time an instruction is executed"]
    pub fn qemu_plugin_register_vcpu_insn_exec_cb(
        insn: *mut qemu_plugin_insn,
        cb: qemu_plugin_vcpu_udata_cb_t,
        flags: qemu_plugin_cb_flags,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_insn_exec_inline_per_vcpu() - insn exec inline op\n @insn: the opaque qemu_plugin_insn handle for an instruction\n @op: the type of qemu_plugin_op (e.g. ADD_U64)\n @entry: entry to run op\n @imm: the op data (e.g. 1)\n\n Insert an inline op to every time an instruction executes."]
    pub fn qemu_plugin_register_vcpu_insn_exec_inline_per_vcpu(
        insn: *mut qemu_plugin_insn,
        op: qemu_plugin_op,
        entry: qemu_plugin_u64,
        imm: u64,
    );
}
extern "C" {
    #[doc = " qemu_plugin_tb_n_insns() - query helper for number of insns in TB\n @tb: opaque handle to TB passed to callback\n\n Returns: number of instructions in this block"]
    pub fn qemu_plugin_tb_n_insns(tb: *const qemu_plugin_tb) -> usize;
}
extern "C" {
    #[doc = " qemu_plugin_tb_vaddr() - query helper for vaddr of TB start\n @tb: opaque handle to TB passed to callback\n\n Returns: virtual address of block start"]
    pub fn qemu_plugin_tb_vaddr(tb: *const qemu_plugin_tb) -> u64;
}
extern "C" {
    #[doc = " qemu_plugin_tb_get_insn() - retrieve handle for instruction\n @tb: opaque handle to TB passed to callback\n @idx: instruction number, 0 indexed\n\n The returned handle can be used in follow up helper queries as well\n as when instrumenting an instruction. It is only valid for the\n lifetime of the callback.\n\n Returns: opaque handle to instruction"]
    pub fn qemu_plugin_tb_get_insn(tb: *const qemu_plugin_tb, idx: usize) -> *mut qemu_plugin_insn;
}
extern "C" {
    #[doc = " qemu_plugin_insn_data() - return ptr to instruction data\n @insn: opaque instruction handle from qemu_plugin_tb_get_insn()\n\n Note: data is only valid for duration of callback. See\n qemu_plugin_insn_size() to calculate size of stream.\n\n Returns: pointer to a stream of bytes containing the value of this\n instructions opcode."]
    pub fn qemu_plugin_insn_data(insn: *const qemu_plugin_insn) -> *const ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " qemu_plugin_insn_size() - return size of instruction\n @insn: opaque instruction handle from qemu_plugin_tb_get_insn()\n\n Returns: size of instruction in bytes"]
    pub fn qemu_plugin_insn_size(insn: *const qemu_plugin_insn) -> usize;
}
extern "C" {
    #[doc = " qemu_plugin_insn_vaddr() - return vaddr of instruction\n @insn: opaque instruction handle from qemu_plugin_tb_get_insn()\n\n Returns: virtual address of instruction"]
    pub fn qemu_plugin_insn_vaddr(insn: *const qemu_plugin_insn) -> u64;
}
extern "C" {
    #[doc = " qemu_plugin_insn_haddr() - return hardware addr of instruction\n @insn: opaque instruction handle from qemu_plugin_tb_get_insn()\n\n Returns: hardware (physical) target address of instruction"]
    pub fn qemu_plugin_insn_haddr(insn: *const qemu_plugin_insn) -> *mut ::std::os::raw::c_void;
}
#[doc = " typedef qemu_plugin_meminfo_t - opaque memory transaction handle\n\n This can be further queried using the qemu_plugin_mem_* query\n functions."]
pub type qemu_plugin_meminfo_t = u32;
#[doc = " struct qemu_plugin_hwaddr - opaque hw address handle"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qemu_plugin_hwaddr {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " qemu_plugin_mem_size_shift() - get size of access\n @info: opaque memory transaction handle\n\n Returns: size of access in ^2 (0=byte, 1=16bit, 2=32bit etc...)"]
    pub fn qemu_plugin_mem_size_shift(info: qemu_plugin_meminfo_t) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " qemu_plugin_mem_is_sign_extended() - was the access sign extended\n @info: opaque memory transaction handle\n\n Returns: true if it was, otherwise false"]
    pub fn qemu_plugin_mem_is_sign_extended(info: qemu_plugin_meminfo_t) -> bool;
}
extern "C" {
    #[doc = " qemu_plugin_mem_is_big_endian() - was the access big endian\n @info: opaque memory transaction handle\n\n Returns: true if it was, otherwise false"]
    pub fn qemu_plugin_mem_is_big_endian(info: qemu_plugin_meminfo_t) -> bool;
}
extern "C" {
    #[doc = " qemu_plugin_mem_is_store() - was the access a store\n @info: opaque memory transaction handle\n\n Returns: true if it was, otherwise false"]
    pub fn qemu_plugin_mem_is_store(info: qemu_plugin_meminfo_t) -> bool;
}
extern "C" {
    #[doc = " qemu_plugin_get_hwaddr() - return handle for memory operation\n @info: opaque memory info structure\n @vaddr: the virtual address of the memory operation\n\n For system emulation returns a qemu_plugin_hwaddr handle to query\n details about the actual physical address backing the virtual\n address. For linux-user guests it just returns NULL.\n\n This handle is *only* valid for the duration of the callback. Any\n information about the handle should be recovered before the\n callback returns."]
    pub fn qemu_plugin_get_hwaddr(
        info: qemu_plugin_meminfo_t,
        vaddr: u64,
    ) -> *mut qemu_plugin_hwaddr;
}
extern "C" {
    #[doc = " qemu_plugin_hwaddr_is_io() - query whether memory operation is IO\n @haddr: address handle from qemu_plugin_get_hwaddr()\n\n Returns true if the handle's memory operation is to memory-mapped IO, or\n false if it is to RAM"]
    pub fn qemu_plugin_hwaddr_is_io(haddr: *const qemu_plugin_hwaddr) -> bool;
}
extern "C" {
    #[doc = " qemu_plugin_hwaddr_phys_addr() - query physical address for memory operation\n @haddr: address handle from qemu_plugin_get_hwaddr()\n\n Returns the physical address associated with the memory operation\n\n Note that the returned physical address may not be unique if you are dealing\n with multiple address spaces."]
    pub fn qemu_plugin_hwaddr_phys_addr(haddr: *const qemu_plugin_hwaddr) -> u64;
}
extern "C" {
    #[doc = " Returns a string representing the device. The string is valid for\n the lifetime of the plugin."]
    pub fn qemu_plugin_hwaddr_device_name(
        h: *const qemu_plugin_hwaddr,
    ) -> *const ::std::os::raw::c_char;
}
#[doc = " typedef qemu_plugin_vcpu_mem_cb_t - memory callback function type\n @vcpu_index: the executing vCPU\n @info: an opaque handle for further queries about the memory\n @vaddr: the virtual address of the transaction\n @userdata: any user data attached to the callback"]
pub type qemu_plugin_vcpu_mem_cb_t = ::std::option::Option<
    unsafe extern "C" fn(
        vcpu_index: ::std::os::raw::c_uint,
        info: qemu_plugin_meminfo_t,
        vaddr: u64,
        userdata: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_mem_cb() - register memory access callback\n @insn: handle for instruction to instrument\n @cb: callback of type qemu_plugin_vcpu_mem_cb_t\n @flags: (currently unused) callback flags\n @rw: monitor reads, writes or both\n @userdata: opaque pointer for userdata\n\n This registers a full callback for every memory access generated by\n an instruction. If the instruction doesn't access memory no\n callback will be made.\n\n The callback reports the vCPU the access took place on, the virtual\n address of the access and a handle for further queries. The user\n can attach some userdata to the callback for additional purposes.\n\n Other execution threads will continue to execute during the\n callback so the plugin is responsible for ensuring it doesn't get\n confused by making appropriate use of locking if required."]
    pub fn qemu_plugin_register_vcpu_mem_cb(
        insn: *mut qemu_plugin_insn,
        cb: qemu_plugin_vcpu_mem_cb_t,
        flags: qemu_plugin_cb_flags,
        rw: qemu_plugin_mem_rw,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " qemu_plugin_register_vcpu_mem_inline_per_vcpu() - inline op for mem access\n @insn: handle for instruction to instrument\n @rw: apply to reads, writes or both\n @op: the op, of type qemu_plugin_op\n @entry: entry to run op\n @imm: immediate data for @op\n\n This registers a inline op every memory access generated by the\n instruction."]
    pub fn qemu_plugin_register_vcpu_mem_inline_per_vcpu(
        insn: *mut qemu_plugin_insn,
        rw: qemu_plugin_mem_rw,
        op: qemu_plugin_op,
        entry: qemu_plugin_u64,
        imm: u64,
    );
}
pub type qemu_plugin_vcpu_syscall_cb_t = ::std::option::Option<
    unsafe extern "C" fn(
        id: qemu_plugin_id_t,
        vcpu_index: ::std::os::raw::c_uint,
        num: i64,
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
    ),
>;
extern "C" {
    pub fn qemu_plugin_register_vcpu_syscall_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_syscall_cb_t,
    );
}
pub type qemu_plugin_vcpu_syscall_ret_cb_t = ::std::option::Option<
    unsafe extern "C" fn(
        id: qemu_plugin_id_t,
        vcpu_idx: ::std::os::raw::c_uint,
        num: i64,
        ret: i64,
    ),
>;
extern "C" {
    pub fn qemu_plugin_register_vcpu_syscall_ret_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_vcpu_syscall_ret_cb_t,
    );
}
extern "C" {
    #[doc = " qemu_plugin_insn_disas() - return disassembly string for instruction\n @insn: instruction reference\n\n Returns an allocated string containing the disassembly"]
    pub fn qemu_plugin_insn_disas(insn: *const qemu_plugin_insn) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " qemu_plugin_insn_symbol() - best effort symbol lookup\n @insn: instruction reference\n\n Return a static string referring to the symbol. This is dependent\n on the binary QEMU is running having provided a symbol table."]
    pub fn qemu_plugin_insn_symbol(insn: *const qemu_plugin_insn) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " qemu_plugin_vcpu_for_each() - iterate over the existing vCPU\n @id: plugin ID\n @cb: callback function\n\n The @cb function is called once for each existing vCPU.\n\n See also: qemu_plugin_register_vcpu_init_cb()"]
    pub fn qemu_plugin_vcpu_for_each(id: qemu_plugin_id_t, cb: qemu_plugin_vcpu_simple_cb_t);
}
extern "C" {
    pub fn qemu_plugin_register_flush_cb(id: qemu_plugin_id_t, cb: qemu_plugin_simple_cb_t);
}
extern "C" {
    #[doc = " qemu_plugin_register_atexit_cb() - register exit callback\n @id: plugin ID\n @cb: callback\n @userdata: user data for callback\n\n The @cb function is called once execution has finished. Plugins\n should be able to free all their resources at this point much like\n after a reset/uninstall callback is called.\n\n In user-mode it is possible a few un-instrumented instructions from\n child threads may run before the host kernel reaps the threads."]
    pub fn qemu_plugin_register_atexit_cb(
        id: qemu_plugin_id_t,
        cb: qemu_plugin_udata_cb_t,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " returns how many vcpus were started at this point"]
    pub fn qemu_plugin_num_vcpus() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " qemu_plugin_outs() - output string via QEMU's logging system\n @string: a string"]
    pub fn qemu_plugin_outs(string: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " qemu_plugin_bool_parse() - parses a boolean argument in the form of\n \"<argname>=[on|yes|true|off|no|false]\"\n\n @name: argument name, the part before the equals sign\n @val: argument value, what's after the equals sign\n @ret: output return value\n\n returns true if the combination @name=@val parses correctly to a boolean\n argument, and false otherwise"]
    pub fn qemu_plugin_bool_parse(
        name: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
        ret: *mut bool,
    ) -> bool;
}
extern "C" {
    #[doc = " qemu_plugin_path_to_binary() - path to binary file being executed\n\n Return a string representing the path to the binary. For user-mode\n this is the main executable. For system emulation we currently\n return NULL. The user should g_free() the string once no longer\n needed."]
    pub fn qemu_plugin_path_to_binary() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " qemu_plugin_start_code() - returns start of text segment\n\n Returns the nominal start address of the main text segment in\n user-mode. Currently returns 0 for system emulation."]
    pub fn qemu_plugin_start_code() -> u64;
}
extern "C" {
    #[doc = " qemu_plugin_end_code() - returns end of text segment\n\n Returns the nominal end address of the main text segment in\n user-mode. Currently returns 0 for system emulation."]
    pub fn qemu_plugin_end_code() -> u64;
}
extern "C" {
    #[doc = " qemu_plugin_entry_code() - returns start address for module\n\n Returns the nominal entry address of the main text segment in\n user-mode. Currently returns 0 for system emulation."]
    pub fn qemu_plugin_entry_code() -> u64;
}
#[doc = " struct qemu_plugin_register - Opaque handle for register access"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qemu_plugin_register {
    _unused: [u8; 0],
}
#[doc = " typedef qemu_plugin_reg_descriptor - register descriptions\n\n @handle: opaque handle for retrieving value with qemu_plugin_read_register\n @name: register name\n @feature: optional feature descriptor, can be NULL"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct qemu_plugin_reg_descriptor {
    pub handle: *mut qemu_plugin_register,
    pub name: *const ::std::os::raw::c_char,
    pub feature: *const ::std::os::raw::c_char,
}
impl Default for qemu_plugin_reg_descriptor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[doc = " qemu_plugin_get_registers() - return register list for current vCPU\n\n Returns a potentially empty GArray of qemu_plugin_reg_descriptor.\n Caller frees the array (but not the const strings).\n\n Should be used from a qemu_plugin_register_vcpu_init_cb() callback\n after the vCPU is initialised, i.e. in the vCPU context."]
    pub fn qemu_plugin_get_registers() -> *mut GArray;
}
extern "C" {
    #[doc = " qemu_plugin_read_register() - read register for current vCPU\n\n @handle: a @qemu_plugin_reg_handle handle\n @buf: A GByteArray for the data owned by the plugin\n\n This function is only available in a context that register read access is\n explicitly requested via the QEMU_PLUGIN_CB_R_REGS flag.\n\n Returns the size of the read register. The content of @buf is in target byte\n order. On failure returns -1."]
    pub fn qemu_plugin_read_register(
        handle: *mut qemu_plugin_register,
        buf: *mut GByteArray,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " qemu_plugin_scoreboard_new() - alloc a new scoreboard\n\n @element_size: size (in bytes) for one entry\n\n Returns a pointer to a new scoreboard. It must be freed using\n qemu_plugin_scoreboard_free."]
    pub fn qemu_plugin_scoreboard_new(element_size: usize) -> *mut qemu_plugin_scoreboard;
}
extern "C" {
    #[doc = " qemu_plugin_scoreboard_free() - free a scoreboard\n @score: scoreboard to free"]
    pub fn qemu_plugin_scoreboard_free(score: *mut qemu_plugin_scoreboard);
}
extern "C" {
    #[doc = " qemu_plugin_scoreboard_find() - get pointer to an entry of a scoreboard\n @score: scoreboard to query\n @vcpu_index: entry index\n\n Returns address of entry of a scoreboard matching a given vcpu_index. This\n address can be modified later if scoreboard is resized."]
    pub fn qemu_plugin_scoreboard_find(
        score: *mut qemu_plugin_scoreboard,
        vcpu_index: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " qemu_plugin_u64_add() - add a value to a qemu_plugin_u64 for a given vcpu\n @entry: entry to query\n @vcpu_index: entry index\n @added: value to add"]
    pub fn qemu_plugin_u64_add(
        entry: qemu_plugin_u64,
        vcpu_index: ::std::os::raw::c_uint,
        added: u64,
    );
}
extern "C" {
    #[doc = " qemu_plugin_u64_get() - get value of a qemu_plugin_u64 for a given vcpu\n @entry: entry to query\n @vcpu_index: entry index"]
    pub fn qemu_plugin_u64_get(entry: qemu_plugin_u64, vcpu_index: ::std::os::raw::c_uint) -> u64;
}
extern "C" {
    #[doc = " qemu_plugin_u64_set() - set value of a qemu_plugin_u64 for a given vcpu\n @entry: entry to query\n @vcpu_index: entry index\n @val: new value"]
    pub fn qemu_plugin_u64_set(
        entry: qemu_plugin_u64,
        vcpu_index: ::std::os::raw::c_uint,
        val: u64,
    );
}
extern "C" {
    #[doc = " qemu_plugin_u64_sum() - return sum of all vcpu entries in a scoreboard\n @entry: entry to sum"]
    pub fn qemu_plugin_u64_sum(entry: qemu_plugin_u64) -> u64;
}
