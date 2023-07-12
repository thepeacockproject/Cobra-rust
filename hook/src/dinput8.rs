use std::{ffi::c_void, mem::transmute};
use windows::{
    core::{IUnknown, GUID, HRESULT, HSTRING, PCWSTR},
    s,
    Win32::{
        Foundation::{FARPROC, HMODULE, MAX_PATH},
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            SystemInformation::GetSystemDirectoryW,
        },
    },
};

static mut ORIG_DIRECT_INPUT8_CREATE: FARPROC = None;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn DirectInput8Create(
    hinst: HMODULE,
    dwVersion: u32,
    riidltf: GUID,
    ppvOut: *mut *mut c_void,
    punkOuter: *const IUnknown,
) -> HRESULT {
    type DirectInput8CreateFunc = extern "system" fn(
        hinst: HMODULE,
        dwVersion: u32,
        riidltf: GUID,
        ppvOut: *mut *mut c_void,
        punkOuter: *const IUnknown,
    ) -> HRESULT;
    let o_DirectInput8Create: DirectInput8CreateFunc = unsafe { transmute(ORIG_DIRECT_INPUT8_CREATE) };

    return o_DirectInput8Create(hinst, dwVersion, riidltf, ppvOut, punkOuter);
}

pub fn setup_dinput8() {
    let sys_dir = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetSystemDirectoryW(Some(&mut buf));
        PCWSTR::from_raw(buf.as_ptr()).to_string().unwrap()
    };
    let dinput8_path = format!("{}\\dinput8.dll", sys_dir);
    let dinput8_inst = unsafe { LoadLibraryW(&HSTRING::from(dinput8_path)) }.unwrap();

    if dinput8_inst.is_invalid() {
        // We can't properly place the hook, we cannot continue.
        panic!("[COBRA//HOOK] Failed to load dinput8.dll!");
    }
    let dinput8_create = unsafe { GetProcAddress(dinput8_inst, s!("DirectInput8Create")) };
    unsafe { ORIG_DIRECT_INPUT8_CREATE = Some(dinput8_create.unwrap()) };
}
