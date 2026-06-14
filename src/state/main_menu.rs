use crate::data::loader::GameData;
use crate::save::storage;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::{
    draw_button, draw_button_muted, draw_ink_divider, draw_mountain_sect_backdrop,
    draw_wrapped_text,
};
use crate::ui::theme::*;
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
        if !self.checked_save {
            self.has_save = storage::exists();
            self.checked_save = true;
        }

        let mouse = mouse_position().into();
        let (continue_btn, rebuild_btn) = menu_button_rects(self.has_save);

        if let Some(rect) = continue_btn {
            if rect.contains(mouse) && is_mouse_button_pressed(MouseButton::Left) {
                return UpdateResult::new().with_action(crate::engine::actions::Action::LoadGame);
            }
        }

        if rebuild_btn.contains(mouse) && is_mouse_button_pressed(MouseButton::Left) {
            return UpdateResult::new().with_transition(StateTransition::ToSectCreation);
        }

        if is_key_pressed(KeyCode::Enter) && self.has_save {
            return UpdateResult::new().with_action(crate::engine::actions::Action::LoadGame);
        }

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            return UpdateResult::new().with_transition(StateTransition::ToSectCreation);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        draw_mountain_sect_backdrop();
        draw_title_column();
        self.draw_command_panel();
        draw_footer(self.has_save);
    }

    fn draw_command_panel(&self) {
        let panel = menu_panel_rect();
        draw_rectangle(
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            Color::new(0.045, 0.032, 0.022, 0.86),
        );
        draw_rectangle(
            panel.x + 8.0,
            panel.y + 8.0,
            panel.w - 16.0,
            panel.h - 16.0,
            Color::new(0.22, 0.15, 0.08, 0.18),
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
            panel.x + 8.0,
            panel.y + 8.0,
            panel.w - 16.0,
            panel.h - 16.0,
            1.0,
            Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.22),
        );

        draw_ui_text(
            "PATRIARCH'S LEDGER",
            panel.x + 28.0,
            panel.y + 48.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );
        draw_ink_divider(panel.x + 28.0, panel.y + 64.0, panel.w - 56.0);

        let body = if self.has_save {
            "The old chronicle is open. Incense still burns in the ancestral hall, and your disciples wait for the next decree."
        } else {
            "The ancestral hall is cracked, the mountain gate is quiet, and a handful of disciples still believe the sect can rise again."
        };
        draw_wrapped_text(
            body,
            panel.x + 28.0,
            panel.y + 104.0,
            panel.w - 56.0,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );

        let (continue_btn, rebuild_btn) = menu_button_rects(self.has_save);
        if let Some(rect) = continue_btn {
            draw_button(rect, "Resume Mandate", true);
        }

        let rebuild_label = if self.has_save {
            "Begin Anew"
        } else {
            "Rebuild the Sect"
        };
        if self.has_save {
            draw_button_muted(rebuild_btn, rebuild_label, true);
        } else {
            draw_button(rebuild_btn, rebuild_label, true);
        }

        if self.has_save && rebuild_btn.contains(mouse_position().into()) {
            draw_ui_text(
                "A new chronicle will replace the current save.",
                panel.x + 28.0,
                rebuild_btn.y + rebuild_btn.h + 28.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }
    }
}

fn menu_panel_rect() -> Rect {
    let sw = screen_width();
    let sh = screen_height();
    let panel_w = sw.min(420.0);
    let panel_h = 360.0_f32.min(sh - 96.0);
    let panel_x = if sw >= 980.0 {
        sw - panel_w - 76.0
    } else {
        (sw - panel_w) / 2.0
    };
    Rect::new(panel_x, (sh - panel_h) / 2.0 + 18.0, panel_w, panel_h)
}

fn menu_button_rects(has_save: bool) -> (Option<Rect>, Rect) {
    let panel = menu_panel_rect();
    let button_w = panel.w - 72.0;
    let button_h = 52.0;
    let first_y = panel.y + panel.h - if has_save { 142.0 } else { 82.0 };
    let x = panel.x + 36.0;

    if has_save {
        (
            Some(Rect::new(x, first_y, button_w, button_h)),
            Rect::new(x, first_y + 64.0, button_w, button_h),
        )
    } else {
        (None, Rect::new(x, first_y, button_w, button_h))
    }
}

fn draw_title_column() {
    let sw = screen_width();
    let sh = screen_height();
    let x = if sw >= 980.0 { 72.0 } else { 48.0 };
    let y = if sw >= 980.0 { sh * 0.26 } else { 72.0 };
    let max_w = if sw >= 980.0 { sw * 0.42 } else { sw - 96.0 };

    draw_ui_text("HEAVENLY", x, y, FONT_TITLE_SIZE, PRIMARY);
    draw_ui_text("MANDATE", x, y + 44.0, FONT_TITLE_SIZE, TEXT_HIGHLIGHT);
    draw_ink_divider(x, y + 66.0, max_w.min(420.0));
    draw_wrapped_text(
        "Patriarch of a fallen mountain sect",
        x + 2.0,
        y + 104.0,
        max_w,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_wrapped_text(
        "Rebuild ruined halls. Send disciples beyond the gate. Watch them risk flesh, fate, and soul in pursuit of immortality.",
        x + 2.0,
        y + 146.0,
        max_w.min(500.0),
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );

    draw_mandate_seal(x + max_w.min(420.0) - 74.0, y + 24.0);
}

fn draw_mandate_seal(x: f32, y: f32) {
    draw_rectangle(
        x,
        y,
        58.0,
        74.0,
        Color::new(ACCENT.r, ACCENT.g, ACCENT.b, 0.72),
    );
    draw_rectangle_lines(
        x,
        y,
        58.0,
        74.0,
        2.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.46),
    );
    let dims = measure_ui_text("SECT", None, FONT_SMALL_SIZE as u16, 1.0);
    draw_ui_text(
        "SECT",
        x + (58.0 - dims.width) / 2.0,
        y + 43.0,
        FONT_SMALL_SIZE,
        TEXT_PRIMARY,
    );
}

fn draw_footer(has_save: bool) {
    let sh = screen_height();
    draw_ui_text("v0.1.0", 24.0, sh - 22.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
    if has_save {
        draw_ui_text(
            "Chronicle found",
            92.0,
            sh - 22.0,
            FONT_SMALL_SIZE,
            TEXT_HIGHLIGHT,
        );
    }
}
