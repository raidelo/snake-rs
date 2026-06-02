use crossterm::style::Color;

#[allow(unused)]
#[derive(Debug)]
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
            r: 0x9b,
            g: 0xbc,
            b: 0x0f,
        },
        snake_body: Color::Rgb {
            r: 0x30,
            g: 0x62,
            b: 0x30,
        },
        snake_head: Color::Rgb {
            r: 0x0f,
            g: 0x38,
            b: 0x0f,
        },
        food: Color::Rgb {
            r: 0x8b,
            g: 0x00,
            b: 0x00,
        },
        borders: Color::Rgb {
            r: 0x0f,
            g: 0x38,
            b: 0x0f,
        },
    };

    pub const NEON_V1: Self = Self {
        background: Color::Rgb {
            r: 0x05,
            g: 0x05,
            b: 0x10,
        },
        snake_body: Color::Rgb {
            r: 0x00,
            g: 0xe5,
            b: 0xff,
        },
        snake_head: Color::Rgb {
            r: 0xff,
            g: 0x00,
            b: 0x90,
        },
        food: Color::Rgb {
            r: 0xff,
            g: 0xe0,
            b: 0x00,
        },
        borders: Color::Rgb {
            r: 0x7b,
            g: 0x2f,
            b: 0xff,
        },
    };

    pub const NEON_V2: Self = Self {
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

    pub const ORGANIC_V1: Self = Self {
        background: Color::Rgb {
            r: 0x1d,
            g: 0x20,
            b: 0x1a,
        },
        snake_body: Color::Rgb {
            r: 0x6a,
            g: 0x99,
            b: 0x55,
        },
        snake_head: Color::Rgb {
            r: 0xd4,
            g: 0xe8,
            b: 0x57,
        },
        food: Color::Rgb {
            r: 0xe8,
            g: 0x3a,
            b: 0x1f,
        },
        borders: Color::Rgb {
            r: 0x4a,
            g: 0x6e,
            b: 0x3c,
        },
    };

    pub const ORGANIC_V2: Self = Self {
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

    pub const DRACULA: Self = Self {
        background: Color::Rgb {
            r: 0x28,
            g: 0x2a,
            b: 0x36,
        },
        snake_body: Color::Rgb {
            r: 0x50,
            g: 0xfa,
            b: 0x7b,
        },
        snake_head: Color::Rgb {
            r: 0xff,
            g: 0x79,
            b: 0xc6,
        },
        food: Color::Rgb {
            r: 0xff,
            g: 0xb8,
            b: 0x6c,
        },
        borders: Color::Rgb {
            r: 0xbd,
            g: 0x93,
            b: 0xf9,
        },
    };

    pub const OCEAN: Self = Self {
        background: Color::Rgb {
            r: 0x00,
            g: 0x1a,
            b: 0x33,
        },
        snake_body: Color::Rgb {
            r: 0x00,
            g: 0x99,
            b: 0xcc,
        },
        snake_head: Color::Rgb {
            r: 0x00,
            g: 0xd4,
            b: 0xff,
        },
        food: Color::Rgb {
            r: 0xff,
            g: 0x6b,
            b: 0x35,
        },
        borders: Color::Rgb {
            r: 0x00,
            g: 0x55,
            b: 0x88,
        },
    };

    pub const MATRIX: Self = Self {
        background: Color::Rgb {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        },
        snake_body: Color::Rgb {
            r: 0x00,
            g: 0xcc,
            b: 0x00,
        },
        snake_head: Color::Rgb {
            r: 0x00,
            g: 0xff,
            b: 0x00,
        },
        food: Color::Rgb {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        },
        borders: Color::Rgb {
            r: 0x00,
            g: 0x66,
            b: 0x00,
        },
    };

    pub const SUNSET: Self = Self {
        background: Color::Rgb {
            r: 0x1a,
            g: 0x0a,
            b: 0x00,
        },
        snake_body: Color::Rgb {
            r: 0xff,
            g: 0x6b,
            b: 0x00,
        },
        snake_head: Color::Rgb {
            r: 0xff,
            g: 0xd7,
            b: 0x00,
        },
        food: Color::Rgb {
            r: 0xff,
            g: 0x00,
            b: 0x40,
        },
        borders: Color::Rgb {
            r: 0xcc,
            g: 0x44,
            b: 0x00,
        },
    };

    pub const NORD: Self = Self {
        background: Color::Rgb {
            r: 0x2e,
            g: 0x34,
            b: 0x40,
        },
        snake_body: Color::Rgb {
            r: 0xa3,
            g: 0xbe,
            b: 0x8c,
        },
        snake_head: Color::Rgb {
            r: 0xeb,
            g: 0xcb,
            b: 0x8b,
        },
        food: Color::Rgb {
            r: 0xbf,
            g: 0x61,
            b: 0x6a,
        },
        borders: Color::Rgb {
            r: 0x81,
            g: 0xa1,
            b: 0xc1,
        },
    };

    pub const RETRO_CGA: Self = Self {
        background: Color::Rgb {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        },
        snake_body: Color::Rgb {
            r: 0x00,
            g: 0xaa,
            b: 0xaa,
        },
        snake_head: Color::Rgb {
            r: 0x55,
            g: 0xff,
            b: 0xff,
        },
        food: Color::Rgb {
            r: 0xaa,
            g: 0x00,
            b: 0xaa,
        },
        borders: Color::Rgb {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        },
    };
}

#[allow(unused)]
#[derive(Debug)]
pub enum PaletteStyle {
    Classic,
    NeonV1,
    NeonV2,
    OrganicV1,
    OrganicV2,
    Dracula,
    Ocean,
    Matrix,
    Sunset,
    Nord,
    RetroCga,
}

impl PaletteStyle {
    pub fn palette(&self) -> PaletteColors {
        match self {
            Self::Classic => PaletteColors::CLASSIC,
            Self::NeonV1 => PaletteColors::NEON_V1,
            Self::NeonV2 => PaletteColors::NEON_V2,
            Self::OrganicV1 => PaletteColors::ORGANIC_V1,
            Self::OrganicV2 => PaletteColors::ORGANIC_V2,
            Self::Dracula => PaletteColors::DRACULA,
            Self::Ocean => PaletteColors::OCEAN,
            Self::Matrix => PaletteColors::MATRIX,
            Self::Sunset => PaletteColors::SUNSET,
            Self::Nord => PaletteColors::NORD,
            Self::RetroCga => PaletteColors::RETRO_CGA,
        }
    }
}
