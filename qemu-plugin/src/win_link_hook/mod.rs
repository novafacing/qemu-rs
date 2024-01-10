//! Hook for linking against exported QEMU symbols at runtime

use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::WindowsProgramming::DELAYLOAD_INFO;

type FailureHook = unsafe extern "C" fn(dli_notify: u32, pdli: DELAYLOAD_INFO) -> HMODULE;

#[no_mangle]
/// Callback invoked by Windows loader to complete linking against QEMU symbols
pub static __pfnDliFailureHook2: FailureHook = delaylink_hook;

extern "C" fn delaylink_hook(dli_notify: u32, pdli: DELAYLOAD_INFO) -> HMODULE {
    if dli_notify == 3 {
        let name = unsafe { pdli.TargetDllName.to_string() }.unwrap_or_default();
        if name == "qemu.exe" {
            match unsafe { GetModuleHandleA(PCSTR::null()) } {
                Ok(h) => return h,
                Err(e) => {
                    eprintln!("Error loading top level qemu module: {e:?}");
                }
            }
        }
    }

    HMODULE::default()
}
