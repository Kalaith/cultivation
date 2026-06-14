use crate::data::history::DeceasedDisciple;
use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub struct LibraryState;

impl LibraryState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(
        &mut self,
        _data: &GameData,
        spirit_stones: u32,
        deceased: &[DeceasedDisciple],
    ) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;

        // --- Header ---
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_ui_text("LIBRARY PAVILION", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        let res_text = format!("Spirit Stones: {}", spirit_stones);
        let res_dims = measure_ui_text(&res_text, None, FONT_HEADER_SIZE as u16, 1.0);
        draw_ui_text(
            &res_text,
            screen_w - res_dims.width - 20.0,
            40.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );

        // --- Layout ---
        let content_y = header_h + 10.0;
        let content_h = screen_h - content_y - 10.0;
        let left_w = screen_w * 0.4;
        let right_w = screen_w - left_w - 30.0;

        // --- Left Panel: Hall of Fallen ---
        let left_rect = Rect::new(10.0, content_y, left_w, content_h);
        draw_panel(left_rect, Some("Hall of Fallen"));

        if deceased.is_empty() {
            draw_ui_text(
                "No disciples have fallen on the path... yet.",
                left_rect.x + 20.0,
                left_rect.y + 60.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        } else {
            let mut y = left_rect.y + 60.0;
            for (i, d) in deceased.iter().enumerate() {
                if y > left_rect.y + left_rect.h - 30.0 {
                    break;
                } // Clip

                let text = format!("{}. {} ({:?})", i + 1, d.name, d.realm_at_death);
                draw_ui_text(&text, left_rect.x + 20.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
                draw_ui_text(
                    &format!("Cause: {}", d.cause_of_death),
                    left_rect.x + 30.0,
                    y + 20.0,
                    FONT_SMALL_SIZE,
                    ACCENT,
                );
                y += 50.0;
            }
        }

        // --- Right Panel: Scriptures (Placeholder) ---
        let right_rect = Rect::new(left_w + 20.0, content_y, right_w, content_h);
        draw_panel(right_rect, Some("Scripture Pavilion"));
        draw_ui_text(
            "Coming Soon: Ancient Techniques & Doctrines",
            right_rect.x + 20.0,
            right_rect.y + 60.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );

        // Back Button
        if draw_button(
            Rect::new(screen_w - 120.0, screen_h - 50.0, 100.0, 40.0),
            "Back",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32, _deceased: &[DeceasedDisciple]) {
        // Draw logic moved to update
    }
}
