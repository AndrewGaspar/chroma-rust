use std::{mem, path::PathBuf, ptr, sync::RwLock};

use libloading::Symbol;

mod effect;
mod error;
mod key;
pub mod sys;

pub use effect::*;
pub use error::{ChromaError, Result};
pub use key::*;

pub const MAX_COLUMN: u8 = sys::MAX_COLUMN as u8;
pub const MAX_ROW: u8 = sys::MAX_ROW as u8;

lazy_static::lazy_static! {
    static ref CHROMA_LIBRARY: RwLock<Option<ChromaLibrary>> = RwLock::default();
}

pub(crate) unsafe fn lib() -> Result<&'static ChromaLibrary> {
    match CHROMA_LIBRARY.try_read() {
        Ok(guard) => {
            if let Some(lib) = guard.as_ref() {
                // It's safe to erase the lifetime because once the Library is loaded, we never mutate
                // it again.
                return Ok(&*(lib as *const _));
            }
        }
        Err(std::sync::TryLockError::Poisoned(_)) => panic!(),
        _ => {}
    }

    // Initialize CHROMA_LIBRARY
    {
        let mut lib = CHROMA_LIBRARY.write().unwrap();
        if lib.is_none() {
            *lib = Some(ChromaLibrary::load()?);
        }
    }

    // It's safe to erase the lifetime because once the Library is loaded, we never mutate it again.
    let lib = CHROMA_LIBRARY.read().unwrap();
    let lib = lib.as_ref().unwrap();
    Ok(&*(lib as *const _))
}

#[allow(dead_code)]
struct ChromaLibrary {
    sdk: *const libloading::Library,
    uninit_fn: Symbol<'static, sys::UnInitFn>,
    create_effect_fn: Symbol<'static, sys::CreateEffectFn>,
    create_keyboard_effect_fn: Symbol<'static, sys::CreateKeyboardEffectFn>,
    create_mouse_effect_fn: Symbol<'static, sys::CreateMouseEffectFn>,
    create_headset_effect_fn: Symbol<'static, sys::CreateHeadsetEffectFn>,
    create_mousepad_effect_fn: Symbol<'static, sys::CreateMousepadEffectFn>,
    create_keypad_effect_fn: Symbol<'static, sys::CreateKeypadEffectFn>,
    create_chroma_link_effect_fn: Symbol<'static, sys::CreateChromaLinkEffectFn>,
    delete_effect_fn: Symbol<'static, sys::DeleteEffectFn>,
    set_effect_fn: Symbol<'static, sys::SetEffectFn>,
    register_event_notification_fn: Symbol<'static, sys::RegisterEventNotificationFn>,
    unregister_event_notification_fn: Symbol<'static, sys::UnregisterEventNotificationFn>,
    query_device_fn: Symbol<'static, sys::QueryDeviceFn>,
}

unsafe impl Send for ChromaLibrary {}
unsafe impl Sync for ChromaLibrary {}

impl ChromaLibrary {
    fn load() -> Result<Self> {
        let sdk_path = PathBuf::from(std::env::var_os("ProgramFiles").unwrap())
            .join("Razer Chroma SDK/bin/RzChromaSDK64.dll");

        let sdk = Box::leak(Box::new(libloading::Library::new(sdk_path)?));

        let init_fn: Symbol<sys::InitFn> = unsafe { sdk.get(b"Init\0")? };

        unsafe {
            init_fn().r()?;
        }

        let uninit_fn = unsafe { sdk.get(b"UnInit\0")? };
        let create_effect_fn = unsafe { sdk.get(b"CreateEffect\0")? };
        let create_keyboard_effect_fn = unsafe { sdk.get(b"CreateKeyboardEffect\0")? };
        let create_mouse_effect_fn = unsafe { sdk.get(b"CreateMouseEffect\0")? };
        let create_headset_effect_fn = unsafe { sdk.get(b"CreateHeadsetEffect\0")? };
        let create_mousepad_effect_fn = unsafe { sdk.get(b"CreateMousepadEffect\0")? };
        let create_keypad_effect_fn = unsafe { sdk.get(b"CreateKeypadEffect\0")? };
        let create_chroma_link_effect_fn = unsafe { sdk.get(b"CreateChromaLinkEffect\0")? };
        let delete_effect_fn = unsafe { sdk.get(b"DeleteEffect\0")? };
        let set_effect_fn = unsafe { sdk.get(b"SetEffect\0")? };
        let register_event_notification_fn = unsafe { sdk.get(b"RegisterEventNotification\0")? };
        let unregister_event_notification_fn =
            unsafe { sdk.get(b"UnregisterEventNotification\0")? };
        let query_device_fn = unsafe { sdk.get(b"QueryDevice\0")? };

        Ok(Self {
            sdk: sdk as *const _,
            uninit_fn,
            create_effect_fn,
            create_keyboard_effect_fn,
            create_mouse_effect_fn,
            create_headset_effect_fn,
            create_mousepad_effect_fn,
            create_keypad_effect_fn,
            create_chroma_link_effect_fn,
            delete_effect_fn,
            set_effect_fn,
            register_event_notification_fn,
            unregister_event_notification_fn,
            query_device_fn,
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
