use std::{mem, path::PathBuf, ptr};

use libloading::Symbol;

mod error;
mod sys;

pub use error::ChromaError;

pub struct ChromaLibrary {
    sdk: *const libloading::Library,
    uninit_fn: Symbol<'static, sys::UnInitFn>,
}

impl ChromaLibrary {
    pub fn load() -> Result<Self, ChromaError> {
        let sdk_path = PathBuf::from(std::env::var_os("ProgramFiles").unwrap())
            .join("Razer Chroma SDK/bin/RzChromaSDK64.dll");

        let sdk = Box::leak(Box::new(libloading::Library::new(sdk_path)?));

        let init_fn: Symbol<sys::InitFn> = unsafe { sdk.get(b"Init\0")? };
        let uninit_fn = unsafe { sdk.get(b"UnInit\0")? };

        unsafe {
            init_fn().r()?;
        }

        Ok(Self {
            sdk: sdk as *const _,
            uninit_fn,
        })
    }
}

impl Drop for ChromaLibrary {
    fn drop(&mut self) {
        unsafe {
            mem::forget((*self.uninit_fn)());
            Box::from_raw(mem::replace(&mut self.sdk, ptr::null()) as *mut libloading::Library);
        }
    }
}
