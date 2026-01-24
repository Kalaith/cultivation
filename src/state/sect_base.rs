use crate::data::buildings::BuildingType;
use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct SectBaseState {
    selected_building: Option<BuildingType>,
    settings_open: bool,
}

impl SectBaseState {
    pub fn new() -> Self {
        Self {
            selected_building: None,
            settings_open: false,
        }
    }

    /// Update handling immediate mode UI
    pub fn update(&mut self, data: &GameData, spirit_stones: u32, herbs: u32, event_log: &[String]) -> UpdateResult {
        // --- Navigation Keys (Keep for accessibility) ---
        if is_key_pressed(KeyCode::Escape) {
            if self.settings_open {
                self.settings_open = false;
            } else {
                return UpdateResult::new().with_transition(StateTransition::ToMainMenu);
            }
        }

        // --- Layout Constants ---
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        let header_h = 60.0;
        let footer_h = 0.0; // No footer needed now
        let left_panel_w = 250.0;
        let right_panel_w = 250.0;
        // Calculate center width dynamically
        let center_w = screen_w - left_panel_w - right_panel_w - 20.0; 

        // --- Header (Resources) ---
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("SECT MANAGEMENT", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        
        let res_text = format!("Spirit Stones: {}   Herbs: {}", spirit_stones, herbs);
        let res_dims = measure_text(&res_text, None, FONT_HEADER_SIZE as u16, 1.0);
        // Position resources to the left of the Cog button area
        draw_text(&res_text, screen_w - res_dims.width - 60.0, 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        // Cog Button (Settings)
        if draw_button(Rect::new(screen_w - 50.0, 10.0, 40.0, 40.0), "O", false) {
            self.settings_open = !self.settings_open;
        }

        // --- Left Panel (Buildings) ---
        let left_rect = Rect::new(0.0, header_h, left_panel_w, screen_h - header_h);
        draw_panel(left_rect, Some("Buildings"));
        
        // Navigation Buttons inside Left Panel (Bottom)
        let nav_y_start = left_rect.y + left_rect.h - 120.0;
        if draw_button(Rect::new(left_rect.x + 10.0, nav_y_start, left_panel_w - 20.0, 40.0), "Disciples", false) {
             return UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster);
        }
        if draw_button(Rect::new(left_rect.x + 10.0, nav_y_start + 50.0, left_panel_w - 20.0, 40.0), "World Map", false) {
             return UpdateResult::new().with_transition(StateTransition::ToWorldMap);
        }

        // Building List
        let building_types = [
            BuildingType::SectHall,
            BuildingType::TrainingYard,
            BuildingType::LibraryPavilion,
            BuildingType::MissionBoard,
            BuildingType::SpiritGarden,
        ];

        let mut btn_y = header_h + 50.0;
        for b_type in building_types {
            let name = format!("{}", b_type);
            let active = Some(b_type.clone()) == self.selected_building;
            
            if draw_button(Rect::new(left_rect.x + 10.0, btn_y, left_panel_w - 20.0, 40.0), &name, active) {
                self.selected_building = Some(b_type.clone());
            }
            btn_y += 50.0;
        }

        // --- Center Panel (Content) ---
        let center_rect = Rect::new(left_panel_w, header_h, center_w, screen_h - header_h);
        draw_panel(center_rect, Some(if let Some(_b) = &self.selected_building { "Details" } else { "Welcome" }));

        if let Some(selected) = &self.selected_building {
             if let Some(building) = data.buildings.get(selected) {
                 draw_text(&format!("{}", selected), center_rect.x + 20.0, center_rect.y + 60.0, FONT_HEADER_SIZE, PRIMARY);
                 draw_text(&format!("Level: {}", building.level), center_rect.x + 20.0, center_rect.y + 90.0, FONT_BODY_SIZE, TEXT_PRIMARY);
                 
                 let info = match selected {
                     BuildingType::SectHall => format!("Max Disciples: {}", building.get_max_disciples()),
                     BuildingType::TrainingYard => format!("Cultivation Mult: x{:.2}", building.get_cultivation_multiplier()),
                     BuildingType::SpiritGarden => format!("Passive Income: {}/tick", building.get_passive_income()),
                     _ => String::new(),
                 };
                 draw_text(&info, center_rect.x + 20.0, center_rect.y + 120.0, FONT_BODY_SIZE, TEXT_SECONDARY);

                 // Actions
                 if *selected == BuildingType::MissionBoard {
                     // Mission list
                     let mut m_y = center_rect.y + 150.0;
                     for mission in &data.missions {
                         if draw_button(Rect::new(center_rect.x + 20.0, m_y, center_w - 40.0, 35.0), &format!("Mission: {}", mission.description), false) {
                              return UpdateResult::new().with_transition(StateTransition::ToMissionAssignment(mission.description.clone()));
                         }
                         m_y += 40.0;
                     }
                 } else {
                     // Upgrade Button
                     if draw_button(Rect::new(center_rect.x + 20.0, center_rect.y + 200.0, 150.0, 40.0), "Upgrade (50 SS)", false) {
                         return UpdateResult::new().with_action(Action::UpgradeBuilding(selected.clone()));
                     }
                     if *selected == BuildingType::SectHall {
                         if draw_button(Rect::new(center_rect.x + 180.0, center_rect.y + 200.0, 150.0, 40.0), "Recruit", false) {
                             return UpdateResult::new().with_action(Action::RecruitDisciple);
                         }
                     }
                 }
             }
        } else {
             draw_text("Select a building to manage.", center_rect.x + 20.0, center_rect.y + 60.0, FONT_BODY_SIZE, TEXT_SECONDARY);
        }

        // --- Right Panel (Event Log) ---
        let right_rect = Rect::new(left_panel_w + center_w, header_h, right_panel_w, screen_h - header_h);
        draw_panel(right_rect, Some("Event Log"));
        
        let mut log_y = right_rect.y + 50.0;
        // Show last 20 events reversed
        for event in event_log.iter().rev().take(20) {
            // Simple word wrap or just truncate for now? Let's truncate to fit width
            // In a real app we'd wrap, but truncation prevents overlap
            let mut display_text = event.clone();
            if display_text.len() > 30 {
                 display_text.truncate(27);
                 display_text.push_str("...");
            }
            draw_text(&display_text, right_rect.x + 10.0, log_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
            log_y += 20.0;
        }

        // --- Settings Overlay ---
        if self.settings_open {
            // Dim background
            draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
            
            let modal_w = 300.0;
            let modal_h = 250.0;
            let modal_x = (screen_w - modal_w) / 2.0;
            let modal_y = (screen_h - modal_h) / 2.0;
            let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);
            
            draw_panel(modal_rect, Some("Settings"));
            
            if draw_button(Rect::new(modal_x + 50.0, modal_y + 60.0, 200.0, 40.0), "Save Game", false) {
                return UpdateResult::new().with_action(Action::SaveGame);
            }

            if draw_button(Rect::new(modal_x + 50.0, modal_y + 120.0, 200.0, 40.0), "Exit to Menu", false) {
                 return UpdateResult::new().with_transition(StateTransition::ToMainMenu);
            }
            
            if draw_button(Rect::new(modal_x + 50.0, modal_y + 180.0, 200.0, 40.0), "Close", false) {
                self.settings_open = false;
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        // Drawing handled in update for immediate mode
    }
}