use quick_error::quick_error;
use winapi::shared::ntdef::LONG;

use crate::sys::{self, RZRESULT};

pub type Result<T> = std::result::Result<T, ChromaError>;

quick_error! {
    #[derive(Debug)]
    pub enum ChromaError {
        RzInvalid {}
        RzAccessDenied {}
        RzInvalidHandle {}
        RzNotSupported {}
        RzInvalidParameter {}
        RzServiceNotActive {}
        RzSingleInstanceApp {}
        RzDeviceNotConnected {}
        RzNotFound {}
        RzRequestAborted {}
        RzAlreadyInitialized {}
        RzResourceDisabled {}
        RzDeviceNotAvailable {}
        RzNotValidState {}
        RzNoMoreItems {}
        RzFailed {}
        WinError(hr: LONG) {}
        LoadError(err: libloading::Error) {
            from()
            description(err.description())
        }
    }
}

impl From<RZRESULT> for ChromaError {
    fn from(e: RZRESULT) -> Self {
        match e {
            sys::RZRESULT_INVALID => ChromaError::RzInvalid,
            sys::RZRESULT_ACCESS_DENIED => ChromaError::RzAccessDenied,
            sys::RZRESULT_INVALID_HANDLE => ChromaError::RzInvalidHandle,
            sys::RZRESULT_NOT_SUPPORTED => ChromaError::RzNotSupported,
            sys::RZRESULT_INVALID_PARAMETER => ChromaError::RzInvalidParameter,
            sys::RZRESULT_SERVICE_NOT_ACTIVE => ChromaError::RzServiceNotActive,
            sys::RZRESULT_SINGLE_INSTANCE_APP => ChromaError::RzSingleInstanceApp,
            sys::RZRESULT_DEVICE_NOT_CONNECTED => ChromaError::RzDeviceNotConnected,
            sys::RZRESULT_NOT_FOUND => ChromaError::RzNotFound,
            sys::RZRESULT_REQUEST_ABORTED => ChromaError::RzRequestAborted,
            sys::RZRESULT_ALREADY_INITIALIZED => ChromaError::RzAlreadyInitialized,
            sys::RZRESULT_RESOURCE_DISABLED => ChromaError::RzResourceDisabled,
            sys::RZRESULT_DEVICE_NOT_AVAILABLE => ChromaError::RzDeviceNotAvailable,
            sys::RZRESULT_NOT_VALID_STATE => ChromaError::RzNotValidState,
            sys::RZRESULT_NO_MORE_ITEMS => ChromaError::RzNoMoreItems,
            sys::RZRESULT_FAILED => ChromaError::RzFailed,
            x => ChromaError::WinError(x.0),
        }
    }
}
