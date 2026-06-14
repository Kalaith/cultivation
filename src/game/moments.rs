use super::Game;
use crate::ui::components::{draw_ink_divider, draw_wrapped_text};
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

#[derive(Clone, Copy)]
pub(in crate::game) enum MomentKind {
    Breakthrough,
    Discovery,
    Recruitment,
    SpiritBeast,
    Tragedy,
    Founding,
}

pub(in crate::game) struct MomentOverlay {
    kind: MomentKind,
    title: String,
    subtitle: String,
    detail: String,
    ticks_remaining: u32,
    total_ticks: u32,
}

impl MomentOverlay {
    fn new(kind: MomentKind, title: String, subtitle: String, detail: String) -> Self {
        Self {
            kind,
            title,
            subtitle,
            detail,
            ticks_remaining: 210,
            total_ticks: 210,
        }
    }
}

impl Game {
    pub(in crate::game) fn show_moment(
        &mut self,
        kind: MomentKind,
        title: impl Into<String>,
        subtitle: impl Into<String>,
        detail: impl Into<String>,
    ) {
        self.active_moment = Some(MomentOverlay::new(
            kind,
            title.into(),
            subtitle.into(),
            detail.into(),
        ));
    }

    pub(in crate::game) fn update_moment_timer(&mut self) {
        let Some(moment) = &mut self.active_moment else {
            return;
        };
        moment.ticks_remaining = moment.ticks_remaining.saturating_sub(1);
        if moment.ticks_remaining == 0 {
            self.active_moment = None;
        }
    }

    pub(in crate::game) fn draw_active_moment(&self) {
        let Some(moment) = &self.active_moment else {
            return;
        };

        let sw = screen_width();
        let sh = screen_height();
        let progress = 1.0 - moment.ticks_remaining as f32 / moment.total_ticks as f32;
        let fade_in = (progress / 0.18).clamp(0.0, 1.0);
        let fade_out = (moment.ticks_remaining as f32 / 36.0).clamp(0.0, 1.0);
        let alpha = fade_in.min(fade_out);
        let accent = moment_color(moment.kind);
        let panel_w = sw.min(780.0);
        let panel_h = 300.0;
        let panel_x = (sw - panel_w) / 2.0;
        let panel_y = sh * 0.16;

        draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.34 * alpha));
        draw_moment_rings(sw, sh, accent, alpha, progress);
        draw_moment_panel(panel_x, panel_y, panel_w, panel_h, accent, alpha);

        let title_y = panel_y + 78.0;
        let dims = measure_ui_text(&moment.title, None, FONT_TITLE_SIZE as u16, 1.0);
        draw_ui_text(
            &moment.title,
            panel_x + (panel_w - dims.width) / 2.0,
            title_y,
            FONT_TITLE_SIZE,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, alpha),
        );

        let subtitle_dims = measure_ui_text(&moment.subtitle, None, FONT_BODY_SIZE as u16, 1.0);
        draw_ui_text(
            &moment.subtitle,
            panel_x + (panel_w - subtitle_dims.width) / 2.0,
            title_y + 36.0,
            FONT_BODY_SIZE,
            Color::new(TEXT_HIGHLIGHT.r, TEXT_HIGHLIGHT.g, TEXT_HIGHLIGHT.b, alpha),
        );

        draw_ink_divider(panel_x + 58.0, title_y + 58.0, panel_w - 116.0);
        draw_wrapped_text(
            &moment.detail,
            panel_x + 72.0,
            title_y + 90.0,
            panel_w - 144.0,
            FONT_BODY_SIZE,
            Color::new(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b, 0.92 * alpha),
        );

        draw_moment_stamp(
            vec2(panel_x + panel_w - 92.0, panel_y + 68.0),
            moment.kind,
            accent,
            alpha,
        );
    }
}

fn draw_moment_rings(sw: f32, sh: f32, accent: Color, alpha: f32, progress: f32) {
    let center = vec2(sw * 0.5, sh * 0.34);
    for i in 0..5 {
        let radius = 82.0 + i as f32 * 46.0 + progress * 22.0;
        draw_circle_lines(
            center.x,
            center.y,
            radius,
            1.0 + i as f32 * 0.25,
            Color::new(
                accent.r,
                accent.g,
                accent.b,
                alpha * (0.16 - i as f32 * 0.018),
            ),
        );
    }
}

fn draw_moment_panel(x: f32, y: f32, w: f32, h: f32, accent: Color, alpha: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.045, 0.032, 0.020, 0.88 * alpha));
    draw_rectangle(
        x + 10.0,
        y + 10.0,
        w - 20.0,
        h - 20.0,
        Color::new(0.23, 0.16, 0.08, 0.20 * alpha),
    );
    draw_rectangle_lines(
        x,
        y,
        w,
        h,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.78 * alpha),
    );
    draw_rectangle_lines(
        x + 10.0,
        y + 10.0,
        w - 20.0,
        h - 20.0,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.34 * alpha),
    );
}

fn draw_moment_stamp(center: Vec2, kind: MomentKind, accent: Color, alpha: f32) {
    draw_circle(
        center.x,
        center.y,
        42.0,
        Color::new(accent.r, accent.g, accent.b, 0.18 * alpha),
    );
    draw_circle_lines(
        center.x,
        center.y,
        44.0,
        3.0,
        Color::new(accent.r, accent.g, accent.b, 0.72 * alpha),
    );
    let label = moment_stamp_label(kind);
    let dims = measure_ui_text(label, None, FONT_SMALL_SIZE as u16, 1.0);
    draw_ui_text(
        label,
        center.x - dims.width / 2.0,
        center.y + 5.0,
        FONT_SMALL_SIZE,
        Color::new(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b, alpha),
    );
}

fn moment_color(kind: MomentKind) -> Color {
    match kind {
        MomentKind::Breakthrough => WARNING,
        MomentKind::Discovery => SECONDARY,
        MomentKind::Recruitment => SUCCESS,
        MomentKind::SpiritBeast => PRIMARY,
        MomentKind::Tragedy => FAILURE,
        MomentKind::Founding => TEXT_HIGHLIGHT,
    }
}

fn moment_stamp_label(kind: MomentKind) -> &'static str {
    match kind {
        MomentKind::Breakthrough => "ASCEND",
        MomentKind::Discovery => "FOUND",
        MomentKind::Recruitment => "ADMIT",
        MomentKind::SpiritBeast => "BIND",
        MomentKind::Tragedy => "FATE",
        MomentKind::Founding => "SECT",
    }
}
