use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct MissionAssignmentState {
    pub mission_description: String,
    selected_disciples: Vec<usize>,
}

impl MissionAssignmentState {
    pub fn new(mission_description: String) -> Self {
        Self {
            mission_description,
            selected_disciples: Vec::new(),
        }
    }

    pub fn update(&mut self, disciples: &[Disciple]) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;

        // --- Header ---
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("ASSIGN DISCIPLES", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        draw_text(&format!("Mission: {}", self.mission_description), 400.0, 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        // --- Layout ---
        let content_y = header_h + 10.0;
        let content_h = screen_h - content_y - 10.0;
        let left_w = screen_w - 300.0;
        
        let list_rect = Rect::new(10.0, content_y, left_w, content_h);
        draw_panel(list_rect, Some("Select Disciples (Max 3)"));
        
        let mut btn_y = list_rect.y + 50.0;
        for (i, disciple) in disciples.iter().enumerate() {
            let is_selected = self.selected_disciples.contains(&i);
            let label = format!("{} ({:?}) - Power estimate: {}", 
                disciple.name, disciple.realm, 
                // Simple power estimate for display
                match disciple.realm {
                    crate::data::disciples::CultivationRealm::Mortal => 1,
                    crate::data::disciples::CultivationRealm::QiRefinement => 2,
                    crate::data::disciples::CultivationRealm::FoundationEstablishment => 4,
                    crate::data::disciples::CultivationRealm::CoreFormation => 7,
                }
            );

            if draw_button(Rect::new(list_rect.x + 10.0, btn_y, left_w - 20.0, 40.0), &label, is_selected) {
                if is_selected {
                    if let Some(pos) = self.selected_disciples.iter().position(|&idx| idx == i) {
                        self.selected_disciples.remove(pos);
                    }
                } else if self.selected_disciples.len() < 3 {
                    self.selected_disciples.push(i);
                }
            }
            btn_y += 50.0;
        }

        // --- Right Panel: Action ---
        let right_rect = Rect::new(left_w + 20.0, content_y, 250.0, content_h);
        draw_panel(right_rect, Some("Orders"));

        draw_text("Selected Team:", right_rect.x + 20.0, right_rect.y + 60.0, FONT_BODY_SIZE, TEXT_SECONDARY);
        let mut y = right_rect.y + 90.0;
        for &idx in &self.selected_disciples {
            if let Some(d) = disciples.get(idx) {
                draw_text(&format!("- {}", d.name), right_rect.x + 20.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
                y += 25.0;
            }
        }

        let start_active = !self.selected_disciples.is_empty();
        let btn_label = if start_active { "Depart" } else { "Select Team" };
        
        if draw_button(Rect::new(right_rect.x + 20.0, right_rect.h - 80.0, 210.0, 50.0), btn_label, start_active) {
            if start_active {
                return UpdateResult::new()
                    .with_action(Action::StartMission(
                        self.mission_description.clone(),
                        self.selected_disciples.clone(),
                    ))
                    .with_transition(StateTransition::ToSectBase);
            }
        }
        
        // Back Button
        if draw_button(Rect::new(right_rect.x + 20.0, right_rect.h - 140.0, 210.0, 40.0), "Cancel", false) {
             return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _disciples: &[Disciple], _spirit_stones: u32) {
        // Handled in update
    }
}
