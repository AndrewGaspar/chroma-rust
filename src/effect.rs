use rgb::RGB;

pub struct Effect(pub(crate) super::sys::RZEFFECTID);

// impl Drop for Effect {

// }

pub enum KeyboardEffect {
    Static(RGB<u8>),
}
