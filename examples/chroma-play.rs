use chroma::{Effect, KeyboardEffect};
use rgb::RGB;
use std::{thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let red = Effect::keyboard(KeyboardEffect::Static(RGB {
        r: 0xff,
        g: 0x00,
        b: 0x00,
    }))?;

    let green = Effect::keyboard(KeyboardEffect::Static(RGB {
        r: 0x00,
        g: 0xff,
        b: 0x00,
    }))?;

    let blue = Effect::keyboard(KeyboardEffect::Static(RGB {
        r: 0x00,
        g: 0x00,
        b: 0xff,
    }))?;

    for effect in [red, green, blue].iter().cycle() {
        effect.set()?;

        sleep(Duration::from_secs(1));
    }

    Ok(())
}
