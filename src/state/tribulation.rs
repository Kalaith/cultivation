use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::engine::tribulation::TribulationState;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct TribulationEncounterState {
    pub tribulation: TribulationState,
    pub disciple_index: usize,
}

impl TribulationEncounterState {
    pub fn new(tribulation: TribulationState, disciple_index: usize) -> Self {
        Self {
            tribulation,
            disciple_index,
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        // If finished, show result and allow exit
        if self.tribulation.is_finished {
            if draw_button(
                Rect::new(
                    screen_width() / 2.0 - 100.0,
                    screen_height() - 100.0,
                    200.0,
                    50.0,
                ),
                "Continue",
                true,
            ) {
                return UpdateResult::new().with_transition(StateTransition::ToSectBase);
            }
            return UpdateResult::new(); // Wait for user
        }

        // Auto-progress waves for MVP or wait for user "Next Wave"?
        // Let's make it manual "Endure Wave" button

        // Input handling
        // ... (Optional items usage here later)

        UpdateResult::new()
    }

    pub fn draw(&mut self, _data: &GameData, disciples: &[Disciple]) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Title
        draw_text(
            &self.tribulation.config.name,
            20.0,
            50.0,
            FONT_TITLE_SIZE,
            Color::new(0.8, 0.2, 0.2, 1.0),
        );
        draw_text(
            &self.tribulation.config.description,
            20.0,
            80.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );

        // Disciple Status
        if let Some(disciple) = disciples.get(self.disciple_index) {
            let cx = screen_w / 2.0;
            let cy = screen_h / 2.0;

            // Draw Disciple Sprite/Box
            draw_rectangle(cx - 30.0, cy - 30.0, 60.0, 60.0, PRIMARY);
            draw_text(
                &disciple.name,
                cx - 40.0,
                cy + 50.0,
                FONT_BODY_SIZE,
                TEXT_PRIMARY,
            );

            // HP Bar
            let hp_pct =
                self.tribulation.disciple_hp as f32 / self.tribulation.disciple_max_hp as f32;
            draw_progress_bar(
                Rect::new(cx - 100.0, cy + 60.0, 200.0, 20.0),
                hp_pct,
                Color::new(0.8, 0.1, 0.1, 1.0),
            );
            draw_text(
                &format!(
                    "{}/{}",
                    self.tribulation.disciple_hp, self.tribulation.disciple_max_hp
                ),
                cx - 20.0,
                cy + 75.0,
                FONT_SMALL_SIZE,
                TEXT_PRIMARY,
            );
        }

        // Wave Status
        draw_text(
            &format!(
                "Wave: {} / {}",
                self.tribulation.current_wave, self.tribulation.config.total_waves
            ),
            screen_w - 200.0,
            50.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );

        // Logs
        let log_x = 20.0;
        let mut log_y = screen_h - 250.0;
        draw_panel(
            Rect::new(log_x, log_y, screen_w - 40.0, 200.0),
            Some("Event Log"),
        );
        log_y += 40.0;

        // Show last 5 logs
        for log in self.tribulation.log.iter().rev().take(5) {
            draw_text(log, log_x + 10.0, log_y, FONT_BODY_SIZE, TEXT_SECONDARY);
            log_y += 25.0;
        }

        // Actions
        if !self.tribulation.is_finished {
            if draw_button(
                Rect::new(screen_w / 2.0 - 100.0, screen_h - 350.0, 200.0, 50.0),
                "Endure Lightning",
                true,
            ) {
                if let Some(disciple) = disciples.get(self.disciple_index) {
                    self.tribulation.process_wave(disciple);
                }
            }
        } else {
            let result_text = if self.tribulation.survived {
                "SUCCESS"
            } else {
                "FAILURE"
            };
            let color = if self.tribulation.survived {
                Color::new(0.2, 0.8, 0.2, 1.0)
            } else {
                Color::new(0.8, 0.2, 0.2, 1.0)
            };
            draw_text(
                result_text,
                screen_w / 2.0 - 60.0,
                screen_h / 2.0 - 100.0,
                60.0,
                color,
            );
        }
    }
}
