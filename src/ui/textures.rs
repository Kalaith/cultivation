use macroquad::prelude::*;
use std::collections::HashMap;

/// Manages all game textures with lazy loading and caching
pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
    load_errors: Vec<String>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            load_errors: Vec::new(),
        }
    }

    /// Load all game textures asynchronously
    pub async fn load_all(&mut self) {
        // Screen backgrounds
        let backgrounds = [
            ("bg_main_menu", "assets/generated/backgrounds/bg_main_menu.png"),
            ("bg_sect_base", "assets/generated/backgrounds/bg_sect_base.png"),
            ("bg_roster", "assets/generated/backgrounds/bg_roster.png"),
            ("bg_world_map", "assets/generated/backgrounds/bg_world_map.png"),
            ("bg_mission_assign", "assets/generated/backgrounds/bg_mission_assign.png"),
            ("bg_mission_result", "assets/generated/backgrounds/bg_mission_result.png"),
            ("bg_library", "assets/generated/backgrounds/bg_library.png"),
            ("bg_factions", "assets/generated/backgrounds/bg_factions.png"),
            ("bg_trade", "assets/generated/backgrounds/bg_trade.png"),
            ("bg_tribulation", "assets/generated/backgrounds/bg_tribulation.png"),
            ("bg_sect_creation", "assets/generated/backgrounds/bg_sect_creation.png"),
        ];

        // Building illustrations
        let buildings = [
            ("bld_sect_hall", "assets/generated/buildings/bld_sect_hall.png"),
            ("bld_dormitory", "assets/generated/buildings/bld_dormitory.png"),
            ("bld_training_yard", "assets/generated/buildings/bld_training_yard.png"),
            ("bld_spirit_garden", "assets/generated/buildings/bld_spirit_garden.png"),
            ("bld_mission_board", "assets/generated/buildings/bld_mission_board.png"),
            ("bld_library", "assets/generated/buildings/bld_library.png"),
            ("bld_alchemy", "assets/generated/buildings/bld_alchemy.png"),
            ("bld_forge", "assets/generated/buildings/bld_forge.png"),
            ("bld_drying", "assets/generated/buildings/bld_drying.png"),
            ("bld_storage", "assets/generated/buildings/bld_storage.png"),
            ("bld_herb_garden", "assets/generated/buildings/bld_herb_garden.png"),
            ("bld_greenhouse", "assets/generated/buildings/bld_greenhouse.png"),
            ("bld_mine", "assets/generated/buildings/bld_mine.png"),
        ];

        // Element themed images
        let elements = [
            ("elem_metal", "assets/generated/elements/elem_metal.png"),
            ("elem_wood", "assets/generated/elements/elem_wood.png"),
            ("elem_water", "assets/generated/elements/elem_water.png"),
            ("elem_fire", "assets/generated/elements/elem_fire.png"),
            ("elem_earth", "assets/generated/elements/elem_earth.png"),
        ];

        // Seasonal images
        let seasons = [
            ("season_spring", "assets/generated/seasons/season_spring.png"),
            ("season_summer", "assets/generated/seasons/season_summer.png"),
            ("season_autumn", "assets/generated/seasons/season_autumn.png"),
            ("season_winter", "assets/generated/seasons/season_winter.png"),
        ];

        // Faction banners
        let factions = [
            ("faction_azure_cloud", "assets/generated/factions/faction_azure_cloud.png"),
            ("faction_golden_merchant", "assets/generated/factions/faction_golden_merchant.png"),
            ("faction_crimson_fang", "assets/generated/factions/faction_crimson_fang.png"),
            ("faction_shadow_moon", "assets/generated/factions/faction_shadow_moon.png"),
            ("faction_imperial", "assets/generated/factions/faction_imperial.png"),
            ("faction_player", "assets/generated/factions/faction_player.png"),
        ];

        // Realm progression images
        let realms = [
            ("realm_mortal", "assets/generated/realms/realm_mortal.png"),
            ("realm_body", "assets/generated/realms/realm_body.png"),
            ("realm_qi", "assets/generated/realms/realm_qi.png"),
            ("realm_foundation", "assets/generated/realms/realm_foundation.png"),
            ("realm_core", "assets/generated/realms/realm_core.png"),
            ("realm_nascent", "assets/generated/realms/realm_nascent.png"),
            ("realm_soul", "assets/generated/realms/realm_soul.png"),
            ("realm_ascension", "assets/generated/realms/realm_ascension.png"),
            ("realm_immortal", "assets/generated/realms/realm_immortal.png"),
        ];

        // Event illustrations
        let events = [
            ("event_breakthrough", "assets/generated/events/event_breakthrough.png"),
            ("event_tribulation", "assets/generated/events/event_tribulation.png"),
            ("event_death", "assets/generated/events/event_death.png"),
            ("event_discovery", "assets/generated/events/event_discovery.png"),
            ("event_war", "assets/generated/events/event_war.png"),
            ("event_peace", "assets/generated/events/event_peace.png"),
            ("event_disaster", "assets/generated/events/event_disaster.png"),
            ("event_blessing", "assets/generated/events/event_blessing.png"),
            ("event_visitor", "assets/generated/events/event_visitor.png"),
            ("event_ascension", "assets/generated/events/event_ascension.png"),
        ];

        // UI elements
        let ui_elements = [
            ("ui_panel_bg", "assets/generated/ui/ui_panel_bg.png"),
            ("ui_divider", "assets/generated/ui/ui_divider.png"),
        ];

        // Item vignettes
        let items = [
            ("item_spirit_stone", "assets/generated/items/item_spirit_stone.png"),
            ("item_spirit_grass", "assets/generated/items/item_spirit_grass.png"),
            ("item_ember_root", "assets/generated/items/item_ember_root.png"),
            ("item_moonwell_lotus", "assets/generated/items/item_moonwell_lotus.png"),
            ("item_iron_fungus", "assets/generated/items/item_iron_fungus.png"),
            ("item_ginseng", "assets/generated/items/item_ginseng.png"),
            ("item_pill", "assets/generated/items/item_pill.png"),
            ("item_sword", "assets/generated/items/item_sword.png"),
            ("item_scroll", "assets/generated/items/item_scroll.png"),
            ("item_relic", "assets/generated/items/item_relic.png"),
            ("item_ore", "assets/generated/items/item_ore.png"),
            ("item_dried_herb", "assets/generated/items/item_dried_herb.png"),
        ];

        // Region maps
        let regions = [
            ("region_central", "assets/generated/regions/region_central.png"),
            ("region_northern", "assets/generated/regions/region_northern.png"),
            ("region_southern", "assets/generated/regions/region_southern.png"),
            ("region_eastern", "assets/generated/regions/region_eastern.png"),
            ("region_western", "assets/generated/regions/region_western.png"),
            ("region_spirit_vein", "assets/generated/regions/region_spirit_vein.png"),
            ("region_demon_lands", "assets/generated/regions/region_demon_lands.png"),
            ("region_celestial", "assets/generated/regions/region_celestial.png"),
        ];

        // Mission scenes
        let missions = [
            ("mission_exploration", "assets/generated/missions/mission_exploration.png"),
            ("mission_gathering", "assets/generated/missions/mission_gathering.png"),
            ("mission_combat", "assets/generated/missions/mission_combat.png"),
            ("mission_diplomacy", "assets/generated/missions/mission_diplomacy.png"),
            ("mission_ruins", "assets/generated/missions/mission_ruins.png"),
        ];

        // Loading screens
        let loading = [
            ("loading_default", "assets/generated/loading/loading_default.png"),
            ("loading_combat", "assets/generated/loading/loading_combat.png"),
            ("loading_peaceful", "assets/generated/loading/loading_peaceful.png"),
            ("loading_ominous", "assets/generated/loading/loading_ominous.png"),
        ];

        // Load all texture groups
        for (name, path) in backgrounds.iter()
            .chain(buildings.iter())
            .chain(elements.iter())
            .chain(seasons.iter())
            .chain(factions.iter())
            .chain(realms.iter())
            .chain(events.iter())
            .chain(ui_elements.iter())
            .chain(items.iter())
            .chain(regions.iter())
            .chain(missions.iter())
            .chain(loading.iter())
        {
            self.load_texture(name, path).await;
        }

        println!("Texture loading complete: {} loaded, {} errors",
            self.textures.len(), self.load_errors.len());

        if !self.load_errors.is_empty() {
            eprintln!("Texture loading errors:");
            for err in &self.load_errors {
                eprintln!("  {}", err);
            }
        }
    }

    /// Load a single texture
    async fn load_texture(&mut self, name: &str, path: &str) {
        match load_texture(path).await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Linear);
                self.textures.insert(name.to_string(), texture);
            }
            Err(e) => {
                self.load_errors.push(format!("{}: {}", path, e));
            }
        }
    }

    /// Get a texture by name
    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
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

    /// Draw a background with a dark overlay for readability
    pub fn draw_background_with_overlay(&self, name: &str, overlay_alpha: f32) {
        // If texture not found, draw a RED fallback so it's obvious
        if self.get(name).is_none() {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.5, 0.0, 0.0, 1.0), // RED fallback - texture missing!
            );
        } else {
            self.draw_background(name);
        }
        // Draw semi-transparent overlay for better text readability
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, overlay_alpha),
        );
    }

    /// Draw a texture at a specific position and size
    pub fn draw_texture_sized(&self, name: &str, x: f32, y: f32, w: f32, h: f32) {
        if let Some(texture) = self.get(name) {
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    ..Default::default()
                },
            );
        }
    }

    /// Draw a texture maintaining aspect ratio within bounds
    pub fn draw_texture_fit(&self, name: &str, x: f32, y: f32, max_w: f32, max_h: f32) {
        if let Some(texture) = self.get(name) {
            let tex_aspect = texture.width() / texture.height();
            let box_aspect = max_w / max_h;

            let (draw_w, draw_h) = if tex_aspect > box_aspect {
                // Texture is wider - fit to width
                (max_w, max_w / tex_aspect)
            } else {
                // Texture is taller - fit to height
                (max_h * tex_aspect, max_h)
            };

            let offset_x = x + (max_w - draw_w) / 2.0;
            let offset_y = y + (max_h - draw_h) / 2.0;

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
        }
    }

    /// Check if textures are loaded
    pub fn is_loaded(&self) -> bool {
        !self.textures.is_empty()
    }

    /// Get the number of loaded textures
    pub fn texture_count(&self) -> usize {
        self.textures.len()
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
