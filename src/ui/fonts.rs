use macroquad::prelude::*;

/// Manages game fonts
pub struct FontManager {
    pub brush_font: Option<Font>,
    pub default_font: Option<Font>,
}

impl FontManager {
    pub fn new() -> Self {
        Self {
            brush_font: None,
            default_font: None,
        }
    }

    /// Load all game fonts asynchronously
    pub async fn load_all(&mut self) {
        // Try to load brush-style font for buttons
        match load_ttf_font("assets/fonts/brush.ttf").await {
            Ok(font) => {
                self.brush_font = Some(font);
                println!("Loaded brush font");
            }
            Err(e) => {
                eprintln!("Could not load brush font: {} - using default", e);
            }
        }
    }

    /// Get the brush font, falling back to default if not loaded
    pub fn get_brush_font(&self) -> Option<&Font> {
        self.brush_font.as_ref()
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}
