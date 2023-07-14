#![allow(non_snake_case)]

use super::config::Config;
use super::glacier::{
    GetApplicationOptionBool, GetApplicationOptionBoolFunc, ZWebServiceClientManagerCreate,
    ZWebServiceClientManagerCreateFunc, GET_APPLICATION_OPTION_BOOL, WEBSERVICE_URL,
    ZWEBSERVICE_CLIENT_MANAGER_CREATE,
};

use retour::GenericDetour;
use std::{ffi::CString, mem::transmute};
use substring::Substring;
use tinyrlibc::{strcpy, CChar};

static mut PATCHED: bool = false;
static mut URL_PATCH_ENABLED: bool = false;

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
    let o_GetApplicationOptionBool: GetApplicationOptionBoolFunc = transmute(0x004984C0_usize);
    let o_ZWebServiceClientManagerCreate: ZWebServiceClientManagerCreateFunc =
        transmute(0x00C043D1_usize);

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
