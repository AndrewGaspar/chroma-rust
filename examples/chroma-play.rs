use chroma::{Key, KeyboardCustomKeyEffectBuilder, KeyboardStaticEffectBuilder};
use rgb::RGB8;
use std::{error::Error, thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let example = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("static-cycle".to_string());

    match example.as_str() {
        "static-cycle" => static_cycle(),
        "grid" => grid(),
        "numpad" => numpad(),
        _ => panic!("Unrecognized example type"),
    }
}

fn static_cycle() -> Result<(), Box<dyn Error>> {
    let red = KeyboardStaticEffectBuilder::new(RGB8 {
        r: 0xff,
        g: 0x00,
        b: 0x00,
    })
    .build()?;

    let green = KeyboardStaticEffectBuilder::new(RGB8 {
        r: 0x00,
        g: 0xff,
        b: 0x00,
    })
    .build()?;

    let blue = KeyboardStaticEffectBuilder::new(RGB8 {
        r: 0x00,
        g: 0x00,
        b: 0xff,
    })
    .build()?;

    for effect in [red, green, blue].iter().cycle() {
        effect.set()?;

        sleep(Duration::from_secs(1));
    }

    unreachable!()
}

fn grid() -> Result<(), Box<dyn Error>> {
    loop {
        for row in 0..chroma::MAX_ROW {
            for column in 0..chroma::MAX_COLUMN {
                let effect = KeyboardCustomKeyEffectBuilder::new()
                    .set_position(
                        row,
                        column,
                        RGB8 {
                            r: 0xff,
                            g: 0xff,
                            b: 0xff,
                        },
                    )
                    .build()?;

                effect.set()?;

                sleep(Duration::from_millis(100));
            }
        }
    }
}

fn numpad() -> Result<(), Box<dyn Error>> {
    let nums = [
        Key::Row1,
        Key::Row2,
        Key::Row3,
        Key::Row4,
        Key::Row5,
        Key::Row6,
        Key::Row7,
        Key::Row8,
        Key::Row9,
    ];

    let effects: Vec<_> = nums
        .iter()
        .map(|key| {
            KeyboardCustomKeyEffectBuilder::new()
                .set_key(
                    *key,
                    RGB8 {
                        r: 0xff,
                        g: 0x00,
                        b: 0x00,
                    },
                )
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;

    for e in effects.iter().cycle() {
        e.set()?;
        sleep(Duration::from_millis(250));
    }

    unreachable!()
}
