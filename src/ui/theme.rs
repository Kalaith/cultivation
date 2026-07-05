use macroquad::prelude::*;

pub const PRIMARY: Color = Color::new(0.86, 0.68, 0.32, 1.0); // Aged gold
pub const SECONDARY: Color = Color::new(0.34, 0.72, 0.63, 1.0); // Jade
pub const ACCENT: Color = Color::new(0.72, 0.18, 0.16, 1.0); // Cinnabar

pub const PANEL_BG: Color = Color::new(0.07, 0.06, 0.045, 0.94); // Ink over parchment
pub const PANEL_BORDER: Color = Color::new(0.62, 0.49, 0.29, 0.95); // Tarnished trim

pub const TEXT_PRIMARY: Color = Color::new(0.97, 0.94, 0.86, 1.0); // Parchment
pub const TEXT_SECONDARY: Color = Color::new(0.80, 0.76, 0.66, 1.0); // Muted ink
pub const TEXT_HIGHLIGHT: Color = Color::new(0.97, 0.80, 0.38, 1.0); // Lit gold

pub const SUCCESS: Color = Color::new(0.32, 0.78, 0.46, 1.0);
pub const FAILURE: Color = Color::new(0.78, 0.24, 0.22, 1.0);

pub const BUTTON_NORMAL: Color = Color::new(0.17, 0.13, 0.09, 1.0);

pub const FONT_TITLE_SIZE: f32 = 40.0;
pub const FONT_HEADER_SIZE: f32 = 28.0;
pub const FONT_BODY_SIZE: f32 = 20.0;
pub const FONT_SMALL_SIZE: f32 = 16.0;

// Feng Shui Feedback Colors
pub const FENG_SHUI_POSITIVE: Color = Color::new(0.3, 0.9, 0.4, 1.0); // Bright green
pub const FENG_SHUI_NEGATIVE: Color = Color::new(0.9, 0.2, 0.2, 1.0); // Bright red
pub const FENG_SHUI_NEUTRAL: Color = Color::new(0.5, 0.5, 0.55, 1.0); // Gray
pub const FENG_SHUI_EXCELLENT: Color = Color::new(1.0, 0.85, 0.2, 1.0); // Gold

// Panel colors (aliases and variants)
pub const PANEL: Color = PANEL_BG;
pub const PANEL_DARK: Color = Color::new(0.045, 0.038, 0.028, 0.96); // Darker ink wash
pub const BORDER: Color = PANEL_BORDER;

// Status colors
pub const WARNING: Color = Color::new(0.92, 0.62, 0.20, 1.0); // Amber
