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
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}
