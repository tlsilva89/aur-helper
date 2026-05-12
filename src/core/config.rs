use iced::theme::Palette;
use iced::{Color, Theme};

pub const BG_PRIMARY: Color = Color { r: 0.04, g: 0.04, b: 0.09, a: 1.0 };
pub const BG_SECONDARY: Color = Color { r: 0.07, g: 0.07, b: 0.15, a: 1.0 };
pub const BG_CARD: Color = Color { r: 0.09, g: 0.09, b: 0.19, a: 1.0 };
pub const BG_HOVER: Color = Color { r: 0.12, g: 0.12, b: 0.25, a: 1.0 };
pub const BORDER_DIM: Color = Color { r: 0.15, g: 0.15, b: 0.32, a: 1.0 };
pub const BORDER_ACTIVE: Color = Color { r: 0.0, g: 0.85, b: 1.0, a: 0.6 };

pub const CYAN: Color = Color { r: 0.0, g: 0.95, b: 1.0, a: 1.0 };
pub const CYAN_DIM: Color = Color { r: 0.0, g: 0.95, b: 1.0, a: 0.15 };
pub const PURPLE: Color = Color { r: 0.48, g: 0.19, b: 1.0, a: 1.0 };
pub const PINK: Color = Color { r: 1.0, g: 0.07, b: 0.57, a: 1.0 };
pub const NEON_GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.53, a: 1.0 };

pub const TEXT_PRIMARY: Color = Color { r: 0.91, g: 0.91, b: 1.0, a: 1.0 };
pub const TEXT_SECONDARY: Color = Color { r: 0.55, g: 0.55, b: 0.75, a: 1.0 };
pub const TEXT_MUTED: Color = Color { r: 0.30, g: 0.30, b: 0.50, a: 1.0 };

pub const SUCCESS: Color = Color { r: 0.0, g: 1.0, b: 0.53, a: 1.0 };
pub const WARNING: Color = Color { r: 1.0, g: 0.80, b: 0.0, a: 1.0 };
pub const DANGER: Color = Color { r: 1.0, g: 0.20, b: 0.40, a: 1.0 };

pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

pub fn cyberpunk_theme() -> Theme {
    Theme::custom(
        "Cyberpunk".to_string(),
        Palette {
            background: BG_PRIMARY,
            text: TEXT_PRIMARY,
            primary: CYAN,
            success: SUCCESS,
            danger: DANGER,
        },
    )
}
