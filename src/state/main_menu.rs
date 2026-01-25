use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::draw_button;
use crate::ui::theme::{FONT_TITLE_SIZE, PRIMARY};
use macroquad::prelude::*;

pub struct MainMenuState {
    start_btn: Rect,
    load_btn: Rect,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            start_btn: Rect::new(screen_width() / 2.0 - 100.0, 300.0, 200.0, 50.0),
            load_btn: Rect::new(screen_width() / 2.0 - 100.0, 370.0, 200.0, 50.0),
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        // Recalculate positions in case of window resize (basic responsiveness)
        self.start_btn.x = screen_width() / 2.0 - 100.0;
        self.load_btn.x = screen_width() / 2.0 - 100.0;

        if draw_button(self.start_btn, "Start Journey", true) {
            return UpdateResult::new().with_transition(StateTransition::ToSectCreation);
        }

        if draw_button(self.load_btn, "Load Game", true) {
             return UpdateResult::new().with_action(crate::engine::actions::Action::LoadGame);
        }

        if is_key_pressed(KeyCode::Space) {
            return UpdateResult::new().with_transition(StateTransition::ToSectCreation);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        let title = "HEAVENLY MANDATE";
        let dims = measure_text(title, None, FONT_TITLE_SIZE as u16, 1.0);
        draw_text(
            title, 
            screen_width() / 2.0 - dims.width / 2.0, 
            150.0, 
            FONT_TITLE_SIZE, 
            PRIMARY
        );
    }
}