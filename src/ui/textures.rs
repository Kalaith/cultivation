use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use std::collections::HashMap;

const GENERATED_ASSET_PACK: &str = "assets/generated.zip";

/// Manages all game textures with lazy loading and caching
pub struct TextureManager {
    assets: AssetManager,
    load_errors: Vec<String>,
}

impl TextureManager {
    pub fn new() -> Self {
        let mut assets = AssetManager::new();
        assets.set_default_filter(FilterMode::Linear);

        Self {
            assets,
            load_errors: Vec::new(),
        }
    }

    /// Load all game textures asynchronously
    pub async fn load_all(&mut self) {
        use macroquad_toolkit::data_loader::load_data;

        self.assets.load_asset_pack(GENERATED_ASSET_PACK).await.ok();

        // Load texture configuration from JSON using toolkit
        match load_data::<HashMap<String, HashMap<String, String>>>("textures").await {
            Ok(texture_config) => {
                for (_category, textures) in texture_config {
                    for (name, path) in textures {
                        self.load_texture(&name, &path).await;
                    }
                }
            }
            Err(e) => {
                self.load_errors
                    .push(format!("Failed to load textures: {}", e));
                eprintln!("Texture Data Load Error: {}", e);
            }
        }

        println!(
            "Texture loading complete: {} loaded, {} errors",
            self.assets.len(),
            self.load_errors.len()
        );

        if !self.load_errors.is_empty() {
            eprintln!("Texture loading errors:");
            for err in &self.load_errors {
                eprintln!("  {}", err);
            }
        }
    }

    /// Load a single texture
    async fn load_texture(&mut self, name: &str, path: &str) {
        if let Err(e) = self
            .assets
            .load_texture_with_filter(name, path, FilterMode::Linear)
            .await
        {
            self.load_errors.push(format!("{}: {}", path, e));
        }
    }

    /// Get a texture by name
    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.assets.get_texture(name)
    }

    /// Draw a background texture scaled to fill the screen
    pub fn draw_background(&self, name: &str) {
        let sw = screen_width();
        let sh = screen_height();

        if let Some(texture) = self.get(name) {
            // Calculate scale to cover entire screen while maintaining aspect ratio
            let tex_aspect = texture.width() / texture.height();
            let screen_aspect = sw / sh;

            let (draw_w, draw_h, offset_x, offset_y) = if screen_aspect > tex_aspect {
                // Screen is wider - fit to width
                let scale = sw / texture.width();
                let h = texture.height() * scale;
                (sw, h, 0.0, (sh - h) / 2.0)
            } else {
                // Screen is taller - fit to height
                let scale = sh / texture.height();
                let w = texture.width() * scale;
                (w, sh, (sw - w) / 2.0, 0.0)
            };

            draw_texture_ex(
                texture,
                offset_x,
                offset_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(draw_w, draw_h)),
                    ..Default::default()
                },
            );
        } else {
            // Texture not found - draw a gradient fallback background
            // Dark blue-gray gradient to indicate missing texture while still being usable
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.08, 0.10, 0.14, 1.0));
        }
    }

    /// Get the number of loaded textures
    pub fn texture_count(&self) -> usize {
        self.assets.len()
    }

    /// Get loading error count
    pub fn error_count(&self) -> usize {
        self.load_errors.len()
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}
