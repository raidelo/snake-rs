use crossterm::style::Color;

#[allow(unused)]
pub struct PaletteColors {
    pub background: Color,
    pub snake_body: Color,
    pub snake_head: Color,
    pub food: Color,
    pub borders: Color,
}

impl PaletteColors {
    pub const CLASSIC: Self = Self {
        background: Color::Rgb {
            r: 0x2d,
            g: 0x37,
            b: 0x2e,
        },
        snake_body: Color::Rgb {
            r: 0x0f,
            g: 0x38,
            b: 0x0f,
        },
        snake_head: Color::Rgb {
            r: 0x00,
            g: 0x2b,
            b: 0x00,
        },
        food: Color::Rgb {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        },
        borders: Color::Rgb {
            r: 0x8b,
            g: 0xac,
            b: 0x0f,
        },
    };
    pub const NEON: Self = Self {
        background: Color::Rgb {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        },
        snake_body: Color::Rgb {
            r: 0x00,
            g: 0xff,
            b: 0xcc,
        },
        snake_head: Color::Rgb {
            r: 0xff,
            g: 0x00,
            b: 0xff,
        },
        food: Color::Rgb {
            r: 0xff,
            g: 0x33,
            b: 0x66,
        },
        borders: Color::Rgb {
            r: 0xff,
            g: 0xff,
            b: 0x00,
        },
    };
    pub const ORGANIC: Self = Self {
        background: Color::Rgb {
            r: 0x28,
            g: 0x28,
            b: 0x28,
        },
        snake_body: Color::Rgb {
            r: 0x8e,
            g: 0xc0,
            b: 0x7c,
        },
        snake_head: Color::Rgb {
            r: 0xb8,
            g: 0xbb,
            b: 0x26,
        },
        food: Color::Rgb {
            r: 0xfb,
            g: 0x49,
            b: 0x34,
        },
        borders: Color::Rgb {
            r: 0x83,
            g: 0xa5,
            b: 0x98,
        },
    };
}

#[allow(unused)]
#[derive(Clone)]
pub enum PaletteStyle {
    Classic,
    Neon,
    Organic,
}

impl PaletteStyle {
    pub fn palette(&self) -> PaletteColors {
        match self {
            Self::Classic => PaletteColors::CLASSIC,
            Self::Neon => PaletteColors::NEON,
            Self::Organic => PaletteColors::ORGANIC,
        }
    }
}
