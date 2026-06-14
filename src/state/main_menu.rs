use crate::data::loader::GameData;
use crate::save::storage;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::draw_button;
use crate::ui::theme::{FONT_SMALL_SIZE, FONT_TITLE_SIZE, PRIMARY, TEXT_HIGHLIGHT, TEXT_SECONDARY};
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub struct MainMenuState {
    has_save: bool,
    checked_save: bool,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            has_save: false,
            checked_save: false,
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        // Check for existing save on first update
        if !self.checked_save {
            self.has_save = storage::exists();
            self.checked_save = true;
        }

        let center_x = screen_width() / 2.0;
        let btn_width = 220.0;
        let btn_height = 50.0;
        let btn_x = center_x - btn_width / 2.0;

        let mut btn_y = 280.0;

        // Continue button (if save exists)
        if self.has_save {
            let continue_btn = Rect::new(btn_x, btn_y, btn_width, btn_height);
            if draw_button(continue_btn, "Continue", true) {
                return UpdateResult::new().with_action(crate::engine::actions::Action::LoadGame);
            }
            btn_y += 60.0;
        }

        // New Game button
        let new_game_btn = Rect::new(btn_x, btn_y, btn_width, btn_height);
        let new_game_label = if self.has_save {
            "New Game"
        } else {
            "Start Journey"
        };
        if draw_button(new_game_btn, new_game_label, !self.has_save) {
            return UpdateResult::new().with_action(crate::engine::actions::Action::StartNewGame(
                "Test".to_string(),
            ));
        }
        btn_y += 60.0;

        // Warning if save exists and hovering new game
        if self.has_save && new_game_btn.contains(mouse_position().into()) {
            draw_ui_text(
                "(This will overwrite your save)",
                center_x - 110.0,
                btn_y - 15.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        if is_key_pressed(KeyCode::Space) {
            return UpdateResult::new().with_transition(StateTransition::ToSectCreation);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        let title = "HEAVENLY MANDATE";
        let dims = measure_ui_text(title, None, FONT_TITLE_SIZE as u16, 1.0);
        draw_ui_text(
            title,
            screen_width() / 2.0 - dims.width / 2.0,
            150.0,
            FONT_TITLE_SIZE,
            PRIMARY,
        );

        // Subtitle
        let subtitle = "A Cultivation Sect Management Game";
        let sub_dims = measure_ui_text(subtitle, None, 20, 1.0);
        draw_ui_text(
            subtitle,
            screen_width() / 2.0 - sub_dims.width / 2.0,
            190.0,
            20.0,
            TEXT_SECONDARY,
        );

        // Version info
        draw_ui_text(
            "v0.1.0",
            screen_width() - 60.0,
            screen_height() - 20.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );

        // Save indicator
        if self.has_save {
            draw_ui_text(
                "Save found",
                20.0,
                screen_height() - 20.0,
                FONT_SMALL_SIZE,
                TEXT_HIGHLIGHT,
            );
        }
    }
}
