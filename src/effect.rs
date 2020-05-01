use std::mem::MaybeUninit;

use rgb::RGB8;

use crate::{lib, sys, Result};

pub struct Effect(pub(crate) super::sys::RZEFFECTID);

impl Effect {
    pub fn keyboard(effect: KeyboardEffect) -> Result<Self> {
        match effect {
            KeyboardEffect::Static(rgb) => {
                let mut effect_type = sys::KEYBOARD_STATIC_EFFECT_TYPE {
                    color: (rgb.r as u32) << 16 | (rgb.g as u32) << 8 | rgb.b as u32,
                };

                let mut effect_id = MaybeUninit::uninit();
                unsafe {
                    (*lib()?.create_keyboard_effect_fn)(
                        sys::KEYBOARD_EFFECT_TYPE::CHROMA_STATIC,
                        // &mut effect_type as *mut _ as *mut _,
                        &mut effect_type as *mut _ as *mut _,
                        effect_id.as_mut_ptr(),
                    )
                    .r()?;

                    Ok(Effect(effect_id.assume_init()))
                }
            }
        }
    }

    pub fn set(&self) -> Result<()> {
        unsafe {
            (*lib()?.set_effect_fn)(self.0).r()?;
        }

        Ok(())
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        unsafe {
            let lib = if let Ok(lib) = lib() {
                lib
            } else {
                return;
            };
            std::mem::forget((*lib.delete_effect_fn)(self.0));
        }
    }
}

pub enum KeyboardEffect {
    Static(RGB8),
}
