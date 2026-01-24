use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use macroquad::prelude::*;

pub struct DiscipleRosterState;

impl DiscipleRosterState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }
        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, disciples: &[Disciple], _spirit_stones: u32) {
        draw_text("DISCIPLE ROSTER", 20.0, 40.0, 40.0, WHITE);
        draw_text("Press ESC to return to Sect Base", 20.0, 70.0, 20.0, GRAY);

        let mut y = 120.0;
        for disciple in disciples {
            let text = format!(
                "Name: {}, Realm: {:?}, Talent: {:?}, EXP: {} / {}",
                disciple.name, disciple.realm, disciple.talent, disciple.exp, disciple.exp_to_next_level
            );
            draw_text(&text, 20.0, y, 24.0, WHITE);
            y += 30.0;
        }
    }
}
