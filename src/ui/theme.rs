use macroquad::prelude::*;

pub const PRIMARY: Color = Color::new(0.9, 0.8, 0.2, 1.0); // Gold
pub const SECONDARY: Color = Color::new(0.2, 0.8, 0.8, 1.0); // Cyan/Jade
pub const ACCENT: Color = Color::new(0.8, 0.3, 0.3, 1.0); // Crimson

pub const PANEL_BG: Color = Color::new(0.1, 0.12, 0.15, 0.75); // Semi-transparent panel
pub const PANEL_BORDER: Color = Color::new(0.3, 0.3, 0.35, 1.0); // Border color

pub const TEXT_PRIMARY: Color = Color::new(0.95, 0.95, 0.95, 1.0); // White-ish
pub const TEXT_SECONDARY: Color = Color::new(0.7, 0.7, 0.75, 1.0); // Gray-ish
pub const TEXT_HIGHLIGHT: Color = Color::new(1.0, 0.9, 0.4, 1.0); // Text Gold

pub const SUCCESS: Color = Color::new(0.3, 0.8, 0.3, 1.0);
pub const FAILURE: Color = Color::new(0.8, 0.3, 0.3, 1.0);

pub const BUTTON_NORMAL: Color = Color::new(0.2, 0.25, 0.3, 1.0);

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
pub const PANEL_DARK: Color = Color::new(0.08, 0.09, 0.12, 0.8); // Darker panel, semi-transparent
pub const BORDER: Color = PANEL_BORDER;

// Status colors
pub const WARNING: Color = Color::new(0.9, 0.7, 0.2, 1.0); // Amber/Yellow
