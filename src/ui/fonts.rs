use macroquad::prelude::*;

/// Manages game fonts
pub struct FontManager {
    pub brush_font: Option<Font>,
}

impl FontManager {
    pub fn new() -> Self {
        Self { brush_font: None }
    }

    /// Load all game fonts asynchronously
    pub async fn load_all(&mut self) {
        match macroquad_toolkit::ui::load_builtin_rajdhani_semibold_font() {
            Ok(font) => {
                self.brush_font = Some(font);
                let _ = macroquad_toolkit::ui::ensure_default_ui_font();
            }
            Err(e) => {
                eprintln!("Could not load toolkit UI font: {} - using default", e);
            }
        }
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}
