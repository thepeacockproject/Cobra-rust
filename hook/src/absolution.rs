#![allow(non_snake_case)]

use super::config::Config;
use super::glacier::ZString;

use retour::GenericDetour;
use std::{
    ffi::{c_void, c_char, CStr, CString},
    mem::transmute
};

type GetApplicationOptionBoolFunc = extern "cdecl" fn(*const ZString, bool) -> bool;
type ZWebServiceClientManagerCreateFunc = extern "fastcall" fn(*mut c_void, *mut c_void, *const c_char, *mut c_void) -> *mut c_void;

static mut GET_APPLICATION_OPTION_BOOL: Option<GenericDetour<GetApplicationOptionBoolFunc>> = None;
static mut ZWEBSERVICE_CLIENT_MANAGER_CREATE: Option<GenericDetour<ZWebServiceClientManagerCreateFunc>> = None;
static mut WEBSERVICE_URL: String = String::new();

extern "cdecl" fn GetApplicationOptionBool(sName: *const ZString, bDefault: bool) -> bool {
    unsafe {
        if CStr::from_ptr(sName.as_ref().unwrap().m_chars).to_str().unwrap() == "EnableSplashScreen" {
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
}

extern "fastcall" fn ZWebServiceClientManagerCreate(instance: *mut c_void, arg1: *mut c_void, _baseUrl: *const c_char, callback: *mut c_void) -> *mut c_void {
    println!("[COBRA//HOOK] ZWebServiceClientManagerCreate called.");
    
    unsafe {
        let url = CString::new(WEBSERVICE_URL.as_str()).unwrap();
        match ZWEBSERVICE_CLIENT_MANAGER_CREATE {
            Some(ref mut hook) => hook.call(instance, arg1, url.as_ptr(), callback),
            None => {
                println!("[COBRA//HOOK] Could not invoke ZWebServiceClientManagerCreate.");
                return 0 as *mut c_void;
            }
        }
    }
}

pub fn init_absolution(cfg: Config) {
    unsafe {
        WEBSERVICE_URL = cfg.hm5.url;

        let o_GetApplicationOptionBool: GetApplicationOptionBoolFunc = transmute(0x00612930 as usize);
        let o_ZWebServiceClientManagerCreate: ZWebServiceClientManagerCreateFunc = transmute(0x00BE22CF as usize);

        GET_APPLICATION_OPTION_BOOL = Some(
            GenericDetour::<GetApplicationOptionBoolFunc>::new(o_GetApplicationOptionBool, GetApplicationOptionBool).unwrap()
        );

        if cfg.hm5.skiplauncher {
            match GET_APPLICATION_OPTION_BOOL {
                Some(ref mut hook) => {
                    hook.enable().unwrap();
                    println!("[COBRA//HOOK] GetApplicationOptionBool hooked.");
                },
                None => println!("[COBRA//HOOK] Failed to hook GetApplicationOptionBool.")
            }
        }

        ZWEBSERVICE_CLIENT_MANAGER_CREATE = Some(
            GenericDetour::<ZWebServiceClientManagerCreateFunc>::new(o_ZWebServiceClientManagerCreate, ZWebServiceClientManagerCreate).unwrap()
        );

        match ZWEBSERVICE_CLIENT_MANAGER_CREATE {
            Some(ref mut hook) => {
                hook.enable().unwrap();
                println!("[COBRA//HOOK] ZWebServiceClientManagerCreate hooked.");
            },
            None => println!("[COBRA//HOOK] Failed to hook ZWebServiceClientManagerCreate.")
        }
    }
}
