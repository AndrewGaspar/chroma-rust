use winapi::shared::ntdef::LONG;

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

pub type InitFn = unsafe extern "C" fn() -> RZRESULT;
pub type UnInitFn = unsafe extern "C" fn() -> RZRESULT;
