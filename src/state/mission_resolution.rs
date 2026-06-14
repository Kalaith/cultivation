use crate::data::loader::GameData;
use crate::data::missions::{MissionOutcome, MissionRewards};
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

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
            let screen_w = screen_width();
            let screen_h = screen_height();
            let w = 720.0;
            let h = 560.0;
            let x = (screen_w - w) / 2.0;
            let y = (screen_h - h) / 2.0;
            let rect = Rect::new(x, y, w, h);

            draw_moment_backdrop(screen_w, screen_h, outcome.success);
            draw_panel(rect, Some("Sect Annal: Mission Return"));

            let accent = if outcome.success { SUCCESS } else { FAILURE };
            draw_status_stamp(
                vec2(rect.x + rect.w - 110.0, rect.y + 108.0),
                outcome.success,
                accent,
            );

            let mut draw_y = rect.y + 66.0;
            draw_centered_wrapped_title(
                &outcome.mission_name,
                rect.x + 34.0,
                draw_y,
                rect.w - 190.0,
            );
            draw_y += 82.0;

            draw_ui_text(
                if outcome.success {
                    "The disciples return through the mountain gate."
                } else {
                    "The mountain gate opens to wounded silence."
                },
                rect.x + 36.0,
                draw_y,
                FONT_BODY_SIZE,
                TEXT_HIGHLIGHT,
            );
            draw_y += 34.0;

            draw_ink_divider(rect.x + 36.0, draw_y, rect.w - 72.0);
            draw_y += 28.0;

            draw_ui_text(
                "Journey Record",
                rect.x + 36.0,
                draw_y,
                FONT_BODY_SIZE,
                PRIMARY,
            );
            draw_y += 26.0;
            draw_y = draw_journey_logs(rect, draw_y, &outcome.logs);

            let rewards_y = rect.y + rect.h - 152.0;
            draw_reward_slips(
                Rect::new(rect.x + 36.0, rewards_y, rect.w - 72.0, 72.0),
                &outcome.rewards,
            );

            if draw_button(
                Rect::new(
                    rect.x + (rect.w - 220.0) / 2.0,
                    rect.y + rect.h - 64.0,
                    220.0,
                    44.0,
                ),
                if outcome.success {
                    "Record Spoils"
                } else {
                    "Close the Annal"
                },
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
        // Handled in update.
    }
}

fn draw_moment_backdrop(screen_w: f32, screen_h: f32, success: bool) {
    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.0, 0.0, 0.0, 0.76),
    );
    let color = if success { PRIMARY } else { ACCENT };
    for i in 0..6 {
        let radius = 90.0 + i as f32 * 58.0;
        draw_circle_lines(
            screen_w * 0.5,
            screen_h * 0.5,
            radius,
            1.0,
            Color::new(color.r, color.g, color.b, 0.055),
        );
    }
    draw_line(
        screen_w * 0.16,
        screen_h * 0.18,
        screen_w * 0.84,
        screen_h * 0.82,
        2.0,
        Color::new(color.r, color.g, color.b, 0.10),
    );
}

fn draw_status_stamp(center: Vec2, success: bool, color: Color) {
    draw_circle(
        center.x,
        center.y,
        56.0,
        Color::new(color.r, color.g, color.b, 0.18),
    );
    draw_circle_lines(
        center.x,
        center.y,
        58.0,
        4.0,
        Color::new(color.r, color.g, color.b, 0.78),
    );
    draw_circle_lines(
        center.x,
        center.y,
        42.0,
        2.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.44),
    );
    let text = if success { "RETURNED" } else { "SETBACK" };
    let dims = measure_ui_text(text, None, FONT_BODY_SIZE as u16, 1.0);
    draw_ui_text(
        text,
        center.x - dims.width / 2.0,
        center.y + 7.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
}

fn draw_centered_wrapped_title(text: &str, x: f32, y: f32, max_width: f32) {
    let end_y = draw_wrapped_text(text, x, y, max_width, FONT_HEADER_SIZE, PRIMARY);
    if end_y <= y + FONT_HEADER_SIZE + 8.0 {
        return;
    }
}

fn draw_journey_logs(rect: Rect, mut y: f32, logs: &[String]) -> f32 {
    for log in logs.iter().rev().take(5).rev() {
        if y > rect.y + rect.h - 180.0 {
            break;
        }
        draw_rectangle(
            rect.x + 36.0,
            y - 17.0,
            rect.w - 72.0,
            28.0,
            Color::new(0.05, 0.035, 0.02, 0.30),
        );
        y = draw_wrapped_text(
            log,
            rect.x + 48.0,
            y + 5.0,
            rect.w - 96.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        y += 6.0;
    }
    y
}

fn draw_reward_slips(rect: Rect, rewards: &MissionRewards) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.04, 0.03, 0.02, 0.38),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.42),
    );

    let entries = reward_entries(rewards);
    if entries.is_empty() {
        draw_ui_text(
            "No spoils were recovered.",
            rect.x + 18.0,
            rect.y + 42.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        return;
    }

    let mut x = rect.x + 16.0;
    for (label, value, color) in entries {
        let w = 126.0;
        draw_rectangle(
            x,
            rect.y + 14.0,
            w,
            44.0,
            Color::new(0.12, 0.08, 0.04, 0.42),
        );
        draw_rectangle_lines(
            x,
            rect.y + 14.0,
            w,
            44.0,
            1.0,
            Color::new(color.r, color.g, color.b, 0.54),
        );
        draw_ui_text(
            label,
            x + 10.0,
            rect.y + 34.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        draw_ui_text(&value, x + 10.0, rect.y + 54.0, FONT_BODY_SIZE, color);
        x += w + 10.0;
        if x + w > rect.x + rect.w - 12.0 {
            break;
        }
    }
}

fn reward_entries(rewards: &MissionRewards) -> Vec<(&'static str, String, Color)> {
    let mut entries = Vec::new();
    if rewards.spirit_stones > 0 {
        entries.push(("Spirit Stones", rewards.spirit_stones.to_string(), PRIMARY));
    }
    if rewards.disciple_exp > 0 {
        entries.push((
            "Cultivation",
            format!("{} exp", rewards.disciple_exp),
            SECONDARY,
        ));
    }
    if rewards.herbs > 0 {
        entries.push(("Herbs", rewards.herbs.to_string(), SUCCESS));
    }
    if rewards.influence > 0 {
        entries.push(("Prestige", rewards.influence.to_string(), WARNING));
    }
    if rewards.relics > 0 {
        entries.push(("Relics", rewards.relics.to_string(), ACCENT));
    }
    if !rewards.items.is_empty() {
        let total: u32 = rewards.items.iter().map(|(_, count)| *count).sum();
        entries.push(("Materials", total.to_string(), TEXT_HIGHLIGHT));
    }
    entries
}
