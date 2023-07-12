mod dinput8;

use dinput8::setup_dinput8;

use std::ffi::c_void;
use windows::Win32::{Foundation::HMODULE, System::SystemServices::DLL_PROCESS_ATTACH};

#[no_mangle]
pub extern "system" fn DllMain(_inst_dll: HMODULE, reason: u32, _reserved: *const c_void) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        setup_dinput8();

        // TODO: Add game detection + hooks.
    }
    true
}
