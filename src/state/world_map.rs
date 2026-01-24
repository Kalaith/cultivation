use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use macroquad::prelude::*;

pub struct WorldMapState;

impl WorldMapState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }
        UpdateResult::new()
    }

    pub fn draw(&self, data: &GameData, _spirit_stones: u32) {
        draw_text("WORLD MAP", 20.0, 40.0, 40.0, WHITE);
        draw_text("Press ESC to return to Sect Base", 20.0, 70.0, 20.0, GRAY);

        for node in &data.map_nodes {
            draw_circle(node.x, node.y, 15.0, RED);
            draw_text(&node.name, node.x + 20.0, node.y, 24.0, WHITE);
            draw_text(
                &format!("Danger: {}", node.danger_level),
                node.x + 20.0,
                node.y + 25.0,
                20.0,
                GRAY,
            );
        }
    }
}
