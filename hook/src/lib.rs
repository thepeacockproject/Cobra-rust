mod absolution;
mod config;
mod dinput8;
mod glacier;
mod sniper;

use absolution::init_absolution;
use config::{Config, ConfigError};
use dinput8::setup_dinput8;
use sniper::init_sniper;

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
    UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_ICONINFORMATION, MB_OK},
};
use windows::{core::PCSTR, s};

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    _inst_dll: HMODULE,
    reason: u32,
    _reserved: *const c_void,
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        setup_dinput8();

        let cfg: Config = match Config::load() {
            Ok(config) => config,
            Err(err) => {
                let message: PCSTR = match err {
                    ConfigError::FileWrite => {
                        s!("Failed to write config file. Cobra will not be started.")
                    }
                    ConfigError::FileRead => {
                        s!("Failed to read config file. Cobra will not be started.")
                    }
                    ConfigError::Parse => {
                        s!("Failed to parse config file. Cobra will not be started.")
                    }
                };

                MessageBoxA(None, message, s!("Cobra - Error"), MB_OK | MB_ICONERROR);

                return true.into();
            }
        };

        if cfg.options.console
            && !(AttachConsole(GetCurrentProcessId()).as_bool() || AllocConsole().as_bool())
        {
            panic!("[COBRA//HOOK] Unable to create console!");
        }

        let timestamp = GetTimestampForLoadedLibrary(GetModuleHandleA(None).unwrap());

        match timestamp {
            0x5047356A => init_sniper(cfg),
            0x5149E0B4 => init_absolution(cfg),
            _ => {
                println!("[COBRA//HOOK] Unknown (or unable to find) game version.");

                MessageBoxA(
                    None,
                    s!("Cobra was unable to find the game version you are running. Cobra will not be started."),
                    s!("Cobra - Game Version Error"),
                    MB_OK | MB_ICONINFORMATION
                );
            }
        }
    }
    true.into()
}
