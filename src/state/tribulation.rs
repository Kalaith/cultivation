use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::engine::tribulation::TribulationState;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

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

    pub fn update(&mut self, disciples: &[Disciple]) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) && self.tribulation.is_finished {
            return self.resolve_and_return();
        }

        if self.tribulation.is_finished {
            if continue_rect().contains(mouse_position().into())
                && is_mouse_button_pressed(MouseButton::Left)
            {
                return self.resolve_and_return();
            }
            return UpdateResult::new();
        }

        if endure_rect().contains(mouse_position().into())
            && is_mouse_button_pressed(MouseButton::Left)
        {
            if let Some(disciple) = disciples.get(self.disciple_index) {
                self.tribulation.process_wave(disciple);
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, disciples: &[Disciple]) {
        draw_mountain_sect_backdrop();
        draw_storm_overlay();
        self.draw_header(disciples);

        if let Some(disciple) = disciples.get(self.disciple_index) {
            self.draw_trial_scene(disciple);
            self.draw_disciple_vitals(disciple);
        } else {
            draw_missing_disciple_notice();
        }

        self.draw_heavenly_record();
        self.draw_action_area();
        self.draw_log_panel();
    }

    fn resolve_and_return(&self) -> UpdateResult {
        UpdateResult::new()
            .with_action(Action::ResolveTribulation {
                disciple_idx: self.disciple_index,
                survived: self.tribulation.survived,
            })
            .with_transition(StateTransition::ToSectBase)
    }

    fn draw_header(&self, disciples: &[Disciple]) {
        let sw = screen_width();
        draw_panel(Rect::new(0.0, 0.0, sw, 88.0), None);
        draw_screen_title(
            "Heavenly Tribulation",
            "The patriarch watches a disciple bargain with thunder for immortality",
            24.0,
            38.0,
        );

        let name = disciples
            .get(self.disciple_index)
            .map(|d| d.name.as_str())
            .unwrap_or("Unknown Disciple");
        let mut seal_x = sw - 442.0;
        seal_x +=
            draw_resource_seal(seal_x, 56.0, "Wave", self.tribulation.current_wave, WARNING) + 8.0;
        seal_x += draw_resource_seal(
            seal_x,
            56.0,
            "Total",
            self.tribulation.config.total_waves,
            PRIMARY,
        ) + 8.0;
        draw_resource_seal(
            seal_x,
            56.0,
            "HP",
            self.tribulation.disciple_hp.max(0) as u32,
            health_color(self.health_pct()),
        );

        let dims = measure_ui_text(name, None, FONT_BODY_SIZE as u16, 1.0);
        draw_ui_text(
            name,
            (sw - dims.width) / 2.0,
            72.0,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );
    }

    fn draw_trial_scene(&self, disciple: &Disciple) {
        let sw = screen_width();
        let sh = screen_height();
        let center = vec2(sw * 0.50, sh * 0.46);
        let pulse = (get_time() as f32 * 2.4).sin() * 0.5 + 0.5;

        draw_celestial_rings(center, pulse, self.tribulation.is_finished);
        draw_lightning_walls(center, self.tribulation.current_wave, pulse);
        draw_disciple_silhouette(
            center,
            disciple,
            self.tribulation.survived,
            self.tribulation.is_finished,
        );

        let title = &self.tribulation.config.name;
        let dims = measure_ui_text(title, None, FONT_HEADER_SIZE as u16, 1.0);
        draw_ui_text(
            title,
            center.x - dims.width / 2.0,
            center.y - 160.0,
            FONT_HEADER_SIZE,
            if self.tribulation.is_finished && !self.tribulation.survived {
                FAILURE
            } else {
                TEXT_HIGHLIGHT
            },
        );
        draw_wrapped_text(
            &self.tribulation.config.description,
            center.x - 230.0,
            center.y - 126.0,
            460.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_disciple_vitals(&self, disciple: &Disciple) {
        let rect = Rect::new(24.0, 112.0, 316.0, 258.0);
        draw_panel(rect, Some("Disciple Under Heaven"));

        draw_ui_text(
            &disciple.name,
            rect.x + 22.0,
            rect.y + 70.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );
        draw_wrapped_text(
            "Flesh, spirit, and fate are weighed beneath the storm.",
            rect.x + 22.0,
            rect.y + 102.0,
            rect.w - 44.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );

        let hp_pct = self.health_pct();
        draw_ui_text(
            "Life Flame",
            rect.x + 22.0,
            rect.y + 166.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        draw_progress_bar(
            Rect::new(rect.x + 22.0, rect.y + 178.0, rect.w - 44.0, 18.0),
            hp_pct,
            health_color(hp_pct),
        );
        draw_ui_text(
            &format!(
                "{}/{}",
                self.tribulation.disciple_hp.max(0),
                self.tribulation.disciple_max_hp
            ),
            rect.x + 22.0,
            rect.y + 220.0,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );

        let wave_text = format!(
            "Heavenly strikes endured: {} of {}",
            self.tribulation.current_wave, self.tribulation.config.total_waves
        );
        draw_ui_text(
            &wave_text,
            rect.x + 22.0,
            rect.y + 246.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_heavenly_record(&self) {
        let sw = screen_width();
        let rect = Rect::new(sw - 340.0, 112.0, 316.0, 258.0);
        draw_panel(rect, Some("Patriarch's Vigil"));

        let result = if self.tribulation.is_finished {
            if self.tribulation.survived {
                ("Mandate Survives", SUCCESS)
            } else {
                ("Heaven Claims a Life", FAILURE)
            }
        } else {
            ("Clouds Still Gather", WARNING)
        };

        draw_ui_text(
            result.0,
            rect.x + 22.0,
            rect.y + 72.0,
            FONT_HEADER_SIZE,
            result.1,
        );
        draw_wrapped_text(
            if self.tribulation.is_finished {
                if self.tribulation.survived {
                    "The final thunder has broken. Record the ascension in the sect annals."
                } else {
                    "The storm has gone silent. The sect must remember the price of ambition."
                }
            } else {
                "Each command to endure invites another strike. Stop only when heaven has answered."
            },
            rect.x + 22.0,
            rect.y + 108.0,
            rect.w - 44.0,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );

        draw_ink_divider(rect.x + 22.0, rect.y + 184.0, rect.w - 44.0);
        draw_ui_text(
            &format!("Thunder Aspect: {}", self.tribulation.config.lightning_type),
            rect.x + 22.0,
            rect.y + 222.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_action_area(&self) {
        if self.tribulation.is_finished {
            draw_button(continue_rect(), "Record the Outcome", true);
        } else {
            draw_button(endure_rect(), "Endure the Next Strike", true);
        };
    }

    fn draw_log_panel(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let rect = Rect::new(24.0, sh - 190.0, sw - 48.0, 156.0);
        draw_panel(rect, Some("Storm Chronicle"));

        let mut y = rect.y + 50.0;
        if self.tribulation.log.is_empty() {
            draw_ui_text(
                "The first thunderbolt has not yet fallen.",
                rect.x + 20.0,
                y,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            return;
        }

        for log in self.tribulation.log.iter().rev().take(4) {
            draw_wrapped_text(
                log,
                rect.x + 20.0,
                y,
                rect.w - 40.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            y += 24.0;
        }
    }

    fn health_pct(&self) -> f32 {
        if self.tribulation.disciple_max_hp <= 0 {
            return 0.0;
        }
        (self.tribulation.disciple_hp.max(0) as f32 / self.tribulation.disciple_max_hp as f32)
            .clamp(0.0, 1.0)
    }
}

fn endure_rect() -> Rect {
    let sw = screen_width();
    Rect::new(sw / 2.0 - 150.0, screen_height() - 268.0, 300.0, 54.0)
}

fn continue_rect() -> Rect {
    let sw = screen_width();
    Rect::new(sw / 2.0 - 150.0, screen_height() - 268.0, 300.0, 54.0)
}

fn draw_storm_overlay() {
    let sw = screen_width();
    let sh = screen_height();
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.01, 0.006, 0.012, 0.48));
    for i in 0..7 {
        let x = sw * (0.12 + i as f32 * 0.14);
        draw_line(
            x,
            90.0,
            x + ((get_time() as f32 + i as f32).sin() * 36.0),
            sh - 210.0,
            1.0,
            Color::new(0.50, 0.62, 0.92, 0.05),
        );
    }
}

fn draw_celestial_rings(center: Vec2, pulse: f32, finished: bool) {
    let color = if finished { PRIMARY } else { WARNING };
    for i in 0..5 {
        let r = 62.0 + i as f32 * 42.0 + pulse * 10.0;
        draw_circle_lines(
            center.x,
            center.y,
            r,
            1.5,
            Color::new(color.r, color.g, color.b, 0.28 - i as f32 * 0.035),
        );
    }
    draw_circle(
        center.x,
        center.y,
        84.0 + pulse * 8.0,
        Color::new(color.r, color.g, color.b, 0.08),
    );
}

fn draw_lightning_walls(center: Vec2, wave: u32, pulse: f32) {
    let count = wave.max(1).min(9);
    for i in 0..count {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let x = center.x + side * (92.0 + i as f32 * 19.0);
        let top = center.y - 210.0;
        let mid = center.y - 84.0 + pulse * 18.0;
        let bottom = center.y + 118.0;
        let color = Color::new(0.64, 0.80, 1.0, 0.20 + i as f32 * 0.025);
        draw_line(x, top, x + side * 22.0, mid, 2.0, color);
        draw_line(x + side * 22.0, mid, x - side * 12.0, bottom, 2.0, color);
    }
}

fn draw_disciple_silhouette(center: Vec2, disciple: &Disciple, survived: bool, finished: bool) {
    let aura = if finished && !survived {
        FAILURE
    } else if finished {
        SUCCESS
    } else {
        SECONDARY
    };

    draw_circle(
        center.x,
        center.y + 10.0,
        58.0,
        Color::new(aura.r, aura.g, aura.b, 0.18),
    );
    draw_circle_lines(
        center.x,
        center.y + 10.0,
        66.0,
        2.0,
        Color::new(aura.r, aura.g, aura.b, 0.52),
    );
    draw_circle(
        center.x,
        center.y - 22.0,
        18.0,
        Color::new(0.92, 0.82, 0.62, 0.94),
    );
    draw_rectangle(
        center.x - 18.0,
        center.y - 5.0,
        36.0,
        62.0,
        Color::new(0.08, 0.05, 0.04, 0.92),
    );
    draw_line(
        center.x - 30.0,
        center.y + 12.0,
        center.x + 30.0,
        center.y + 12.0,
        4.0,
        aura,
    );

    let dims = measure_ui_text(&disciple.name, None, FONT_BODY_SIZE as u16, 1.0);
    draw_ui_text(
        &disciple.name,
        center.x - dims.width / 2.0,
        center.y + 100.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
}

fn draw_missing_disciple_notice() {
    let text = "The disciple record is missing from the sect annals.";
    let dims = measure_ui_text(text, None, FONT_BODY_SIZE as u16, 1.0);
    draw_ui_text(
        text,
        (screen_width() - dims.width) / 2.0,
        screen_height() / 2.0,
        FONT_BODY_SIZE,
        FAILURE,
    );
}

fn health_color(pct: f32) -> Color {
    if pct > 0.55 {
        SUCCESS
    } else if pct > 0.25 {
        WARNING
    } else {
        FAILURE
    }
}
