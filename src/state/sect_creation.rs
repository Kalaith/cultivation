use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::{
    draw_button, draw_button_muted, draw_ink_divider, draw_mountain_sect_backdrop,
    draw_wrapped_text,
};
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub struct SectCreationState {
    pub input_buffer: String,
    using_suggested_name: bool,
}

impl SectCreationState {
    pub fn new() -> Self {
        Self {
            input_buffer: "Fallen Peak".to_string(),
            using_suggested_name: true,
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToMainMenu);
        }

        while let Some(c) = get_char_pressed() {
            if !is_valid_sect_name_char(c) || self.input_buffer.len() >= 24 {
                continue;
            }
            if self.using_suggested_name {
                self.input_buffer.clear();
                self.using_suggested_name = false;
            }
            self.input_buffer.push(c);
        }

        if is_key_pressed(KeyCode::Backspace) {
            if self.using_suggested_name {
                self.input_buffer.clear();
                self.using_suggested_name = false;
            } else {
                self.input_buffer.pop();
            }
        }

        let (begin_rect, back_rect) = founding_button_rects();
        let mouse = mouse_position().into();

        if back_rect.contains(mouse) && is_mouse_button_pressed(MouseButton::Left) {
            return UpdateResult::new().with_transition(StateTransition::ToMainMenu);
        }

        let sect_name = self.input_buffer.trim();
        let begin_requested = is_key_pressed(KeyCode::Enter)
            || (begin_rect.contains(mouse) && is_mouse_button_pressed(MouseButton::Left));
        if begin_requested && !sect_name.is_empty() {
            return UpdateResult::new().with_action(Action::StartNewGame(sect_name.to_string()));
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData) {
        draw_mountain_sect_backdrop();
        draw_foundation_heading();
        self.draw_foundation_panel();
    }

    fn draw_foundation_panel(&self) {
        let panel = founding_panel_rect();
        draw_rectangle(
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            Color::new(0.045, 0.032, 0.022, 0.88),
        );
        draw_rectangle(
            panel.x + 10.0,
            panel.y + 10.0,
            panel.w - 20.0,
            panel.h - 20.0,
            Color::new(0.24, 0.17, 0.08, 0.18),
        );
        draw_rectangle_lines(
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            2.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.76),
        );
        draw_rectangle_lines(
            panel.x + 10.0,
            panel.y + 10.0,
            panel.w - 20.0,
            panel.h - 20.0,
            1.0,
            Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.22),
        );

        draw_ui_text(
            "RESTORE THE MOUNTAIN GATE",
            panel.x + 30.0,
            panel.y + 48.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );
        draw_ink_divider(panel.x + 30.0, panel.y + 64.0, panel.w - 60.0);
        draw_wrapped_text(
            "The name carved above the gate is the first oath your disciples will carry into danger.",
            panel.x + 30.0,
            panel.y + 104.0,
            panel.w - 60.0,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );

        self.draw_name_input(panel);
        draw_foundation_slips(panel);

        let (begin_rect, back_rect) = founding_button_rects();
        draw_button(
            begin_rect,
            "Raise the Banner",
            !self.input_buffer.trim().is_empty(),
        );
        draw_button_muted(back_rect, "Return", true);
    }

    fn draw_name_input(&self, panel: Rect) {
        let input = Rect::new(panel.x + 38.0, panel.y + 196.0, panel.w - 76.0, 58.0);
        draw_rectangle(
            input.x,
            input.y,
            input.w,
            input.h,
            Color::new(0.03, 0.024, 0.018, 0.82),
        );
        draw_rectangle_lines(
            input.x,
            input.y,
            input.w,
            input.h,
            1.5,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.58),
        );

        draw_ui_text(
            "Sect Name",
            input.x + 14.0,
            input.y - 12.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );

        let display = if self.input_buffer.is_empty() {
            "Name the restored sect"
        } else {
            &self.input_buffer
        };
        let color = if self.input_buffer.is_empty() {
            TEXT_SECONDARY
        } else {
            TEXT_HIGHLIGHT
        };
        let font_size = input_text_size(display, input.w - 28.0);
        draw_ui_text(display, input.x + 14.0, input.y + 38.0, font_size, color);

        if !self.input_buffer.is_empty() && (get_time() * 2.0) as i32 % 2 == 0 {
            let dims = measure_ui_text(display, None, font_size as u16, 1.0);
            let cursor_x = (input.x + 16.0 + dims.width).min(input.x + input.w - 14.0);
            draw_line(
                cursor_x,
                input.y + 14.0,
                cursor_x,
                input.y + input.h - 14.0,
                2.0,
                TEXT_HIGHLIGHT,
            );
        }
    }
}

fn founding_panel_rect() -> Rect {
    let sw = screen_width();
    let sh = screen_height();
    let panel_w = sw.min(560.0);
    let panel_h = 440.0_f32.min(sh - 92.0);
    Rect::new(
        (sw - panel_w) / 2.0,
        (sh - panel_h) / 2.0 + 24.0,
        panel_w,
        panel_h,
    )
}

fn founding_button_rects() -> (Rect, Rect) {
    let panel = founding_panel_rect();
    let button_w = (panel.w - 92.0) * 0.62;
    let back_w = (panel.w - 92.0) - button_w;
    let y = panel.y + panel.h - 80.0;
    (
        Rect::new(panel.x + 38.0, y, button_w, 52.0),
        Rect::new(panel.x + 54.0 + button_w, y, back_w, 52.0),
    )
}

fn draw_foundation_heading() {
    let sw = screen_width();
    let y = 58.0;
    let title = "NAME THE RESTORED SECT";
    let dims = measure_ui_text(title, None, FONT_TITLE_SIZE as u16, 1.0);
    draw_ui_text(title, (sw - dims.width) / 2.0, y, FONT_TITLE_SIZE, PRIMARY);

    let subtitle = "A patriarch's first decree becomes the mountain's future.";
    let sub_dims = measure_ui_text(subtitle, None, FONT_BODY_SIZE as u16, 1.0);
    draw_ui_text(
        subtitle,
        (sw - sub_dims.width) / 2.0,
        y + 36.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
}

fn draw_foundation_slips(panel: Rect) {
    let labels = ["Hall", "Disciples", "Immortality"];
    let start_x = panel.x + 44.0;
    let y = panel.y + 276.0;

    for (i, label) in labels.iter().enumerate() {
        let x = start_x + i as f32 * 128.0;
        draw_rectangle(
            x,
            y,
            98.0,
            28.0,
            Color::new(ACCENT.r, ACCENT.g, ACCENT.b, 0.26),
        );
        draw_rectangle_lines(
            x,
            y,
            98.0,
            28.0,
            1.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.34),
        );
        let dims = measure_ui_text(label, None, FONT_SMALL_SIZE as u16, 1.0);
        draw_ui_text(
            label,
            x + (98.0 - dims.width) / 2.0,
            y + 20.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }
}

fn input_text_size(text: &str, max_width: f32) -> f32 {
    let mut size = FONT_BODY_SIZE;
    while size > FONT_SMALL_SIZE {
        let dims = measure_ui_text(text, None, size as u16, 1.0);
        if dims.width <= max_width {
            break;
        }
        size -= 1.0;
    }
    size
}

fn is_valid_sect_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '\''
}
