#![allow(non_snake_case)]

use super::config::Config;
use super::glacier::ZString;

use retour::GenericDetour;
use std::{
    ffi::{c_char, c_void, CStr, CString},
    mem::transmute,
};
use substring::Substring;
use tinyrlibc::{strcpy, CChar};

type GetApplicationOptionBoolFunc = unsafe extern "cdecl" fn(*const ZString, bool) -> bool;
type ZWebServiceClientManagerCreateFunc =
    unsafe extern "cdecl" fn(*const c_char, *mut c_void) -> *mut c_void;

static mut GET_APPLICATION_OPTION_BOOL: Option<GenericDetour<GetApplicationOptionBoolFunc>> = None;
static mut ZWEBSERVICE_CLIENT_MANAGER_CREATE: Option<
    GenericDetour<ZWebServiceClientManagerCreateFunc>,
> = None;
static mut WEBSERVICE_URL: String = String::new();
static mut PATCHED: bool = false;
static mut URL_PATCH_ENABLED: bool = false;

unsafe extern "cdecl" fn GetApplicationOptionBool(sName: *const ZString, bDefault: bool) -> bool {
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

unsafe extern "cdecl" fn ZWebServiceClientManagerCreate(
    _baseUrl: *const c_char,
    callback: *mut c_void,
) -> *mut c_void {
    let url = CString::new(WEBSERVICE_URL.as_str()).unwrap();
    match ZWEBSERVICE_CLIENT_MANAGER_CREATE {
        Some(ref mut hook) => hook.call(url.as_ptr(), callback),
        None => {
            println!("[COBRA//HOOK] Could not invoke ZWebServiceClientManagerCreate.");
            return 0 as *mut c_void;
        }
    }
}

pub unsafe fn patch_sniper() {
    // Don't patch the memory if it isn't sniper or if we've already done it.
    // Technically DirectInput8Create should only be called once, but just to be sure.
    if PATCHED || !URL_PATCH_ENABLED {
        return;
    }

    let url = CString::new(WEBSERVICE_URL.substring(0, 256)).unwrap();
    strcpy(0x113D5E8 as *mut CChar, url.as_bytes().as_ptr());

    PATCHED = true;
}

pub unsafe fn init_sniper(cfg: Config) {
    WEBSERVICE_URL = cfg.sniper.url;

    // This is a horrible way of checking if this is sniper.
    URL_PATCH_ENABLED = true;

    // FIXME: These hooks currently do not work.
    let o_GetApplicationOptionBool: GetApplicationOptionBoolFunc = transmute(0x004984C0 as usize);
    let o_ZWebServiceClientManagerCreate: ZWebServiceClientManagerCreateFunc =
        transmute(0x00C043D1 as usize);

    GET_APPLICATION_OPTION_BOOL = Some(
        GenericDetour::<GetApplicationOptionBoolFunc>::new(
            o_GetApplicationOptionBool,
            GetApplicationOptionBool,
        )
        .unwrap(),
    );

    if cfg.sniper.skiplauncher {
        match GET_APPLICATION_OPTION_BOOL {
            Some(ref mut hook) => {
                hook.enable().unwrap();
                println!("[COBRA//HOOK] GetApplicationOptionBool hooked.");
            }
            None => println!("[COBRA//HOOK] Failed to hook GetApplicationOptionBool."),
        }
    }

    ZWEBSERVICE_CLIENT_MANAGER_CREATE = Some(
        GenericDetour::<ZWebServiceClientManagerCreateFunc>::new(
            o_ZWebServiceClientManagerCreate,
            ZWebServiceClientManagerCreate,
        )
        .unwrap(),
    );

    match ZWEBSERVICE_CLIENT_MANAGER_CREATE {
        Some(ref mut _hook) => {
            //hook.enable().unwrap();
            println!("[COBRA//HOOK] ZWebServiceClientManagerCreate not hooked (currently disabled in code).");
        }
        None => println!("[COBRA//HOOK] Failed to hook ZWebServiceClientManagerCreate."),
    }
}
