//! Hook for linking against exported QEMU symbols at runtime

use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetModuleHandleExA;
use windows::Win32::System::WindowsProgramming::DELAYLOAD_INFO;

/// The helper function for linker-supported delayed loading which is what actually
/// loads the DLL at runtime.
type DelayHook = unsafe extern "C" fn(dli_notify: DliNotify, pdli: DELAYLOAD_INFO) -> HMODULE;

#[no_mangle]
/// Helper function invoked when failures occur in delay linking (as opposed to
/// notifications)
pub static __pfnDliFailureHook2: DelayHook = delaylink_hook;

#[repr(C)]
/// Delay load import hook notifications
///
enum DliNotify {
    /// Used to bypass
    DliNoteStartProcessing = 0,
    /// Called just before LoadLibrary, can override w/ new HMODULE return val
    DliNotePreLoadLibrary,
    /// Called just before GetProcAddress, override w/ new FARPROC return value
    DliNotePreGetProcAddress,
    /// Failed to load library, fix it by
    ///  returning a valid HMODULE
    DliFailLoadLib,
    /// Failed to get proc address, fix it by
    ///  returning a valid FARPROC
    DliFailGetProc,
    /// Called after all processing is done, no bypass possible at this point except by
    /// longjmp()/throw()/RaiseException.
    DliNoteEndProcessing,
}

/// Helper function invoked when notifications or failures occur in delay linking
///
/// # Arguments
///
/// * `dli_notify` - The
extern "C" fn delaylink_hook(dli_notify: DliNotify, pdli: DELAYLOAD_INFO) -> HMODULE {
    if let DliNotify::DliFailLoadLib = dli_notify {
        // SAFETY: Conversion of `PCSTR` to String is not safe because it involves an unchecked
        // nul-byte dependent `strcpy`. In this instance, it is safe because
        let name = unsafe { pdli.TargetDllName.to_string() }.unwrap_or_default();
        let mut module = HMODULE::default();

        // NOTE: QEMU executables on windows are named qemu-system.*.exe
        if name.starts_with("qemu") {
            // SAFETY: Getting the module handle for NULL is safe and does not dereference any
            // pointers except to write the `module` argument which we know is alive here.
            match unsafe { GetModuleHandleExA(0, PCSTR::null(), &mut module as *mut HMODULE) } {
                Ok(_) => return module,
                Err(e) => panic!("Failed to open QEMU module: {e:?}"),
            }
        }
    }

    HMODULE::default()
}
