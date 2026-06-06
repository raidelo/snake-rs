use crossterm::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct WindowPalette {
    pub background: Color,
    pub borders: Color,
    pub score_text: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct SnakePalette {
    pub body: Color,
    pub head: Color,
    pub body_inv: Color,
    pub head_inv: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct FoodPalette {
    pub food: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub window: WindowPalette,
    pub snake: SnakePalette,
    pub food: FoodPalette,
}

impl Palette {
    pub const CLASSIC: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x9b,
                g: 0xbc,
                b: 0x0f,
            },
            borders: Color::Rgb {
                r: 0x0f,
                g: 0x38,
                b: 0x0f,
            },
            score_text: Color::Rgb {
                r: 0x0f,
                g: 0x38,
                b: 0x0f,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x30,
                g: 0x62,
                b: 0x30,
            },
            head: Color::Rgb {
                r: 0x0f,
                g: 0x38,
                b: 0x0f,
            },
            body_inv: Color::Rgb {
                r: 0x65,
                g: 0x8f,
                b: 0x1f,
            },
            head_inv: Color::Rgb {
                r: 0x54,
                g: 0x7a,
                b: 0x0f,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0x8b,
                g: 0x00,
                b: 0x00,
            },
        },
    };

    pub const NEON_V1: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x05,
                g: 0x05,
                b: 0x10,
            },
            borders: Color::Rgb {
                r: 0x7b,
                g: 0x2f,
                b: 0xff,
            },
            score_text: Color::Rgb {
                r: 0xff,
                g: 0xe0,
                b: 0x00,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x00,
                g: 0xe5,
                b: 0xff,
            },
            head: Color::Rgb {
                r: 0xff,
                g: 0x00,
                b: 0x90,
            },
            body_inv: Color::Rgb {
                r: 0x00,
                g: 0x75,
                b: 0x88,
            },
            head_inv: Color::Rgb {
                r: 0x80,
                g: 0x00,
                b: 0x50,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xff,
                g: 0xe0,
                b: 0x00,
            },
        },
    };

    pub const NEON_V2: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x00,
                g: 0x00,
                b: 0x00,
            },
            borders: Color::Rgb {
                r: 0xff,
                g: 0xff,
                b: 0x00,
            },
            score_text: Color::Rgb {
                r: 0x00,
                g: 0xff,
                b: 0xcc,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x00,
                g: 0xff,
                b: 0xcc,
            },
            head: Color::Rgb {
                r: 0xff,
                g: 0x00,
                b: 0xff,
            },
            body_inv: Color::Rgb {
                r: 0x00,
                g: 0x66,
                b: 0x55,
            },
            head_inv: Color::Rgb {
                r: 0x66,
                g: 0x00,
                b: 0x66,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xff,
                g: 0x33,
                b: 0x66,
            },
        },
    };

    pub const ORGANIC_V1: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x1d,
                g: 0x20,
                b: 0x1a,
            },
            borders: Color::Rgb {
                r: 0x4a,
                g: 0x6e,
                b: 0x3c,
            },
            score_text: Color::Rgb {
                r: 0xd4,
                g: 0xe8,
                b: 0x57,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x6a,
                g: 0x99,
                b: 0x55,
            },
            head: Color::Rgb {
                r: 0xd4,
                g: 0xe8,
                b: 0x57,
            },
            body_inv: Color::Rgb {
                r: 0x43,
                g: 0x5c,
                b: 0x37,
            },
            head_inv: Color::Rgb {
                r: 0x78,
                g: 0x84,
                b: 0x38,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xe8,
                g: 0x3a,
                b: 0x1f,
            },
        },
    };

    pub const ORGANIC_V2: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x28,
                g: 0x28,
                b: 0x28,
            },
            borders: Color::Rgb {
                r: 0x83,
                g: 0xa5,
                b: 0x98,
            },
            score_text: Color::Rgb {
                r: 0xb8,
                g: 0xbb,
                b: 0x26,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x8e,
                g: 0xc0,
                b: 0x7c,
            },
            head: Color::Rgb {
                r: 0xb8,
                g: 0xbb,
                b: 0x26,
            },
            body_inv: Color::Rgb {
                r: 0x5b,
                g: 0x74,
                b: 0x52,
            },
            head_inv: Color::Rgb {
                r: 0x70,
                g: 0x72,
                b: 0x27,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xfb,
                g: 0x49,
                b: 0x34,
            },
        },
    };

    pub const DRACULA: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x28,
                g: 0x2a,
                b: 0x36,
            },
            borders: Color::Rgb {
                r: 0xbd,
                g: 0x93,
                b: 0xf9,
            },
            score_text: Color::Rgb {
                r: 0xf8,
                g: 0xf8,
                b: 0xf2,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x50,
                g: 0xfa,
                b: 0x7b,
            },
            head: Color::Rgb {
                r: 0xff,
                g: 0x79,
                b: 0xc6,
            },
            body_inv: Color::Rgb {
                r: 0x3c,
                g: 0x92,
                b: 0x58,
            },
            head_inv: Color::Rgb {
                r: 0x94,
                g: 0x52,
                b: 0x7e,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xff,
                g: 0xb8,
                b: 0x6c,
            },
        },
    };

    pub const OCEAN: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x00,
                g: 0x1a,
                b: 0x33,
            },
            borders: Color::Rgb {
                r: 0x00,
                g: 0x55,
                b: 0x88,
            },
            score_text: Color::Rgb {
                r: 0xff,
                g: 0xff,
                b: 0xff,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x00,
                g: 0x99,
                b: 0xcc,
            },
            head: Color::Rgb {
                r: 0x00,
                g: 0xd4,
                b: 0xff,
            },
            body_inv: Color::Rgb {
                r: 0x00,
                g: 0x57,
                b: 0x80,
            },
            head_inv: Color::Rgb {
                r: 0x00,
                g: 0x77,
                b: 0x99,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xff,
                g: 0x6b,
                b: 0x35,
            },
        },
    };

    pub const MATRIX: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x00,
                g: 0x00,
                b: 0x00,
            },
            borders: Color::Rgb {
                r: 0x00,
                g: 0x66,
                b: 0x00,
            },
            score_text: Color::Rgb {
                r: 0x00,
                g: 0xff,
                b: 0x00,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x00,
                g: 0xcc,
                b: 0x00,
            },
            head: Color::Rgb {
                r: 0x00,
                g: 0xff,
                b: 0x00,
            },
            body_inv: Color::Rgb {
                r: 0x00,
                g: 0x55,
                b: 0x00,
            },
            head_inv: Color::Rgb {
                r: 0x00,
                g: 0x77,
                b: 0x00,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xff,
                g: 0xff,
                b: 0xff,
            },
        },
    };

    pub const SUNSET: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x1a,
                g: 0x0a,
                b: 0x00,
            },
            borders: Color::Rgb {
                r: 0xcc,
                g: 0x44,
                b: 0x00,
            },
            score_text: Color::Rgb {
                r: 0xff,
                g: 0xd7,
                b: 0x00,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0xff,
                g: 0x6b,
                b: 0x00,
            },
            head: Color::Rgb {
                r: 0xff,
                g: 0xd7,
                b: 0x00,
            },
            body_inv: Color::Rgb {
                r: 0x8d,
                g: 0x3c,
                b: 0x00,
            },
            head_inv: Color::Rgb {
                r: 0x8d,
                g: 0x77,
                b: 0x00,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xff,
                g: 0x00,
                b: 0x40,
            },
        },
    };

    pub const NORD: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x2e,
                g: 0x34,
                b: 0x40,
            },
            borders: Color::Rgb {
                r: 0x81,
                g: 0xa1,
                b: 0xc1,
            },
            score_text: Color::Rgb {
                r: 0xe5,
                g: 0xe9,
                b: 0xf0,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0xa3,
                g: 0xbe,
                b: 0x8c,
            },
            head: Color::Rgb {
                r: 0xeb,
                g: 0xcb,
                b: 0x8b,
            },
            body_inv: Color::Rgb {
                r: 0x68,
                g: 0x79,
                b: 0x66,
            },
            head_inv: Color::Rgb {
                r: 0x8d,
                g: 0x7f,
                b: 0x66,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xbf,
                g: 0x61,
                b: 0x6a,
            },
        },
    };

    pub const RETRO_CGA: Self = Self {
        window: WindowPalette {
            background: Color::Rgb {
                r: 0x00,
                g: 0x00,
                b: 0x00,
            },
            borders: Color::Rgb {
                r: 0xff,
                g: 0xff,
                b: 0xff,
            },
            score_text: Color::Rgb {
                r: 0xaa,
                g: 0x00,
                b: 0xaa,
            },
        },
        snake: SnakePalette {
            body: Color::Rgb {
                r: 0x00,
                g: 0xaa,
                b: 0xaa,
            },
            head: Color::Rgb {
                r: 0x55,
                g: 0xff,
                b: 0xff,
            },
            body_inv: Color::Rgb {
                r: 0x00,
                g: 0x55,
                b: 0x55,
            },
            head_inv: Color::Rgb {
                r: 0x2a,
                g: 0x88,
                b: 0x88,
            },
        },
        food: FoodPalette {
            food: Color::Rgb {
                r: 0xaa,
                g: 0x00,
                b: 0xaa,
            },
        },
    };
}

#[derive(Debug, Clone, Copy)]
pub enum Theme {
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

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            Self::Classic => Palette::CLASSIC,
            Self::NeonV1 => Palette::NEON_V1,
            Self::NeonV2 => Palette::NEON_V2,
            Self::OrganicV1 => Palette::ORGANIC_V1,
            Self::OrganicV2 => Palette::ORGANIC_V2,
            Self::Dracula => Palette::DRACULA,
            Self::Ocean => Palette::OCEAN,
            Self::Matrix => Palette::MATRIX,
            Self::Sunset => Palette::SUNSET,
            Self::Nord => Palette::NORD,
            Self::RetroCga => Palette::RETRO_CGA,
        }
    }
}
