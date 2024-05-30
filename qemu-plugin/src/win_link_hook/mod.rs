//! Hook for linking against exported QEMU symbols at runtime

use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::WindowsProgramming::DELAYLOAD_INFO;

/// The helper function for linker-supported delayed loading which is what actually
/// loads the DLL at runtime.
type DelayHook = unsafe extern "C" fn(dli_notify: DliNotify, pdli: DELAYLOAD_INFO) -> HMODULE;

#[no_mangle]
/// Helper function invoked when failures occur in delay linking (as opposed to
/// notifications)
static __pfnDliFailureHook2: DelayHook = delaylink_hook;

#[allow(dead_code, clippy::enum_variant_names)] //We only need one of these variants.
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
/// * `dli_notify` - The type of notification
/// * `pdli` - The delay load information
///
/// # Return value
///
/// * `HMODULE` - The handle to the module
extern "C" fn delaylink_hook(dli_notify: DliNotify, pdli: DELAYLOAD_INFO) -> HMODULE {
    if let DliNotify::DliFailLoadLib = dli_notify {
        // SAFETY: Conversion of `PCSTR` to String is not safe because it involves an unchecked
        // nul-byte dependent `strcpy`. In this instance, it is as safe as OS guarantees because
        // the target dll name is provided by Windows and is null-terminated.
        let name = unsafe { pdli.TargetDllName.to_string() }.unwrap_or_default();

        // NOTE: QEMU executables on windows are named qemu-system.*.exe
        if name == "qemu.exe" {
            return HMODULE(
                libloading::os::windows::Library::this()
                    .expect("Get QEMU module")
                    .into_raw(),
            );
        }
    }

    HMODULE::default()
}
