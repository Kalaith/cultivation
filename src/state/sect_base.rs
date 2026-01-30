use crate::data::buildings::{BuildingStatus, BuildingType};
use crate::data::disciples::Disciple;
use crate::data::elements::Element;
use crate::data::herbs::Season;
use crate::data::loader::GameData;
use crate::data::grid::Grid;
use crate::data::missions::{MissionOutcome, OngoingMission};
use crate::data::spirit_beasts::SpiritBeast;
use crate::engine::actions::Action;
use crate::state::{StateTransition, TutorialState, UpdateResult};
use crate::ui::components::*;
use crate::ui::herbs;
use crate::ui::TextureManager;
use crate::ui::tech::{self, TechTreeState};
use crate::ui::theme::*;
use macroquad::prelude::*;
use std::collections::HashMap;

const SECT_MAP_WIDTH: f32 = 4096.0;
const SECT_MAP_HEIGHT: f32 = 4096.0;
const BUILDING_SCALE: f32 = 0.25;
const ALPHA_THRESHOLD: u8 = 10;

#[derive(Clone, PartialEq)]
enum SectView {
    Map,
    BuildingDetails(u64), // Using ID
    SpiritBeasts,
}

pub struct SectBaseState {
    view: SectView,
    settings_open: bool,
    feng_shui_overlay_active: bool,
    crafting_modal_open: bool,
    tech_tree_open: bool,
    tech_tree_state: TechTreeState,
    construction_scroll: f32,
    placement_mode: Option<crate::data::buildings::BuildingType>,
    map_zoom: f32,
    map_center: Vec2,
    map_dragging: bool,
    map_last_mouse: Vec2,
    building_mask_cache: HashMap<String, Image>,
    selected_beast_index: Option<usize>,
    beast_equip_modal_open: bool,
    // Herb system modals
    herb_planting_modal: Option<usize>, // Some(plot_index) when open
    disciple_assignment_modal: bool,
    infusion_modal_open: bool,
}

impl SectBaseState {
    pub fn new() -> Self {
        Self {
            view: SectView::Map,
            settings_open: false,
            feng_shui_overlay_active: false,
            crafting_modal_open: false,
            tech_tree_open: false,
            tech_tree_state: TechTreeState::new(),
            construction_scroll: 0.0,
            placement_mode: None,
            map_zoom: 0.35,
            map_center: vec2(SECT_MAP_WIDTH / 2.0, SECT_MAP_HEIGHT / 2.0),
            map_dragging: false,
            map_last_mouse: vec2(0.0, 0.0),
            building_mask_cache: HashMap::new(),
            selected_beast_index: None,
            beast_equip_modal_open: false,
            herb_planting_modal: None,
            disciple_assignment_modal: false,
            infusion_modal_open: false,
        }
    }

