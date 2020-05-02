use std::mem::MaybeUninit;

use rgb::RGB8;
use winapi::shared::windef::COLORREF;

use crate::{lib, sys, Result};

pub struct Effect(pub(crate) super::sys::RZEFFECTID);

impl Effect {
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

#[derive(Copy, Clone, Default)]
pub struct KeyboardStaticEffectBuilder {
    color: RGB8,
}

impl KeyboardStaticEffectBuilder {
    pub fn new(color: RGB8) -> Self {
        Self { color }
    }

    pub fn set_color<'a>(&'a mut self, color: RGB8) -> &'a mut Self {
        self.color = color;
        self
    }

    pub fn build(&self) -> Result<Effect> {
        let mut effect_type = sys::keyboard::STATIC_EFFECT_TYPE {
            color: colorref_from_rgb(self.color),
        };

        unsafe {
            let mut effect_id = MaybeUninit::uninit();

            (*lib()?.create_keyboard_effect_fn)(
                sys::KEYBOARD_EFFECT_TYPE::CHROMA_STATIC,
                &mut effect_type as *mut _ as *mut _,
                effect_id.as_mut_ptr(),
            )
            .r()?;

            Ok(Effect(effect_id.assume_init()))
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct KeyboardCustomKeyEffectBuilder {
    effect_type: sys::keyboard::CUSTOM_KEY_EFFECT_TYPE,
}

impl KeyboardCustomKeyEffectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear<'a>(&'a mut self) -> &'a mut Self {
        *self = Default::default();
        self
    }

    pub fn position(&self, i: u8, j: u8) -> RGB8 {
        rgb_from_colorref(self.effect_type.color[i as usize][j as usize])
    }

    pub fn set_position<'a>(&'a mut self, row: u8, column: u8, color: RGB8) -> &'a mut Self {
        let (row, column) = (row as usize, column as usize);

        self.effect_type.color[row][column] = colorref_from_rgb(color);
        self.effect_type.key[row][column] &= 0xffffff;

        self
    }

    pub fn key(&self, key: crate::Key) -> RGB8 {
        rgb_from_colorref(
            self.effect_type.key[key.row() as usize][key.column() as usize] & 0xffffff,
        )
    }

    pub fn set_key<'a>(&'a mut self, key: crate::Key, color: RGB8) -> &'a mut Self {
        let (row, column) = (key.row() as usize, key.column() as usize);
        self.effect_type.key[row][column] = colorref_from_rgb(color) | 0x1000000;
        self
    }

    pub fn build(&self) -> Result<Effect> {
        let mut effect_type = self.effect_type;

        let mut effect_id = MaybeUninit::uninit();
        unsafe {
            (*lib()?.create_keyboard_effect_fn)(
                sys::KEYBOARD_EFFECT_TYPE::CHROMA_CUSTOM_KEY,
                &mut effect_type as *mut _ as *mut _,
                effect_id.as_mut_ptr(),
            )
            .r()?;

            Ok(Effect(effect_id.assume_init()))
        }
    }
}

fn rgb_from_colorref(color: COLORREF) -> RGB8 {
    RGB8 {
        r: ((color & 0xff0000) >> 16) as u8,
        g: ((color & 0xff00) >> 8) as u8,
        b: (color & 0xff) as u8,
    }
}

fn colorref_from_rgb(rgb: RGB8) -> COLORREF {
    (rgb.b as u32) << 16 | (rgb.g as u32) << 8 | rgb.r as u32
}
