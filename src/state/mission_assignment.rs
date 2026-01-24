use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use macroquad::prelude::*;

pub struct MissionAssignmentState {
    pub mission_description: String,
    selected_disciples: Vec<usize>,
    start_button_rect: Rect,
}

impl MissionAssignmentState {
    pub fn new(mission_description: String) -> Self {
        Self {
            mission_description,
            selected_disciples: Vec::new(),
            start_button_rect: Rect::new(550.0, 650.0, 200.0, 50.0),
        }
    }

    pub fn update(&mut self, disciples: &[Disciple]) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let mouse_pos = mouse_position().into();
        
        // Handle disciple selection
        let mut y = 150.0;
        for (i, _disciple) in disciples.iter().enumerate() {
            let rect = Rect::new(20.0, y - 10.0, 500.0, 35.0);
            if rect.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left) {
                if let Some(pos) = self.selected_disciples.iter().position(|&idx| idx == i) {
                    self.selected_disciples.remove(pos);
                } else if self.selected_disciples.len() < 3 {
                    self.selected_disciples.push(i);
                }
            }
            y += 40.0;
        }

        // Handle start button click
        if !self.selected_disciples.is_empty()
            && self.start_button_rect.contains(mouse_pos)
            && is_mouse_button_pressed(MouseButton::Left)
        {
            return UpdateResult::new()
                .with_action(Action::StartMission(
                    self.mission_description.clone(),
                    self.selected_disciples.clone(),
                ))
                .with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, disciples: &[Disciple], _spirit_stones: u32) {
        draw_text("ASSIGN DISCIPLES", 20.0, 40.0, 40.0, WHITE);
        draw_text(&self.mission_description, 20.0, 100.0, 24.0, WHITE);
        draw_text("Press ESC to return to Sect Base", 20.0, 70.0, 20.0, GRAY);

        // Draw disciple list
        let mut y = 150.0;
        for (i, disciple) in disciples.iter().enumerate() {
            let color = if self.selected_disciples.contains(&i) {
                LIME
            } else {
                WHITE
            };
            let text = format!(
                "Name: {}, Realm: {:?}, Talent: {:?}",
                disciple.name, disciple.realm, disciple.talent
            );
            draw_text(&text, 20.0, y, 24.0, color);
            y += 40.0;
        }

        // Draw Start button
        let btn_color = if self.selected_disciples.is_empty() {
            DARKGRAY
        } else {
            GREEN
        };
        draw_rectangle(
            self.start_button_rect.x,
            self.start_button_rect.y,
            self.start_button_rect.w,
            self.start_button_rect.h,
            btn_color,
        );
        draw_text(
            "Start Mission",
            self.start_button_rect.x + 20.0,
            self.start_button_rect.y + 35.0,
            30.0,
            BLACK,
        );
    }
}
