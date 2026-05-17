mod building_details;
mod crafting;
mod map_view;
mod panels;
mod spirit_beasts;
mod tutorial;

use crate::data::buildings::{BuildingStatus, BuildingType};
use crate::data::disciples::Disciple;
use crate::data::elements::Element;
use crate::data::grid::Grid;
use crate::data::herbs::Season;
use crate::data::loader::GameData;
use crate::data::missions::{MissionOutcome, OngoingMission};
use crate::data::spirit_beasts::SpiritBeast;
use crate::engine::actions::Action;
use crate::state::{StateTransition, TutorialState, UpdateResult};
use crate::ui::components::*;
use crate::ui::herbs;
use crate::ui::tech::{self, TechTreeState};
use crate::ui::theme::*;
use crate::ui::TextureManager;
use macroquad::prelude::*;
use std::collections::HashMap;

pub(super) const SECT_MAP_WIDTH: f32 = 4096.0;
pub(super) const SECT_MAP_HEIGHT: f32 = 4096.0;
pub(super) const BUILDING_SCALE: f32 = 0.25;
pub(super) const ALPHA_THRESHOLD: u8 = 10;

#[derive(Clone, PartialEq)]
pub(super) enum SectView {
    Map,
    BuildingDetails(u64),
    SpiritBeasts,
}

pub struct SectBaseState {
    pub(super) view: SectView,
    pub(super) settings_open: bool,
    pub(super) feng_shui_overlay_active: bool,
    pub(super) crafting_modal_open: bool,
    pub(super) tech_tree_open: bool,
    pub(super) tech_tree_state: TechTreeState,
    pub(super) construction_scroll: f32,
    pub(super) placement_mode: Option<BuildingType>,
    pub(super) map_zoom: f32,
    pub(super) map_center: Vec2,
    pub(super) map_dragging: bool,
    pub(super) map_last_mouse: Vec2,
    pub(super) building_mask_cache: HashMap<String, Image>,
    pub(super) selected_beast_index: Option<usize>,
    pub(super) beast_equip_modal_open: bool,
    pub(super) selected_mission: Option<String>,
    pub(super) herb_planting_modal: Option<usize>,
    pub(super) disciple_assignment_modal: bool,
    pub(super) infusion_modal_open: bool,
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
            selected_mission: None,
            herb_planting_modal: None,
            disciple_assignment_modal: false,
            infusion_modal_open: false,
        }
    }

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
        if let Some(res) = self.handle_global_input() {
            return res;
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;
        let left_panel_w = 250.0;
        let right_panel_w = 250.0;
        let center_w = screen_w - left_panel_w - right_panel_w - 20.0;

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

        self.draw_header(
            screen_w,
            header_h,
            spirit_stones,
            herbs,
            influence,
            relics,
            current_season,
            season_ticks,
            tutorial,
        );

        if let Some(res) = self.draw_left_panel(header_h, screen_h, left_panel_w, data) {
            return res;
        }

        self.update_tutorial_progress(
            tutorial,
            data,
            unlocked_techs,
            disciples,
            ongoing_missions,
            completed_history,
        );
        self.draw_right_panel(
            header_h,
            screen_h,
            left_panel_w,
            center_w,
            right_panel_w,
            event_log,
        );

        if tutorial.active && !tutorial.hidden {
            self.draw_tutorial_overlay(screen_w, screen_h, tutorial);
        }

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

    fn handle_global_input(&mut self) -> Option<UpdateResult> {
        if !is_key_pressed(KeyCode::Escape) {
            return None;
        }

        if self.settings_open {
            self.settings_open = false;
        } else if self.tech_tree_open {
            self.tech_tree_open = false;
        } else if self.crafting_modal_open {
            self.crafting_modal_open = false;
        } else if self.placement_mode.is_some() {
            self.placement_mode = None;
        } else if let SectView::BuildingDetails(_) = self.view {
            self.view = SectView::Map;
        } else if let SectView::SpiritBeasts = self.view {
            self.view = SectView::Map;
        } else {
            return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
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
            SectView::Map => {
                self.draw_map_view(rect, data, grid, textures, spirit_stones, unlocked_techs)
            }
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
            SectView::SpiritBeasts => {
                self.draw_spirit_beasts_view(rect, data, spirit_beasts, inventory)
            }
        }
    }
}
