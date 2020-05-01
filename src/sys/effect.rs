#![allow(non_camel_case_types)]

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// (Deprecated)
    CHROMA_WAVE,
    /// (Deprecated)
    CHROMA_SPECTRUMCYCLING,
    /// (Deprecated)
    CHROMA_BREATHING,
    /// (Deprecated)
    CHROMA_BLINKING,
    /// (Deprecated)
    CHROMA_REACTIVE,
    /// Static single color effect
    CHROMA_STATIC,
    /// Custom effect. For mice, please see Mouse::CHROMA_CUSTOM2
    CHROMA_CUSTOM,
    /// Reserved, do not use.
    CHROMA_RESERVED,
    /// Invalid effect
    CHROMA_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum KEYBOARD_EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// (Deprecated)
    CHROMA_BREATHING,
    /// Custom effect type.
    CHROMA_CUSTOM,
    /// (Deprecated)
    CHROMA_REACTIVE,
    /// Static single color effect
    CHROMA_STATIC,
    /// (Deprecated)
    CHROMA_SPECTRUMCYCLING,
    /// (Deprecated)
    CHROMA_WAVE,
    /// Reserved, do not use.
    CHROMA_RESERVED,
    /// Custom effects with keys
    CHROMA_CUSTOM_KEY,
    /// CUstom effects with keys using 8x24 grid.
    CHROMA_CUSTOM2,
    /// Invalid effect
    CHROMA_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum MOUSE_EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// (Deprecated)
    CHROMA_BLINKING,
    /// (Deprecated)
    CHROMA_BREATHING,
    /// (Deprecated)
    CHROMA_CUSTOM,
    /// (Deprecated)
    CHROMA_REACTIVE,
    /// (Deprecated)
    CHROMA_SPECTRUMCYCLING,
    /// Static single color effect
    CHROMA_STATIC,
    /// (Deprecated)
    CHROMA_WAVE,
    /// Custom effects using a virtual grid.
    CHROMA_CUSTOM2,
    /// Invalid effect
    CHROMA_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum HEADSET_EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// Static single color effect
    CHROMA_STATIC,
    /// (Deprecated)
    CHROMA_BREATHING,
    /// (Deprecated)
    CHROMA_SPECTRUMCYCLING,
    /// Custom effect type.
    CHROMA_CUSTOM,
    /// Invalid effect
    CHROMA_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum MOUSEPAD_EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// (Deprecated)
    CHROMA_BREATHING,
    /// Custom effect type.
    CHROMA_CUSTOM,
    /// (Deprecated)
    CHROMA_SPECTRUMCYCLING,
    /// Static single color effect
    CHROMA_STATIC,
    /// (Deprecated)
    CHROMA_WAVE,
    /// Custom effect with 20 virtual LEDs. First element starts from top-right and it goes
    /// clockwise full circle with 5 LEDs on each side.
    CHROMA_CUSTOM2,
    /// Invalid effect
    CHROMA_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum KEYPAD_EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// (Deprecated)
    CHROMA_BREATHING,
    /// Custom effect type.
    CHROMA_CUSTOM,
    /// (Deprecated)
    CHROMA_REACTIVE,
    /// (Deprecated)
    CHROMA_SPECTRUMCYCLING,
    /// Static single color effect
    CHROMA_STATIC,
    /// (Deprecated)
    CHROMA_WAVE,
    /// Invalid effect
    CHROMA_INVALID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum CHROMA_LINK_EFFECT_TYPE {
    /// No effect.
    CHROMA_NONE,
    /// Custom effect type.
    CHROMA_CUSTOM,
    /// Static single color effect
    CHROMA_STATIC,
    /// Invalid effect
    CHROMA_INVALID,
}
