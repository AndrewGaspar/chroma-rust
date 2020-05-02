#![allow(dead_code)]

use std::os::raw::c_void;

use winapi::shared::{
    guiddef::{IsEqualGUID, GUID},
    ntdef::LONG,
    windef::HWND,
};

mod effect;
mod key;

pub use effect::*;
pub use key::*;

pub type InitFn = unsafe extern "C" fn() -> RZRESULT;
pub type UnInitFn = unsafe extern "C" fn() -> RZRESULT;

pub type CreateEffectFn = unsafe extern "C" fn(
    device_id: RZDEVICEID,
    effect: EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type CreateKeyboardEffectFn = unsafe extern "C" fn(
    effect: KEYBOARD_EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type CreateMouseEffectFn = unsafe extern "C" fn(
    effect: MOUSE_EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type CreateHeadsetEffectFn = unsafe extern "C" fn(
    effect: HEADSET_EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type CreateMousepadEffectFn = unsafe extern "C" fn(
    effect: MOUSEPAD_EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type CreateKeypadEffectFn = unsafe extern "C" fn(
    effect: KEYPAD_EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type CreateChromaLinkEffectFn = unsafe extern "C" fn(
    effect: CHROMA_LINK_EFFECT_TYPE,
    param: PRZPARAM,
    effect_id: *mut RZEFFECTID,
) -> RZRESULT;

pub type DeleteEffectFn = unsafe extern "C" fn(id: RZEFFECTID) -> RZRESULT;
pub type SetEffectFn = unsafe extern "C" fn(id: RZEFFECTID) -> RZRESULT;
pub type RegisterEventNotificationFn = unsafe extern "C" fn(hwnd: HWND) -> RZRESULT;
pub type UnregisterEventNotificationFn = unsafe extern "C" fn() -> RZRESULT;
pub type QueryDeviceFn =
    unsafe extern "C" fn(id: RZDEVICEID, device_info: Option<&mut DEVICE_TYPE>) -> RZRESULT;

pub const MAX_ROW: usize = 6;
pub const MAX_COLUMN: usize = 22;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct RZDEVICEID(pub GUID);

impl PartialEq for RZDEVICEID {
    fn eq(&self, other: &Self) -> bool {
        IsEqualGUID(&self.0, &other.0)
    }
}
impl Eq for RZDEVICEID {}

pub type PRZPARAM = *mut c_void;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct RZEFFECTID(pub GUID);

impl PartialEq for RZEFFECTID {
    fn eq(&self, other: &Self) -> bool {
        IsEqualGUID(&self.0, &other.0)
    }
}
impl Eq for RZEFFECTID {}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum DEVICE_TYPE {
    DEVICE_KEYBOARD = 1,
    DEVICE_MOUSE = 2,
    DEVICE_HEADSET = 3,
    DEVICE_MOUSEPAD = 4,
    DEVICE_KEYPAD = 5,
    DEVICE_SYSTEM = 6,
    DEVICE_SPEAKERS = 7,
    DEVICE_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[must_use = "Don't forget to check RZRESULTs"]
#[repr(transparent)]
pub struct RZRESULT(pub LONG);

impl RZRESULT {
    pub fn r(self) -> Result<(), RZRESULT> {
        if self == RZRESULT_SUCCESS {
            Ok(())
        } else {
            Err(self)
        }
    }
}

pub const RZRESULT_INVALID: RZRESULT = RZRESULT(-1);
pub const RZRESULT_SUCCESS: RZRESULT = RZRESULT(0);
pub const RZRESULT_ACCESS_DENIED: RZRESULT = RZRESULT(5);
pub const RZRESULT_INVALID_HANDLE: RZRESULT = RZRESULT(6);
pub const RZRESULT_NOT_SUPPORTED: RZRESULT = RZRESULT(50);
pub const RZRESULT_INVALID_PARAMETER: RZRESULT = RZRESULT(87);
pub const RZRESULT_SERVICE_NOT_ACTIVE: RZRESULT = RZRESULT(1062);
pub const RZRESULT_SINGLE_INSTANCE_APP: RZRESULT = RZRESULT(1152);
pub const RZRESULT_DEVICE_NOT_CONNECTED: RZRESULT = RZRESULT(1167);
pub const RZRESULT_NOT_FOUND: RZRESULT = RZRESULT(1168);
pub const RZRESULT_REQUEST_ABORTED: RZRESULT = RZRESULT(1235);
pub const RZRESULT_ALREADY_INITIALIZED: RZRESULT = RZRESULT(1247);
pub const RZRESULT_RESOURCE_DISABLED: RZRESULT = RZRESULT(4309);
pub const RZRESULT_DEVICE_NOT_AVAILABLE: RZRESULT = RZRESULT(4319);
pub const RZRESULT_NOT_VALID_STATE: RZRESULT = RZRESULT(5023);
pub const RZRESULT_NO_MORE_ITEMS: RZRESULT = RZRESULT(259);
pub const RZRESULT_FAILED: RZRESULT = RZRESULT(2147500037i64 as i32);
