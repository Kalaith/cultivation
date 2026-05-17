use crate::data::loader::GameData;
use crate::data::missions::MissionOutcome;
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct MissionResolutionState {
    current_outcome: Option<MissionOutcome>,
}

impl MissionResolutionState {
    pub fn new() -> Self {
        Self {
            current_outcome: None,
        }
    }

    pub fn update(&mut self, completed_missions: &mut Vec<MissionOutcome>) -> UpdateResult {
        if self.current_outcome.is_none() {
            if let Some(outcome) = completed_missions.pop() {
                self.current_outcome = Some(outcome);
            } else {
                return UpdateResult::new().with_transition(StateTransition::ToSectBase);
            }
        }

        if let Some(outcome) = &self.current_outcome {
            // Draw Modal
            let screen_w = screen_width();
            let screen_h = screen_height();
            let w = 600.0;
            let h = 500.0;
            let x = (screen_w - w) / 2.0;
            let y = (screen_h - h) / 2.0;
            let rect = Rect::new(x, y, w, h);

            // Dim background
            draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));

            draw_panel(rect, Some("Mission Report"));

            let max_text_width = w - 40.0;
            let font_size = FONT_HEADER_SIZE;
            // Simple manual wrapping
            let words: Vec<&str> = outcome.mission_name.split_whitespace().collect();
            let mut line = String::new();
            let mut draw_y = y + 60.0;

            for word in words {
                let test_line: String = if line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", line, word)
                };

                let dims = measure_text(&test_line, None, font_size as u16, 1.0);
                if dims.width > max_text_width {
                    // Draw current line and start new
                    let line_dims = measure_text(&line, None, font_size as u16, 1.0);
                    draw_text(
                        &line,
                        x + (w - line_dims.width) / 2.0,
                        draw_y,
                        font_size,
                        PRIMARY,
                    );
                    draw_y += 30.0;
                    line = word.to_string();
                } else {
                    line = test_line;
                }
            }
            if !line.is_empty() {
                let line_dims = measure_text(&line, None, font_size as u16, 1.0);
                draw_text(
                    &line,
                    x + (w - line_dims.width) / 2.0,
                    draw_y,
                    font_size,
                    PRIMARY,
                );
                draw_y += 30.0;
            }

            let (status_text, color) = if outcome.success {
                ("SUCCESS", SUCCESS)
            } else {
                ("FAILURE", FAILURE)
            };
            let status_dims = measure_text(status_text, None, FONT_TITLE_SIZE as u16, 1.0);
            draw_y += 20.0;
            draw_text(
                status_text,
                x + (w - status_dims.width) / 2.0,
                draw_y,
                FONT_TITLE_SIZE,
                color,
            );

            // Logs
            let mut log_y = draw_y + 50.0;
            for log in &outcome.logs {
                if log_y > y + h - 100.0 {
                    break;
                }
                draw_text(log, x + 40.0, log_y, FONT_BODY_SIZE, TEXT_SECONDARY);
                log_y += 25.0;
            }

            // Claim Button
            if draw_button(
                Rect::new(x + (w - 200.0) / 2.0, y + h - 70.0, 200.0, 50.0),
                "Claim Rewards",
                true,
            ) {
                let action = Action::ClaimRewards(outcome.clone());
                self.current_outcome = None;
                return UpdateResult::new().with_action(action);
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        // Handled in update
    }
}
