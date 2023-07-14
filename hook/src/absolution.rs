#![allow(non_snake_case)]

use super::config::Config;
use super::glacier::{
    GetApplicationOptionBool, GetApplicationOptionBoolFunc, ZWebServiceClientManagerCreate,
    ZWebServiceClientManagerCreateFunc, GET_APPLICATION_OPTION_BOOL, WEBSERVICE_URL,
    ZWEBSERVICE_CLIENT_MANAGER_CREATE,
};

use retour::GenericDetour;
use std::mem::transmute;

pub unsafe fn init_absolution(cfg: Config) {
    WEBSERVICE_URL = cfg.hm5.url;

    let o_GetApplicationOptionBool: GetApplicationOptionBoolFunc = transmute(0x00612930 as usize);
    let o_ZWebServiceClientManagerCreate: ZWebServiceClientManagerCreateFunc =
        transmute(0x00BE22CF as usize);

    GET_APPLICATION_OPTION_BOOL = Some(
        GenericDetour::<GetApplicationOptionBoolFunc>::new(
            o_GetApplicationOptionBool,
            GetApplicationOptionBool,
        )
        .unwrap(),
    );

    if cfg.hm5.skiplauncher {
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
        Some(ref mut hook) => {
            hook.enable().unwrap();
            println!("[COBRA//HOOK] ZWebServiceClientManagerCreate hooked.");
        }
        None => println!("[COBRA//HOOK] Failed to hook ZWebServiceClientManagerCreate."),
    }
}
