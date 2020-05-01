use std::{mem, path::PathBuf, ptr};

use libloading::Symbol;

mod error;
mod sys;

pub use error::ChromaError;

#[allow(dead_code)]
pub struct ChromaLibrary {
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

impl ChromaLibrary {
    pub fn load() -> Result<Self, ChromaError> {
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