    /// Update handling immediate mode UI
    pub fn update(
        &mut self,
        data: &mut GameData,
        grid: &mut Grid,
        textures: &TextureManager,
        spirit_stones: u32,
        herbs: u32,
        influence: u32,
        relics: u32,
        inventory: &std::collections::HashMap<String, u32>,
        unlocked_techs: &[String],
        event_log: &[String],
        ongoing_missions: &[OngoingMission],
        completed_missions: &[MissionOutcome],
        completed_history: &[String],
        disciples: &[Disciple],
        spirit_beasts: &[SpiritBeast],
        current_season: &Season,
        season_ticks: u32,
        tutorial: &mut TutorialState,
        discovered_recipes: &[String],
    ) -> UpdateResult {
        
        // --- 1. Global Input & Navigation ---
        if let Some(res) = self.handle_global_input() {
            return res;
        }

        // --- 2. Layout & Setup ---
        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;
        let left_panel_w = 250.0;
        let right_panel_w = 250.0;
        let center_w = screen_w - left_panel_w - right_panel_w - 20.0;
        
        // --- 3. Draw Center Panel (Main Content) ---
        let center_rect = Rect::new(left_panel_w, header_h, center_w, screen_h - header_h);
        if let Some(res) = self.draw_center_panel(
            center_rect,
            data,
            grid,
            textures,
            spirit_stones,
            herbs,
            inventory,
            unlocked_techs,
            ongoing_missions,
            completed_missions,
            completed_history,
            disciples,
            spirit_beasts,
            current_season,
            discovered_recipes,
        ) {
            return res;
        }

        // --- 4. Draw Header ---
        self.draw_header(screen_w, header_h, spirit_stones, herbs, influence, relics, current_season, season_ticks, tutorial);

        // --- 5. Draw Left Panel (Buildings & Navigation) ---
        if let Some(res) = self.draw_left_panel(header_h, screen_h, left_panel_w, data) {
            return res;
        }

        // --- 6. Draw Right Panel (Event Log) ---
        self.update_tutorial_progress(tutorial, data, unlocked_techs, disciples, ongoing_missions, completed_history);
        self.draw_right_panel(header_h, screen_h, left_panel_w, center_w, right_panel_w, event_log);

        // --- 6.5 Tutorial Overlay (Centered) ---
        if tutorial.active && !tutorial.hidden {
            self.draw_tutorial_overlay(screen_w, screen_h, tutorial);
        }

        // --- 7. Modals (Settings, etc) ---
        if self.settings_open {
            if let Some(res) = self.draw_settings_modal(screen_w, screen_h) {
                return res;
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _grid: &Grid, _spirit_stones: u32) {
        // Drawing handled in update for immediate mode
    }

    // --- Helper Functions ---

    fn handle_global_input(&mut self) -> Option<UpdateResult> {
        if is_key_pressed(KeyCode::Escape) {
            if self.settings_open {
                self.settings_open = false;
            } else if self.tech_tree_open {
                self.tech_tree_open = false;
            } else if self.crafting_modal_open {
                // If in details view, might close modal, but if in map view (construction), also close
                self.crafting_modal_open = false;
            } else if self.placement_mode.is_some() {
                self.placement_mode = None;
            } else if let SectView::BuildingDetails(_) = self.view {
                self.view = SectView::Map; // Go back to map
            } else if let SectView::SpiritBeasts = self.view {
                self.view = SectView::Map;
            } else {
                return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
            }
        }
        None
    }

    fn draw_header(&mut self, screen_w: f32, header_h: f32, spirit_stones: u32, herbs: u32, influence: u32, relics: u32, current_season: &Season, season_ticks: u32, tutorial: &mut TutorialState) {
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("SECT MANAGEMENT", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);

        // Season indicator
        herbs::draw_season_indicator(250.0, 40.0, current_season, season_ticks);

        let res_text = format!("SS: {}  Herbs: {}  Infl: {}  Relics: {}", spirit_stones, herbs, influence, relics);
        let res_dims = measure_text(&res_text, None, FONT_HEADER_SIZE as u16, 1.0);

        draw_text(&res_text, screen_w - res_dims.width - 60.0, 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        // Tutorial toggle (if hidden)
        if tutorial.active && tutorial.hidden {
            if draw_button(Rect::new(screen_w - 100.0, 10.0, 40.0, 40.0), "?", false) {
                tutorial.hidden = false;
            }
        }

        // Cog Button (Settings)
        if draw_button(Rect::new(screen_w - 50.0, 10.0, 40.0, 40.0), "O", false) {
            self.settings_open = !self.settings_open;
        }
    }

    fn draw_left_panel(&mut self, header_h: f32, screen_h: f32, width: f32, data: &GameData) -> Option<UpdateResult> {
        let rect = Rect::new(0.0, header_h, width, screen_h - header_h);
        draw_panel(rect, Some("Buildings"));

        // List constructed buildings
        let mut btn_y = rect.y + 40.0;
        for building in &data.buildings {
            let status_str = match building.status {
                BuildingStatus::Active => "",
                BuildingStatus::Ruined => " (Ruined)",
                BuildingStatus::Constructing => " (Building...)",
            };
            let label = format!("{}{}", building.building_type, status_str);

            if draw_button(Rect::new(rect.x + 10.0, btn_y, width - 20.0, 35.0), &label, false) {
                self.view = SectView::BuildingDetails(building.id);
            }
            btn_y += 40.0;

            // Prevent overflow
            if btn_y > rect.y + rect.h - 200.0 {
                draw_text("...", rect.x + 10.0, btn_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                break;
            }
        }

        // Navigation buttons at bottom
        let nav_y_start = rect.y + rect.h - 270.0;
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start, width - 20.0, 40.0), "Disciples", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster));
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 50.0, width - 20.0, 40.0), "World Map", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToWorldMap));
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 100.0, width - 20.0, 40.0), "Factions", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToFactionScreen));
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 150.0, width - 20.0, 40.0), "Trade", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToTradeScreen));
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 200.0, width - 20.0, 40.0), "Construction", false) {
             self.view = SectView::Map;
             self.crafting_modal_open = true;
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 250.0, width - 20.0, 40.0), "Spirit Beasts", false) {
            self.view = SectView::SpiritBeasts;
        }
        None
    }

    fn draw_center_panel(
        &mut self,
        rect: Rect,
        data: &mut GameData,
        grid: &mut Grid,
        textures: &TextureManager,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        unlocked_techs: &[String],
        ongoing_missions: &[OngoingMission],
        completed_missions: &[MissionOutcome],
        completed_history: &[String],
        disciples: &[Disciple],
        spirit_beasts: &[SpiritBeast],
        current_season: &Season,
        discovered_recipes: &[String],
    ) -> Option<UpdateResult> {
        match self.view {
            SectView::Map => self.draw_map_view(rect, data, grid, textures, spirit_stones, unlocked_techs),
            SectView::BuildingDetails(id) => self.draw_building_details(
                rect,
                id,
                data,
                spirit_stones,
                herbs,
                inventory,
                unlocked_techs,
                ongoing_missions,
                completed_missions,
                completed_history,
                disciples,
                current_season,
                discovered_recipes,
            ),
            SectView::SpiritBeasts => self.draw_spirit_beasts_view(rect, data, spirit_beasts, inventory),
        }
    }

    fn draw_map_view(
        &mut self,
        rect: Rect,
        data: &mut GameData,
        _grid: &mut Grid,
        textures: &TextureManager,
        _spirit_stones: u32,
        _unlocked_techs: &[String],
    ) -> Option<UpdateResult> {
        draw_panel(rect, Some("Sect Map"));

        let mouse = vec2(mouse_position().0, mouse_position().1);

        // Pan/Zoom controls within map view
        if rect.contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if wheel.abs() > 0.0 {
                let prev_zoom = self.map_zoom;
                self.map_zoom = (self.map_zoom * (1.0 + wheel * 0.1)).clamp(0.2, 2.0);
                let world_before = self.screen_to_world_with_zoom(rect, mouse, prev_zoom);
                let world_after = self.screen_to_world(rect, mouse);
                self.map_center += world_before - world_after;
            }
        }

        let drag_active = is_mouse_button_down(MouseButton::Middle)
            || (is_key_down(KeyCode::Space) && is_mouse_button_down(MouseButton::Left));
        if drag_active && rect.contains(mouse.into()) {
            if !self.map_dragging {
                self.map_dragging = true;
                self.map_last_mouse = mouse;
            } else {
                let delta = mouse - self.map_last_mouse;
                self.map_center -= delta / self.map_zoom;
                self.map_last_mouse = mouse;
            }
        } else {
            self.map_dragging = false;
        }

        self.clamp_map_center(rect);

        // Draw base map
        if let Some(tex) = textures.get("sect_map_base") {
            let top_left = self.world_to_screen(rect, vec2(0.0, 0.0));
            draw_texture_ex(
                tex,
                top_left.x,
                top_left.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(SECT_MAP_WIDTH * self.map_zoom, SECT_MAP_HEIGHT * self.map_zoom)),
                    ..Default::default()
                },
            );
        } else {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.12, 0.11, 0.10, 1.0));
        }

        self.draw_placement_preview(rect, 0.0, 0.0);

        // Placement preview + placement action
        if let Some(place_type) = self.placement_mode.clone() {
            if rect.contains(mouse.into()) {
                let world_pos = self.screen_to_world(rect, mouse);
                let can_place = self.can_place_building(textures, data, &place_type, world_pos);
                let tint = if can_place {
                    Color::new(0.2, 1.0, 0.2, 0.65)
                } else {
                    Color::new(1.0, 0.2, 0.2, 0.65)
                };
                self.draw_building_sprite(rect, textures, &place_type, world_pos, tint);

                if is_mouse_button_pressed(MouseButton::Left) && can_place {
                    self.placement_mode = None;
                    return Some(UpdateResult::new().with_action(Action::ConstructBuilding(
                        place_type,
                        world_pos.x.round() as i32,
                        world_pos.y.round() as i32,
                    )));
                }
            }

            if is_mouse_button_pressed(MouseButton::Right) {
                self.placement_mode = None;
            }
        }

        // Draw buildings
        let mut hovered_building_id: Option<u64> = None;
        let mouse_world = self.screen_to_world(rect, mouse);

        for building in data.buildings.iter() {
            let world_pos = vec2(building.x as f32, building.y as f32);
            let tint = if building.status == BuildingStatus::Ruined {
                Color::new(0.6, 0.6, 0.6, 1.0)
            } else {
                WHITE
            };
            self.draw_building_sprite(rect, textures, &building.building_type, world_pos, tint);

            if self.placement_mode.is_none() && rect.contains(mouse.into()) {
                if self.point_hits_building(textures, building, mouse_world) {
                    hovered_building_id = Some(building.id);
                }
            }
        }

        if let Some(id) = hovered_building_id {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.view = SectView::BuildingDetails(id);
            }
        }

        // Construction Modal (reusing crafting_modal_open)
        if self.crafting_modal_open {
            return self.draw_construction_modal(data, _unlocked_techs);
        }

        None
    }

    fn draw_placement_preview(&mut self, rect: Rect, _map_x: f32, _map_y: f32) {
        if let Some(place_type) = &self.placement_mode {
             draw_text(&format!("Placing: {:?} (Click to Build, RMB/Esc to Cancel)", place_type), rect.x + 20.0, rect.y + rect.h - 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
        }
    }

    fn world_to_screen(&self, rect: Rect, world: Vec2) -> Vec2 {
        let view_center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        view_center + (world - self.map_center) * self.map_zoom
    }

    fn screen_to_world(&self, rect: Rect, screen: Vec2) -> Vec2 {
        self.screen_to_world_with_zoom(rect, screen, self.map_zoom)
    }

    fn screen_to_world_with_zoom(&self, rect: Rect, screen: Vec2, zoom: f32) -> Vec2 {
        let view_center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        self.map_center + (screen - view_center) / zoom
    }

    fn clamp_map_center(&mut self, rect: Rect) {
        let half_view = vec2(rect.w / (2.0 * self.map_zoom), rect.h / (2.0 * self.map_zoom));
        let min_x = half_view.x;
        let min_y = half_view.y;
        let max_x = SECT_MAP_WIDTH - half_view.x;
        let max_y = SECT_MAP_HEIGHT - half_view.y;

        if max_x < min_x {
            self.map_center.x = SECT_MAP_WIDTH * 0.5;
        } else {
            self.map_center.x = self.map_center.x.clamp(min_x, max_x);
        }

        if max_y < min_y {
            self.map_center.y = SECT_MAP_HEIGHT * 0.5;
        } else {
            self.map_center.y = self.map_center.y.clamp(min_y, max_y);
        }
    }

    fn building_texture_key(&self, b_type: &BuildingType) -> Option<&'static str> {
        match b_type {
            BuildingType::SectHall => Some("bld_sect_hall"),
            BuildingType::Dormitory => Some("bld_dormitory"),
            BuildingType::TrainingYard => Some("bld_training_yard"),
            BuildingType::LibraryPavilion => Some("bld_library"),
            BuildingType::MissionBoard => Some("bld_mission_board"),
            BuildingType::SpiritGarden => Some("bld_spirit_garden"),
            BuildingType::AlchemyFurnace => Some("bld_alchemy"),
            BuildingType::ArtifactForge => Some("bld_forge"),
            BuildingType::Blacksmith => Some("bld_forge"),
            BuildingType::HerbGarden => Some("bld_herb_garden"),
            BuildingType::Greenhouse => Some("bld_greenhouse"),
            BuildingType::DryingPavilion => Some("bld_drying"),
            BuildingType::HerbStorage => Some("bld_storage"),
            BuildingType::TalismanScriptorium => Some("bld_scriptorium"),
            BuildingType::Decoration => None,
        }
    }

    fn building_sprite_metrics(
        &mut self,
        textures: &TextureManager,
        b_type: &BuildingType,
        world_pos: Vec2,
    ) -> Option<(String, Vec2, Vec2, Vec2)> {
        let key = self.building_texture_key(b_type)?.to_string();
        let tex = textures.get(&key)?;
        let tex_size = vec2(tex.width(), tex.height());
        let size = tex_size * BUILDING_SCALE;
        let top_left = world_pos - vec2(size.x * 0.5, size.y);
        Some((key, top_left, size, tex_size))
    }

    fn draw_building_sprite(
        &mut self,
        rect: Rect,
        textures: &TextureManager,
        b_type: &BuildingType,
        world_pos: Vec2,
        tint: Color,
    ) {
        if let Some((key, top_left, size, _tex_size)) = self.building_sprite_metrics(textures, b_type, world_pos) {
            if let Some(tex) = textures.get(&key) {
                let screen_pos = self.world_to_screen(rect, top_left);
                let draw_size = size * self.map_zoom;
                draw_texture_ex(
                    tex,
                    screen_pos.x,
                    screen_pos.y,
                    tint,
                    DrawTextureParams {
                        dest_size: Some(draw_size),
                        ..Default::default()
                    },
                );
                return;
            }
        }

        // Fallback brush stroke
        let screen_pos = self.world_to_screen(rect, world_pos);
        draw_brush_stroke(screen_pos.x - 12.0, screen_pos.y - 12.0, 24.0, 24.0, tint, 1.0);
    }

    fn get_texture_mask<'a>(
        &'a mut self,
        textures: &'a TextureManager,
        key: &str,
    ) -> Option<&'a Image> {
        if !self.building_mask_cache.contains_key(key) {
            if let Some(tex) = textures.get(key) {
                let image = tex.get_texture_data();
                self.building_mask_cache.insert(key.to_string(), image);
            }
        }
        self.building_mask_cache.get(key)
    }

    fn alpha_at(image: &Image, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= image.width as i32 || y >= image.height as i32 {
            return 0;
        }
        let idx = ((y * image.width as i32 + x) * 4) as usize;
        image.bytes.get(idx + 3).copied().unwrap_or(0)
    }

    fn point_hits_building(
        &mut self,
        textures: &TextureManager,
        building: &crate::data::buildings::Building,
        world_point: Vec2,
    ) -> bool {
        let world_pos = vec2(building.x as f32, building.y as f32);
        let Some((key, top_left, size, _tex_size)) =
            self.building_sprite_metrics(textures, &building.building_type, world_pos)
        else {
            return false;
        };

        let bottom_right = top_left + size;
        if world_point.x < top_left.x
            || world_point.y < top_left.y
            || world_point.x > bottom_right.x
            || world_point.y > bottom_right.y
        {
            return false;
        }

        let Some(mask) = self.get_texture_mask(textures, &key) else {
            return true;
        };

        let local = (world_point - top_left) / BUILDING_SCALE;
        let alpha = Self::alpha_at(mask, local.x.floor() as i32, local.y.floor() as i32);
        alpha > ALPHA_THRESHOLD
    }

    fn can_place_building(
        &mut self,
        textures: &TextureManager,
        data: &GameData,
        b_type: &BuildingType,
        world_pos: Vec2,
    ) -> bool {
        let Some((key, top_left, size, _tex_size)) = self.building_sprite_metrics(textures, b_type, world_pos) else {
            return false;
        };

        let bottom_right = top_left + size;
        if top_left.x < 0.0
            || top_left.y < 0.0
            || bottom_right.x > SECT_MAP_WIDTH
            || bottom_right.y > SECT_MAP_HEIGHT
        {
            return false;
        }

        for existing in &data.buildings {
            let existing_pos = vec2(existing.x as f32, existing.y as f32);
            if self.sprites_overlap(textures, &key, top_left, size, existing, existing_pos) {
                return false;
            }
        }

        true
    }

    fn sprites_overlap(
        &mut self,
        textures: &TextureManager,
        candidate_key: &str,
        candidate_top_left: Vec2,
        candidate_size: Vec2,
        existing: &crate::data::buildings::Building,
        existing_pos: Vec2,
    ) -> bool {
        let Some((existing_key, existing_top_left, existing_size, _existing_tex_size)) =
            self.building_sprite_metrics(textures, &existing.building_type, existing_pos)
        else {
            // Fallback to bounding boxes if no texture
            let a_max = candidate_top_left + candidate_size;
            let fallback_size = vec2(64.0, 64.0);
            let fallback_top_left = existing_pos - vec2(fallback_size.x * 0.5, fallback_size.y);
            let b_max = fallback_top_left + fallback_size;
            return candidate_top_left.x < b_max.x
                && a_max.x > fallback_top_left.x
                && candidate_top_left.y < b_max.y
                && a_max.y > fallback_top_left.y;
        };

        let a_max = candidate_top_left + candidate_size;
        let b_max = existing_top_left + existing_size;

        let inter_left = candidate_top_left.x.max(existing_top_left.x).floor() as i32;
        let inter_top = candidate_top_left.y.max(existing_top_left.y).floor() as i32;
        let inter_right = a_max.x.min(b_max.x).ceil() as i32;
        let inter_bottom = a_max.y.min(b_max.y).ceil() as i32;

        if inter_right <= inter_left || inter_bottom <= inter_top {
            return false;
        }

        let candidate_mask = self.get_texture_mask(textures, candidate_key).cloned();
        let existing_mask = self.get_texture_mask(textures, &existing_key).cloned();
        let Some(candidate_mask) = candidate_mask else {
            return true;
        };
        let Some(existing_mask) = existing_mask else {
            return true;
        };

        for y in inter_top..inter_bottom {
            for x in inter_left..inter_right {
                let world = vec2(x as f32 + 0.5, y as f32 + 0.5);
                let cand_local = (world - candidate_top_left) / BUILDING_SCALE;
                let ex_local = (world - existing_top_left) / BUILDING_SCALE;

                let cand_alpha = Self::alpha_at(&candidate_mask, cand_local.x.floor() as i32, cand_local.y.floor() as i32);
                if cand_alpha <= ALPHA_THRESHOLD {
                    continue;
                }
                let ex_alpha = Self::alpha_at(&existing_mask, ex_local.x.floor() as i32, ex_local.y.floor() as i32);
                if ex_alpha > ALPHA_THRESHOLD {
                    return true;
                }
            }
        }

        false
    }

    fn draw_building_details(
        &mut self,
        rect: Rect,
        id: u64,
        data: &mut GameData,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        unlocked_techs: &[String],
        ongoing_missions: &[OngoingMission],
        completed_missions: &[MissionOutcome],
        completed_history: &[String],
        disciples: &[Disciple],
        current_season: &Season,
        discovered_recipes: &[String],
    ) -> Option<UpdateResult> {

        let building = data.buildings.iter().find(|b| b.id == id)?.clone();
        let b_type = building.building_type.clone();

        draw_text(&format!("{}", b_type), rect.x + 20.0, rect.y + 60.0, FONT_HEADER_SIZE, PRIMARY);
        let d_y = rect.y + 100.0;

        draw_text(&format!("Level: {}", building.level), rect.x + 20.0, d_y, FONT_BODY_SIZE, TEXT_PRIMARY);
        draw_text(&format!("Element: {:?}", building.element), rect.x + 20.0, d_y + 30.0, FONT_BODY_SIZE, TEXT_SECONDARY);
        draw_text(&format!("Feng Shui: {:.1}", building.feng_shui_score), rect.x + 20.0, d_y + 60.0, FONT_BODY_SIZE,
            if building.feng_shui_score > 0.0 { Color::new(0.2, 0.8, 0.2, 1.0) }
            else if building.feng_shui_score < 0.0 { Color::new(0.8, 0.2, 0.2, 1.0) }
            else { TEXT_SECONDARY }
        );

        // Close Button for Details
        if draw_button(Rect::new(rect.x + rect.w - 80.0, rect.y + 10.0, 60.0, 30.0), "Back", false) {
            self.view = SectView::Map;
            self.herb_planting_modal = None;
            self.disciple_assignment_modal = false;
            self.infusion_modal_open = false;
            return None;
        }

        // Specialized Building Actions
        let mut action_y = d_y + 100.0;
        if b_type == BuildingType::SectHall {
            let capacity: u32 = data
                .buildings
                .iter()
                .filter(|b| b.status == BuildingStatus::Active)
                .map(|b| match b.building_type {
                    BuildingType::SectHall => b.get_max_disciples(),
                    BuildingType::Dormitory => b.get_dorm_capacity(),
                    _ => 0,
                })
                .sum();
            let current = disciples.len() as u32;
            draw_text(
                &format!("Population: {}/{}", current, capacity),
                rect.x + 20.0,
                d_y + 90.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            action_y = d_y + 130.0;
        }

        if building.status == BuildingStatus::Ruined {
            draw_text("(Ruined)", rect.x + 200.0, rect.y + 60.0, FONT_HEADER_SIZE, Color::new(0.8, 0.2, 0.2, 1.0));
            let repair_cost = building.repair_cost;
            let repair_label = if b_type == BuildingType::SectHall {
                format!("Restore ({} SS)", repair_cost)
            } else {
                format!("Repair ({} SS)", repair_cost)
            };
            if draw_button(Rect::new(rect.x + 20.0, action_y, 200.0, 40.0), &repair_label, false) {
                 return Some(UpdateResult::new().with_action(Action::RepairBuilding(building.id)));
            }
        } else if b_type == BuildingType::MissionBoard {
            if let Some(res) = self.draw_mission_list(rect, data, ongoing_missions, completed_missions, completed_history, action_y) {
                return Some(res);
            }
        } else if matches!(b_type, BuildingType::HerbGarden | BuildingType::Greenhouse) {
            // Herb garden/greenhouse specific UI
            return self.draw_herb_building_details(rect, &building, data, disciples, current_season, action_y);
        } else if b_type == BuildingType::DryingPavilion {
            // Drying Pavilion UI
            return self.draw_drying_pavilion_details(rect, &building, data, inventory, action_y);
        } else if b_type == BuildingType::HerbStorage {
            // Herb Storage UI
            return self.draw_herb_storage_details(rect, &building, data, inventory, action_y);
        } else {
             // Upgrade Button
             if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
                  return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
             }

             // Specific Buttons
             if b_type == BuildingType::SectHall {
                  if draw_button(Rect::new(rect.x + 180.0, action_y, 150.0, 40.0), "Recruit", false) {
                      return Some(UpdateResult::new().with_action(Action::RecruitDisciple));
                  }
                  if draw_button(Rect::new(rect.x + 340.0, action_y, 150.0, 40.0), "Research / Tech", false) {
                      self.tech_tree_open = true;
                  }
               } else if matches!(b_type, BuildingType::AlchemyFurnace | BuildingType::ArtifactForge | BuildingType::Blacksmith | BuildingType::TalismanScriptorium) {
                  if draw_button(Rect::new(rect.x + 180.0, action_y, 150.0, 40.0), "Crafting", false) {
                       self.crafting_modal_open = true;
                  }
             }
        }

        // Modals overlaying details
        if self.tech_tree_open {
            let (screen_w, screen_h) = (screen_width(), screen_height());
            let modal_w = 650.0;
            let modal_x = (screen_w - modal_w) / 2.0;
            let modal_y = (screen_h - 550.0) / 2.0;

            let action = tech::draw_tech_tree_modal(&mut self.tech_tree_state, data, unlocked_techs, spirit_stones);

            if tech::check_tech_tree_close(modal_x, modal_y, modal_w) {
                self.tech_tree_open = false;
            } else if let Some(action) = action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.crafting_modal_open {
             if let Some(res) = self.draw_crafting_modal(data, &b_type, spirit_stones, herbs, inventory, disciples, discovered_recipes) {
                 return Some(res);
             }
        }

        None
    }

    /// Draw herb garden or greenhouse building details
    fn draw_herb_building_details(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        data: &GameData,
        disciples: &[Disciple],
        current_season: &Season,
        action_y: f32,
    ) -> Option<UpdateResult> {
        let b_type = &building.building_type;

        // Action buttons row
        if draw_button(Rect::new(rect.x + 20.0, action_y, 140.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
        }

        if draw_button(Rect::new(rect.x + 170.0, action_y, 140.0, 40.0), "Assign Worker", false) {
            self.disciple_assignment_modal = true;
        }

        // Greenhouse-specific: infusion button
        if *b_type == BuildingType::Greenhouse {
            if draw_button(Rect::new(rect.x + 320.0, action_y, 140.0, 40.0), "Set Infusion", false) {
                self.infusion_modal_open = true;
            }
        }

        // Draw herb garden panel
        let panel_y = action_y + 60.0;
        if *b_type == BuildingType::Greenhouse {
            herbs::draw_greenhouse_panel(building, data, disciples, current_season, rect.x + 20.0, panel_y, rect.w - 40.0);
        } else {
            herbs::draw_herb_garden_panel(building, data, disciples, current_season, rect.x + 20.0, panel_y, rect.w - 40.0);
        }

        // Plant buttons for each empty plot
        let mut plot_btn_y = panel_y + 30.0;
        for (i, plot) in building.herb_plots.iter().enumerate() {
            if i >= building.get_max_herb_plots() {
                break;
            }
            if plot.growing.is_none() {
                // Show plant button next to the plot display
                if draw_button(Rect::new(rect.x + rect.w - 120.0, plot_btn_y + 10.0, 80.0, 30.0), "Plant", false) {
                    self.herb_planting_modal = Some(i);
                }
            }
            plot_btn_y += 65.0;
        }

        // Handle modals
        if let Some(plot_idx) = self.herb_planting_modal {
            let result = herbs::draw_herb_planting_modal(building, data, current_season, plot_idx);
            if result.close_modal {
                self.herb_planting_modal = None;
            }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.disciple_assignment_modal {
            let result = herbs::draw_disciple_assignment_modal(building, disciples, &data.buildings);
            if result.close_modal {
                self.disciple_assignment_modal = false;
            }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.infusion_modal_open {
            let result = herbs::draw_infusion_modal(building);
            if result.close_modal {
                self.infusion_modal_open = false;
            }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        None
    }

    /// Draw drying pavilion building details
    fn draw_drying_pavilion_details(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        data: &GameData,
        inventory: &std::collections::HashMap<String, u32>,
        action_y: f32,
    ) -> Option<UpdateResult> {
        // Upgrade button
        if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(building.building_type.clone())));
        }

        // Draw drying panel
        let panel_y = action_y + 60.0;
        if let Some(action) = herbs::draw_drying_pavilion_panel(building, data, inventory, rect.x + 20.0, panel_y, rect.w - 40.0) {
            return Some(UpdateResult::new().with_action(action));
        }

        None
    }

    /// Draw herb storage building details
    fn draw_herb_storage_details(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        data: &GameData,
        inventory: &std::collections::HashMap<String, u32>,
        action_y: f32,
    ) -> Option<UpdateResult> {
        // Upgrade button
        if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(building.building_type.clone())));
        }

        // Draw storage panel
        let panel_y = action_y + 60.0;
        herbs::draw_herb_storage_panel(building, data, inventory, rect.x + 20.0, panel_y, rect.w - 40.0);

        None
    }

    fn draw_mission_list(
        &self, 
        rect: Rect, 
        data: &GameData, 
        ongoing_missions: &[OngoingMission], 
        completed_missions: &[MissionOutcome], 
        completed_history: &[String],
        start_y: f32
    ) -> Option<UpdateResult> {
         let mut m_y = start_y;
         for mission in &data.missions {
             let is_ongoing = ongoing_missions.iter().any(|m| m.mission.description == mission.description);
             let is_pending = completed_missions.iter().any(|m| m.description == mission.description);
             let is_historically_complete = completed_history.contains(&mission.description);
             
             let available = if mission.repeatable {
                 !is_ongoing && !is_pending
             } else {
                 !is_ongoing && !is_pending && !is_historically_complete
             };

             if available {
                 if draw_button(Rect::new(rect.x + 20.0, m_y, rect.w - 40.0, 35.0), &format!("Mission: {}", mission.description), false) {
                      return Some(UpdateResult::new().with_transition(StateTransition::ToMissionAssignment(mission.description.clone())));
                 }
                 m_y += 40.0;
             }
         }
         None
    }

    fn draw_construction_modal(&mut self, data: &GameData, unlocked_techs: &[String]) -> Option<UpdateResult> {
         let (screen_w, screen_h) = (screen_width(), screen_height());
         draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

         let modal_w = 400.0;
         let modal_h = 500.0;
         let modal_x = (screen_w - modal_w) / 2.0;
         let modal_y = (screen_h - modal_h) / 2.0;

         draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Construction Blueprints"));

         if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
             self.crafting_modal_open = false;
         }

         let content_y = modal_y + 50.0;
         let content_h = modal_h - 70.0;
         let content_rect = Rect::new(modal_x + 10.0, content_y, modal_w - 20.0, content_h);

         // Use loaded definitions
         // Sort by cost for consistent display
         let mut build_opts: Vec<_> = data.building_definitions.values().collect();
         build_opts.sort_by_key(|a| a.cost);

         let available_defs: Vec<_> = build_opts
             .into_iter()
             .filter(|def| {
                 let req_tech = def.tech_required.clone().unwrap_or_default();
                 let tech_unlocked = unlocked_techs.contains(&req_tech) || req_tech.is_empty();
                 let already_built = def.unique && data.buildings.iter().any(|b| b.building_type == def.building_type);
                 tech_unlocked && !already_built
             })
             .collect();

         let item_height = 70.0; // Increased to fit description
         let total_height = available_defs.len() as f32 * item_height;
         if content_rect.contains(mouse_position().into()) {
             let wheel = mouse_wheel().1;
             if total_height > content_h {
                 self.construction_scroll -= wheel * 30.0;
                 self.construction_scroll = self.construction_scroll.clamp(0.0, (total_height - content_h).max(0.0));
             } else {
                 self.construction_scroll = 0.0;
             }
         }

         if available_defs.is_empty() {
             draw_text("No available buildings.", modal_x + 20.0, content_y + 20.0, FONT_BODY_SIZE, TEXT_SECONDARY);
             return None;
         }

         let mut b_y = content_y - self.construction_scroll;
         for def in available_defs {
             if b_y + 40.0 < content_y {
                 b_y += item_height;
                 continue;
             }
             if b_y > content_y + content_h {
                 break;
             }

             if draw_button(Rect::new(modal_x + 20.0, b_y, modal_w - 40.0, 40.0), &format!("{} ({} SS)", def.name, def.cost), false) {
                 self.crafting_modal_open = false;
                 self.placement_mode = Some(def.building_type.clone());
             }

             // Show building description below button
             let desc_y = b_y + 50.0;
             if desc_y < content_y + content_h {
                 // Truncate description if too long
                 let desc = if def.description.len() > 50 {
                     format!("{}...", &def.description[..47])
                 } else {
                     def.description.clone()
                 };
                 draw_text(&desc, modal_x + 25.0, desc_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
             }

             b_y += item_height;
         }

         if total_height > content_h {
             let track_x = modal_x + modal_w - 12.0;
             let track_y = content_y;
             let track_h = content_h;
             draw_rectangle(track_x, track_y, 4.0, track_h, PANEL_BORDER);

             let handle_h = (content_h * content_h / total_height).max(20.0);
             let max_offset = (total_height - content_h).max(1.0);
             let handle_y = track_y + (self.construction_scroll / max_offset) * (track_h - handle_h);
             draw_rectangle(track_x - 1.0, handle_y, 6.0, handle_h, TEXT_HIGHLIGHT);

             draw_text("Scroll", modal_x + 20.0, modal_y + modal_h - 15.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
         }
         None
    }

    fn draw_crafting_modal(
        &mut self,
        data: &GameData,
        b_type: &BuildingType,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        disciples: &[Disciple],
        discovered_recipes: &[String],
    ) -> Option<UpdateResult> {
        use crate::engine::crafting::{self, CraftingContext};

        let (screen_w, screen_h) = (screen_width(), screen_height());
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

        let modal_w = 550.0;
        let modal_h = 650.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Crafting Menu"));

        if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
            self.crafting_modal_open = false;
        }

        // Find building info for context
        let building = data.buildings.iter().find(|b| {
            b.building_type == *b_type
                && b.status == BuildingStatus::Active
        });
        let building_level = building.map(|b| b.level).unwrap_or(1);
        let building_element = building.map(|b| b.element.clone()).unwrap_or_default();

        // Find assigned disciple
        let assigned_disciple = building
            .and_then(|b| b.assigned_disciple)
            .and_then(|did| disciples.iter().find(|d| d.id == did));
        let disciple_mind = assigned_disciple.map(|d| d.attributes.mind);

        // Show building info
        let info_y = modal_y + 45.0;
        draw_text(
            &format!("Building Lv.{}", building_level),
            modal_x + 20.0,
            info_y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        if let Some(d) = assigned_disciple {
            draw_text(
                &format!("Worker: {} (Mind: {})", d.name, d.attributes.mind),
                modal_x + 150.0,
                info_y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        } else {
            draw_text(
                "No worker assigned",
                modal_x + 150.0,
                info_y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        let mut r_y = modal_y + 70.0;
        let recipes: Vec<_> = data.recipes.iter()
            .filter(|r| r.required_building == *b_type && discovered_recipes.contains(&r.id))
            .collect();

        if recipes.is_empty() {
            draw_text("No discovered recipes for this station.", modal_x + 20.0, r_y, FONT_BODY_SIZE, TEXT_SECONDARY);
        } else {
            let ctx = CraftingContext {
                building_level,
                building_element,
                disciple_mind,
            };

            for recipe in recipes {
                let success_chance = crafting::calculate_success_chance(recipe, &ctx);

                let mut can_craft = true;
                let mut ing_parts: Vec<String> = Vec::new();
                for (ing, amt) in &recipe.ingredients {
                    let has = match ing.as_str() {
                        "spirit_stones" => spirit_stones,
                        "herbs" => herbs,
                        _ => *inventory.get(ing).unwrap_or(&0),
                    };
                    if has < *amt {
                        can_craft = false;
                    }
                    ing_parts.push(format!("{}x {} ({}/{})", amt, ing, has, amt));
                }

                // Color-code by success chance
                let chance_color = if success_chance > 70 {
                    Color::new(0.2, 0.8, 0.2, 1.0) // green
                } else if success_chance > 40 {
                    Color::new(0.9, 0.8, 0.2, 1.0) // yellow
                } else {
                    Color::new(0.9, 0.3, 0.2, 1.0) // red
                };

                // Recipe name + success chance
                let label = format!("{} [{}%]", recipe.name, success_chance);

                if draw_button(Rect::new(modal_x + 20.0, r_y, modal_w - 40.0, 35.0), &label, false) {
                    if can_craft {
                        return Some(UpdateResult::new().with_action(Action::CraftItem(recipe.id.clone())));
                    }
                }

                // Draw success chance indicator
                draw_text(
                    &format!("{}%", success_chance),
                    modal_x + modal_w - 70.0,
                    r_y + 22.0,
                    FONT_SMALL_SIZE,
                    chance_color,
                );

                // Ingredient list below button
                let ing_text = ing_parts.join(", ");
                let status_text = if can_craft {
                    ing_text
                } else {
                    format!("MISSING: {}", ing_text)
                };
                let status_color = if can_craft { TEXT_SECONDARY } else { Color::new(0.8, 0.3, 0.3, 1.0) };
                draw_text(
                    &status_text,
                    modal_x + 25.0,
                    r_y + 50.0,
                    FONT_SMALL_SIZE,
                    status_color,
                );

                r_y += 65.0;

                if r_y > modal_y + modal_h - 30.0 {
                    draw_text("...", modal_x + 20.0, r_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                    break;
                }
            }
        }
        None
    }

    fn draw_spirit_beasts_view(
        &mut self,
        rect: Rect,
        data: &GameData,
        spirit_beasts: &[SpiritBeast],
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        draw_panel(rect, Some("Spirit Beasts"));

        let header_y = rect.y + 40.0;
        draw_text(
            &format!("Owned: {}", spirit_beasts.len()),
            rect.x + 20.0,
            header_y,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );

        let left_w = 280.0;
        let list_rect = Rect::new(rect.x + 10.0, header_y + 30.0, left_w, rect.h - 90.0);
        draw_panel(list_rect, Some("Roster"));

        let mut y = list_rect.y + 40.0;
        for (i, beast) in spirit_beasts.iter().enumerate() {
            let selected = self.selected_beast_index == Some(i);
            let label = format!("{} ({})", beast.name, beast.species);
            if draw_button(Rect::new(list_rect.x + 10.0, y, left_w - 20.0, 32.0), &label, selected) {
                self.selected_beast_index = Some(i);
            }
            y += 38.0;
            if y > list_rect.y + list_rect.h - 40.0 {
                break;
            }
        }

        let right_rect = Rect::new(list_rect.x + left_w + 10.0, list_rect.y, rect.w - left_w - 30.0, list_rect.h);
        draw_panel(right_rect, Some("Details"));

        if let Some(idx) = self.selected_beast_index.and_then(|i| spirit_beasts.get(i).map(|_| i)) {
            let beast = &spirit_beasts[idx];
            let mut dy = right_rect.y + 40.0;
            draw_text(&beast.name, right_rect.x + 20.0, dy, FONT_HEADER_SIZE, PRIMARY);
            dy += 24.0;
            draw_text(&format!("Species: {}", beast.species), right_rect.x + 20.0, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
            dy += 20.0;
            draw_text(&format!("Tier: {:?}", beast.tier), right_rect.x + 20.0, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
            dy += 20.0;
            draw_text(&format!("Loyalty: {}", beast.loyalty), right_rect.x + 20.0, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
            dy += 20.0;
            draw_text(&format!("Hunger: {}", beast.hunger), right_rect.x + 20.0, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
            dy += 30.0;

            draw_text("Equipment:", right_rect.x + 20.0, dy, FONT_SMALL_SIZE, TEXT_PRIMARY);
            dy += 18.0;

            let slots = [
                crate::data::spirit_beasts::BeastEquipmentSlot::Collar,
                crate::data::spirit_beasts::BeastEquipmentSlot::Harness,
                crate::data::spirit_beasts::BeastEquipmentSlot::Talisman,
                crate::data::spirit_beasts::BeastEquipmentSlot::Relic,
            ];

            for slot in slots.iter() {
                let slot_name = match slot {
                    crate::data::spirit_beasts::BeastEquipmentSlot::Collar => "Collar",
                    crate::data::spirit_beasts::BeastEquipmentSlot::Harness => "Harness",
                    crate::data::spirit_beasts::BeastEquipmentSlot::Talisman => "Talisman",
                    crate::data::spirit_beasts::BeastEquipmentSlot::Relic => "Relic",
                };

                let item_label = beast
                    .equipment
                    .iter()
                    .find(|eq| eq.slot == *slot)
                    .and_then(|eq| data.beast_equipment_definitions.get(&eq.item_id))
                    .map(|item| item.name.clone())
                    .unwrap_or_else(|| "Empty".to_string());

                draw_text(&format!("{}: {}", slot_name, item_label), right_rect.x + 30.0, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                dy += 18.0;
            }

            if draw_button(Rect::new(right_rect.x + 20.0, right_rect.y + right_rect.h - 60.0, 180.0, 36.0), "Equip Beast Gear", false) {
                self.beast_equip_modal_open = true;
            }
        }

        if self.beast_equip_modal_open {
            if let Some(idx) = self.selected_beast_index.and_then(|i| spirit_beasts.get(i).map(|_| i)) {
                if let Some(res) = self.draw_beast_equip_modal(data, spirit_beasts[idx].id, inventory) {
                    return Some(res);
                }
            } else {
                self.beast_equip_modal_open = false;
            }
        }

        if draw_button(
            Rect::new(rect.x + rect.w - 120.0, rect.y + 10.0, 100.0, 30.0),
            "Back",
            false,
        ) {
            self.view = SectView::Map;
        }

        None
    }

    fn draw_beast_equip_modal(
        &mut self,
        data: &GameData,
        beast_id: u64,
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 420.0;
        let modal_h = 420.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Beast Equipment"));

        if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "X", false) {
            self.beast_equip_modal_open = false;
        }

        let mut y = modal_y + 50.0;
        let mut items: Vec<_> = data.beast_equipment_definitions.values().collect();
        items.sort_by_key(|i| i.name.clone());

        let mut found_any = false;
        for item in items {
            let count = *inventory.get(&item.id).unwrap_or(&0);
            if count == 0 {
                continue;
            }
            found_any = true;
            let label = format!("{} ({:?}) x{}", item.name, item.slot, count);
            if draw_button(Rect::new(modal_x + 20.0, y, modal_w - 40.0, 32.0), &label, false) {
                self.beast_equip_modal_open = false;
                return Some(UpdateResult::new().with_action(Action::EquipBeastItem(beast_id, item.id.clone())));
            }
            y += 36.0;
            if y > modal_y + modal_h - 40.0 {
                break;
            }
        }

        if !found_any {
            draw_text("No beast equipment in inventory.", modal_x + 20.0, modal_y + 80.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
        }

        None
    }

    fn draw_right_panel(&self, header_h: f32, screen_h: f32, left_w: f32, center_w: f32, width: f32, event_log: &[String]) {
        let rect = Rect::new(left_w + center_w, header_h, width, screen_h - header_h);
        self.draw_event_log_panel(rect, event_log, screen_h);
    }

    fn draw_event_log_panel(&self, rect: Rect, event_log: &[String], screen_h: f32) {
        draw_panel(rect, Some("Event Log"));

        let mut log_y = rect.y + 50.0;
        let max_width = rect.w - 20.0;

        for event in event_log.iter().rev().take(20) {
            let words: Vec<&str> = event.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };

                if test_line.len() as f32 * 7.0 > max_width {
                    draw_text(&current_line, rect.x + 10.0, log_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                    log_y += 18.0;
                    current_line = word.to_string();
                } else {
                    current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                draw_text(&current_line, rect.x + 10.0, log_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                log_y += 18.0;
            }
            log_y += 4.0;
            if log_y > screen_h - 20.0 { break; }
        }
    }

    fn draw_tutorial_overlay(&self, screen_w: f32, screen_h: f32, tutorial: &mut TutorialState) {
        let total_steps = 5;
        let (title, body) = self.get_tutorial_step_text(tutorial.step);
        let header = format!("Tutorial {}/{}", (tutorial.step + 1).min(total_steps), total_steps);

        let overlay_w = 700.0;
        let overlay_h = 220.0;
        let overlay_x = (screen_w - overlay_w) / 2.0;
        let overlay_y = screen_h - overlay_h - 10.0;
        let rect = Rect::new(overlay_x, overlay_y, overlay_w, overlay_h);

        draw_panel(rect, Some(&header));

        // Portrait placeholders (left/right)
        let portrait_w = 110.0;
        let portrait_h = 140.0;
        let portrait_y = rect.y + 50.0;
        let left_portrait = Rect::new(rect.x + 10.0, portrait_y, portrait_w, portrait_h);
        let right_portrait = Rect::new(rect.x + rect.w - portrait_w - 10.0, portrait_y, portrait_w, portrait_h);
        draw_panel(left_portrait, Some("Portrait"));
        draw_panel(right_portrait, Some("Portrait"));

        let text_x = rect.x + portrait_w + 25.0;
        let text_w = rect.w - (portrait_w * 2.0) - 50.0;

        draw_text(title, text_x, rect.y + 60.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        let mut text_y = rect.y + 85.0;
        let max_width = text_w;
        let words: Vec<&str> = body.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            if test_line.len() as f32 * 7.0 > max_width {
                draw_text(&current_line, text_x, text_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                text_y += 18.0;
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }
        if !current_line.is_empty() {
            draw_text(&current_line, text_x, text_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
        }

        // Controls
        let btn_y = rect.y + rect.h - 40.0;
        if draw_button(Rect::new(rect.x + rect.w - 190.0, btn_y, 80.0, 30.0), "Hide", false) {
            tutorial.hidden = true;
        }
        if draw_button(Rect::new(rect.x + rect.w - 100.0, btn_y, 80.0, 30.0), "Skip", false) {
            tutorial.active = false;
        }
    }

    fn update_tutorial_progress(
        &self,
        tutorial: &mut TutorialState,
        data: &GameData,
        unlocked_techs: &[String],
        disciples: &[Disciple],
        ongoing_missions: &[OngoingMission],
        completed_history: &[String],
    ) {
        if !tutorial.active {
            return;
        }

        let total_steps = 5;
        while tutorial.step < total_steps
            && self.is_tutorial_step_complete(
                tutorial.step,
                data,
                unlocked_techs,
                disciples,
                ongoing_missions,
                completed_history,
            )
        {
            tutorial.step += 1;
        }

        if tutorial.step >= total_steps {
            tutorial.active = false;
        }
    }

    fn is_tutorial_step_complete(
        &self,
        step: usize,
        data: &GameData,
        unlocked_techs: &[String],
        disciples: &[Disciple],
        ongoing_missions: &[OngoingMission],
        completed_history: &[String],
    ) -> bool {
        match step {
            0 => data.buildings.iter().any(|b| {
                b.building_type == BuildingType::SectHall && b.status == BuildingStatus::Active
            }),
            1 => unlocked_techs.iter().any(|t| t == "sect_administration"),
            2 => data.buildings.iter().any(|b| b.building_type == BuildingType::MissionBoard),
            3 => disciples.len() > 1,
            4 => !ongoing_missions.is_empty() || !completed_history.is_empty(),
            _ => true,
        }
    }

    fn get_tutorial_step_text(&self, step: usize) -> (&'static str, &'static str) {
        match step {
            0 => (
                "Restore the Sect Hall",
                "Select the ruined Sect Hall and choose Restore (50 SS).",
            ),
            1 => (
                "Unlock the Mission Board",
                "Open Research / Tech and learn Sect Administration (0 SS).",
            ),
            2 => (
                "Build the Mission Board",
                "Open Construction and place a Mission Board on the map.",
            ),
            3 => (
                "Recruit a Disciple",
                "Select the Sect Hall and press Recruit to bring in help.",
            ),
            4 => (
                "Send a Mission",
                "Open the Mission Board, assign a team, and depart on a mission.",
            ),
            _ => ("Tutorial Complete", "All steps finished."),
        }
    }

    fn draw_settings_modal(&mut self, screen_w: f32, screen_h: f32) -> Option<UpdateResult> {
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        
        let modal_w = 300.0;
        let modal_h = 250.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;
        let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);
        
        draw_panel(modal_rect, Some("Settings"));
        
        if draw_button(Rect::new(modal_x + 50.0, modal_y + 60.0, 200.0, 40.0), "Save Game", false) {
             return Some(UpdateResult::new().with_action(Action::SaveGame));
        }

        if draw_button(Rect::new(modal_x + 50.0, modal_y + 120.0, 200.0, 40.0), "Exit to Menu", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
        }
        
        if draw_button(Rect::new(modal_x + 50.0, modal_y + 180.0, 200.0, 40.0), "Close", false) {
            self.settings_open = false;
        }
        None
    }
}