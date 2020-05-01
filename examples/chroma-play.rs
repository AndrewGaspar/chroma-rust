use chroma::{ChromaLibrary, KeyboardEffect};
use rgb::RGB;
use std::{thread::sleep, time::Duration};

const RED: RGB<u8> = RGB {
    r: 0xff,
    g: 0x00,
    b: 0x00,
};
const GREEN: RGB<u8> = RGB {
    r: 0x00,
    g: 0xff,
    b: 0x00,
};
const BLUE: RGB<u8> = RGB {
    r: 0x00,
    g: 0x00,
    b: 0xff,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lib = ChromaLibrary::load()?;

    for c in [RED, BLUE, GREEN].iter().cycle() {
        let effect = lib.create_keyboard_effect(KeyboardEffect::Static(*c))?;
        lib.set_effect(&effect)?;
        lib.delete_effect(effect)?;

        sleep(Duration::from_secs(1));
    }

    Ok(())
}
