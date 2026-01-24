use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use macroquad::prelude::*;

impl MainMenuState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self) -> UpdateResult {
        if is_key_pressed(KeyCode::Space) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }
        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        draw_text("HEAVENLY MANDATE", 20.0, 50.0, 50.0, WHITE);
        draw_text("Press SPACE to Start", 20.0, 100.0, 30.0, WHITE);
    }
}

pub struct MainMenuState;