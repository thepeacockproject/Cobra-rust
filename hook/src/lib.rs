mod config;
mod dinput8;
mod glacier;
mod absolution;

use absolution::init_absolution;
use dinput8::setup_dinput8;
use config::{Config, ConfigError};

use std::ffi::c_void;
use windows::Win32::{
    Foundation::{BOOL, HMODULE},
    System::{
        Console::{AllocConsole, AttachConsole},
        Diagnostics::Debug::GetTimestampForLoadedLibrary,
        LibraryLoader::GetModuleHandleA,
        SystemServices::DLL_PROCESS_ATTACH,
        Threading::GetCurrentProcessId,
    },
    UI::WindowsAndMessaging::{
        MessageBoxA, MB_OK, MB_ICONERROR, MB_ICONINFORMATION
    }
};
use windows::{
    core::PCSTR,
    s,
};

#[no_mangle]
pub extern "stdcall" fn DllMain(_inst_dll: HMODULE, reason: u32, _reserved: *const c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        setup_dinput8();

        let cfg: Config;
        match Config::load() {
            Ok(config) => cfg = config,
            Err(err) => {
                let message: PCSTR;

                match err {
                    ConfigError::FileWrite => message = s!("Failed to write config file. Cobra will not be started."),
                    ConfigError::FileRead => message = s!("Failed to read config file. Cobra will not be started."),
                    ConfigError::Parse => message = s!("Failed to parse config file. Cobra will not be started.")
                }

                unsafe {
                    MessageBoxA(
                        None,
                        message,
                        s!("Cobra - Error"),
                        MB_OK | MB_ICONERROR
                    );
                }

                return true.into();
            }
        }

        if cfg.options.console {
            unsafe {
                if !(AttachConsole(GetCurrentProcessId()).as_bool() || AllocConsole().as_bool()) {
                    panic!("[COBRA//HOOK] Unable to create console!");
                }
            }
        }

        let timestamp = unsafe { GetTimestampForLoadedLibrary(GetModuleHandleA(None).unwrap()) };

        match timestamp {
            0x5047356A => println!("[COBRA//HOOK] HITMAN: Sniper Challenge (STEAM) found!"),
            0x5149E0B4 => init_absolution(cfg),
            _ => {
                println!("[COBRA//HOOK] Unknown (or unable to find) game version.");

                unsafe {
                    MessageBoxA(
                        None,
                        s!("Cobra was unable to find the game version you are running. Cobra will not be started."),
                        s!("Cobra - Game Version Error"),
                        MB_OK | MB_ICONINFORMATION
                    );
                }
            },
        }

        // TODO: Add hooks.
    }
    true.into()
}
