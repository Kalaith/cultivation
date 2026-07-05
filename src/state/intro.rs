use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

struct IntroBeat {
    title: &'static str,
    stamp: &'static str,
    text: &'static str,
}

const BEATS: &[IntroBeat] = &[
    IntroBeat {
        title: "The Long Road Home",
        stamp: "RETURN",
        text: "Snow chokes the mountain pass. After thirty years of wandering, the patriarch \
               climbs the ten thousand steps alone. No gongs greet him. No incense burns.",
    },
    IntroBeat {
        title: "Ashes of the Ancestral Hall",
        stamp: "RUIN",
        text: "The gate hangs from one hinge. The ancestral hall lies cracked open to the sky, \
               its tablets scattered like teeth. Of the hundred halls of the sect, not one \
               still stands whole.",
    },
    IntroBeat {
        title: "What the Enemy Left Behind",
        stamp: "GRIEF",
        text: "He buries what the crows left. An elder's shattered sword. A junior's single \
               sandal. The mountain remembers every name, and so will he.",
    },
    IntroBeat {
        title: "The Vow",
        stamp: "VOW",
        text: "Kneeling before the broken tablets, the patriarch presses his brow to cold \
               stone. 'By my blood and dao-heart: this sect will rise again. Higher than \
               before. Beyond the reach of any enemy under heaven.'",
    },
    IntroBeat {
        title: "The First Decree",
        stamp: "SECT",
        text: "Dawn. One ruined hall, fifty spirit stones, and an immortal ambition. The \
               rebuilding begins with a single decree: restore the Sect Hall.",
    },
];

pub struct IntroState {
    beat: usize,
    beat_started: f64,
}

impl IntroState {
    pub fn new() -> Self {
        Self {
            beat: 0,
            beat_started: get_time(),
        }
    }

    pub fn update(&mut self, sect_name: &str) -> UpdateResult {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Ink-black vignette over whatever background is beneath.
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.02, 0.02, 0.03, 0.90));

        let beat = &BEATS[self.beat.min(BEATS.len() - 1)];
        let fade = (((get_time() - self.beat_started) / 0.6).min(1.0)) as f32;

        let panel_w = 680.0_f32.min(screen_w - 80.0);
        let panel_h = 340.0;
        let panel_x = (screen_w - panel_w) / 2.0;
        let panel_y = (screen_h - panel_h) / 2.0;
        let rect = Rect::new(panel_x, panel_y, panel_w, panel_h);

        // Slow-breathing rings behind the panel, echoing the moment overlays.
        let t = get_time() as f32;
        for i in 0..3 {
            let phase = (t * 0.4 + i as f32 * 0.33) % 1.0;
            let radius = 200.0 + phase * 240.0;
            draw_circle_lines(
                screen_w / 2.0,
                screen_h / 2.0,
                radius,
                1.5,
                Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.12 * (1.0 - phase) * fade),
            );
        }

        draw_panel(rect, None);

        // Cinnabar seal stamp in the upper-left of the panel.
        let seal = Rect::new(rect.x + 24.0, rect.y + 24.0, 76.0, 76.0);
        draw_rectangle_lines(
            seal.x,
            seal.y,
            seal.w,
            seal.h,
            3.0,
            Color::new(ACCENT.r, ACCENT.g, ACCENT.b, 0.85 * fade),
        );
        let stamp_dims = measure_ui_text(beat.stamp, None, FONT_SMALL_SIZE as u16, 1.0);
        draw_ui_text(
            beat.stamp,
            seal.x + (seal.w - stamp_dims.width) / 2.0,
            seal.y + seal.h / 2.0 + 6.0,
            FONT_SMALL_SIZE,
            Color::new(ACCENT.r, ACCENT.g, ACCENT.b, fade),
        );

        draw_ui_text(
            sect_name,
            rect.x + 120.0,
            rect.y + 46.0,
            FONT_SMALL_SIZE,
            Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.8 * fade),
        );
        draw_ui_text(
            beat.title,
            rect.x + 120.0,
            rect.y + 82.0,
            FONT_HEADER_SIZE,
            Color::new(TEXT_HIGHLIGHT.r, TEXT_HIGHLIGHT.g, TEXT_HIGHLIGHT.b, fade),
        );
        draw_ink_divider(rect.x + 120.0, rect.y + 98.0, rect.w - 150.0);

        draw_wrapped_text(
            beat.text,
            rect.x + 120.0,
            rect.y + 136.0,
            rect.w - 160.0,
            FONT_BODY_SIZE,
            Color::new(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b, fade),
        );

        // Beat progress dots.
        let dots_w = BEATS.len() as f32 * 22.0;
        let mut dot_x = rect.x + (rect.w - dots_w) / 2.0 + 8.0;
        for i in 0..BEATS.len() {
            let filled = i <= self.beat;
            if filled {
                draw_circle(dot_x, rect.y + rect.h - 28.0, 5.0, PRIMARY);
            } else {
                draw_circle_lines(
                    dot_x,
                    rect.y + rect.h - 28.0,
                    5.0,
                    1.5,
                    Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.4),
                );
            }
            dot_x += 22.0;
        }

        let last_beat = self.beat + 1 >= BEATS.len();
        let cont_label = if last_beat { "Begin" } else { "Continue" };
        let cont_rect = Rect::new(rect.x + rect.w - 140.0, rect.y + rect.h - 52.0, 116.0, 34.0);
        let skip_rect = Rect::new(rect.x + rect.w - 90.0, rect.y + 12.0, 66.0, 26.0);

        let continue_clicked = draw_button(cont_rect, cont_label, false);
        let skip_clicked = if last_beat {
            false
        } else {
            draw_button(skip_rect, "Skip", false)
        };

        draw_ui_text(
            "Click or press Space to continue",
            rect.x + 24.0,
            rect.y + rect.h - 30.0,
            FONT_SMALL_SIZE,
            Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.55),
        );

        if skip_clicked || is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let mouse = mouse_position();
        let clicked_elsewhere = is_mouse_button_pressed(MouseButton::Left)
            && !cont_rect.contains(mouse.into())
            && !skip_rect.contains(mouse.into());
        let advance = continue_clicked
            || clicked_elsewhere
            || is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter);

        if advance {
            if last_beat {
                return UpdateResult::new().with_transition(StateTransition::ToSectBase);
            }
            self.beat += 1;
            self.beat_started = get_time();
        }

        UpdateResult::new()
    }

    pub fn draw(&self) {
        // Drawing handled in update for immediate mode
    }
}
