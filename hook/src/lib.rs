mod dinput8;

use dinput8::setup_dinput8;

use std::ffi::c_void;
use windows::Win32::{
    Foundation::{HMODULE, BOOL},
    System::{
        SystemServices::DLL_PROCESS_ATTACH,
        LibraryLoader::GetModuleHandleA,
        Diagnostics::Debug::GetTimestampForLoadedLibrary,
        Console::{AllocConsole, AttachConsole},
        Threading::GetCurrentProcessId
    }
};

#[no_mangle]
pub extern "stdcall" fn DllMain(_inst_dll: HMODULE, reason: u32, _reserved: *const c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        setup_dinput8();

        // TODO: Make showing a console configurable
        unsafe {
            if !(AttachConsole(GetCurrentProcessId()).as_bool() || AllocConsole().as_bool()) {
                panic!("[COBRA//HOOK] Unable to create console!");
            }
        }

        let timestamp = unsafe { GetTimestampForLoadedLibrary(GetModuleHandleA(None).unwrap()) };

        match timestamp {
            0x5047356A => println!("[COBRA//HOOK] HITMAN: Sniper Challenge (STEAM) found!"),
            0x5149E0B4 => println!("[COBRA//HOOK] HITMAN: Absolution (STEAM) found!"),
            _ => println!("[COBRA//HOOK] Unknown game version found.")
        }

        // TODO: Add hooks.
    }
    true.into()
}
