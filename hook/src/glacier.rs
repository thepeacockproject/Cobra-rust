#![allow(non_snake_case)]

use retour::GenericDetour;
use std::ffi::{c_char, c_void, CStr, CString};

pub static mut GET_APPLICATION_OPTION_BOOL: Option<GenericDetour<GetApplicationOptionBoolFunc>> =
    None;
pub static mut ZWEBSERVICE_CLIENT_MANAGER_CREATE: Option<
    GenericDetour<ZWebServiceClientManagerCreateFunc>,
> = None;
pub static mut WEBSERVICE_URL: String = String::new();

#[repr(C)]
pub struct ZString {
    pub m_length: u32,
    pub m_chars: *const c_char,
}

pub type GetApplicationOptionBoolFunc = unsafe extern "cdecl" fn(*const ZString, bool) -> bool;
pub type ZWebServiceClientManagerCreateFunc = unsafe extern "fastcall" fn(
    *mut c_void,
    *mut c_void,
    *const c_char,
    *mut c_void,
) -> *mut c_void;

pub unsafe extern "cdecl" fn GetApplicationOptionBool(
    sName: *const ZString,
    bDefault: bool,
) -> bool {
    if CStr::from_ptr(sName.as_ref().unwrap().m_chars)
        .to_str()
        .unwrap()
        == "EnableSplashScreen"
    {
        return false;
    }

    match GET_APPLICATION_OPTION_BOOL {
        Some(ref mut hook) => hook.call(sName, bDefault),
        None => {
            println!("[COBRA//HOOK] Could not invoke GetApplicationOptionBool.");
            return true;
        }
    }
}

pub unsafe extern "fastcall" fn ZWebServiceClientManagerCreate(
    instance: *mut c_void,
    arg1: *mut c_void,
    _baseUrl: *const c_char,
    callback: *mut c_void,
) -> *mut c_void {
    let url = CString::new(WEBSERVICE_URL.as_str()).unwrap();
    match ZWEBSERVICE_CLIENT_MANAGER_CREATE {
        Some(ref mut hook) => hook.call(instance, arg1, url.as_ptr(), callback),
        None => {
            println!("[COBRA//HOOK] Could not invoke ZWebServiceClientManagerCreate.");
            return 0 as *mut c_void;
        }
    }
}
