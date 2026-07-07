use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, draw_ui_text_ex, measure_ui_text};

fn resolved_font(font: Option<&Font>) -> Option<&Font> {
    font.or(macroquad_toolkit::ui::default_ui_font())
}

/// Draws a styled panel with a border and optional title.
pub fn draw_panel(rect: Rect, title: Option<&str>) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(PANEL_BG).with_border(2.0, PANEL_BORDER);
    macroquad_toolkit::ui::draw_surface(rect, &surface);
    draw_rectangle(
        rect.x + 3.0,
        rect.y + 3.0,
        rect.w - 6.0,
        rect.h - 6.0,
        Color::new(0.20, 0.15, 0.09, 0.13),
    );
    draw_line(
        rect.x + 10.0,
        rect.y + 8.0,
        rect.x + rect.w - 10.0,
        rect.y + 8.0,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.28),
    );

    if let Some(t) = title {
        macroquad_toolkit::ui::draw_ui_text(
            t,
            rect.x + 15.0,
            rect.y + 28.0,
            FONT_HEADER_SIZE,
            TEXT_PRIMARY,
        );
        draw_line(
            rect.x + 12.0,
            rect.y + 40.0,
            rect.x + rect.w - 12.0,
            rect.y + 40.0,
            1.0,
            Color::new(PANEL_BORDER.r, PANEL_BORDER.g, PANEL_BORDER.b, 0.65),
        );
    }
}

pub fn draw_screen_title(title: &str, subtitle: &str, x: f32, y: f32) {
    draw_ui_text(title, x, y, FONT_TITLE_SIZE, PRIMARY);
    if !subtitle.is_empty() {
        draw_ui_text(subtitle, x + 2.0, y + 24.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
    }
}

pub fn draw_mountain_sect_backdrop() {
    let sw = screen_width();
    let sh = screen_height();

    for i in 0..10 {
        let t = i as f32 / 10.0;
        draw_rectangle(
            0.0,
            sh * t,
            sw,
            sh / 10.0 + 1.0,
            Color::new(0.16 - t * 0.07, 0.22 - t * 0.09, 0.24 - t * 0.10, 1.0),
        );
    }

    draw_circle(
        sw * 0.72,
        sh * 0.20,
        78.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.12),
    );
    draw_circle_lines(
        sw * 0.72,
        sh * 0.20,
        98.0,
        2.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.10),
    );

    draw_backdrop_mountains(
        sh * 0.66,
        Color::new(0.31, 0.42, 0.43, 0.42),
        Color::new(0.14, 0.20, 0.21, 0.36),
        0.03,
    );
    draw_backdrop_mountains(
        sh * 0.78,
        Color::new(0.14, 0.18, 0.18, 0.74),
        Color::new(0.07, 0.09, 0.09, 0.70),
        -0.02,
    );
    draw_backdrop_sect(sw * 0.54, sh * 0.58, sw.min(sh) * 0.22);
    draw_backdrop_waterfall(sw * 0.58, sh * 0.34, sh * 0.54);
    draw_backdrop_clouds(sh * 0.35, 0.22);
    draw_backdrop_clouds(sh * 0.76, 0.28);

    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.02, 0.015, 0.01, 0.30));
    draw_rectangle(0.0, 0.0, sw * 0.40, sh, Color::new(0.02, 0.015, 0.01, 0.22));
    draw_rectangle(
        0.0,
        sh * 0.82,
        sw,
        sh * 0.18,
        Color::new(0.02, 0.015, 0.01, 0.28),
    );
}

fn draw_backdrop_mountains(base_y: f32, ridge: Color, shade: Color, offset: f32) {
    let sw = screen_width();
    let sh = screen_height();
    let peaks: [(f32, f32); 8] = [
        (-0.08 + offset, 0.28),
        (0.06 + offset, 0.12),
        (0.20 + offset, 0.32),
        (0.34 + offset, 0.08),
        (0.49 + offset, 0.30),
        (0.66 + offset, 0.10),
        (0.84 + offset, 0.36),
        (1.08 + offset, 0.16),
    ];

    for window in peaks.windows(2) {
        let (left, lh) = window[0];
        let (right, rh) = window[1];
        let peak_x = sw * ((left + right) * 0.5);
        let peak_y = sh * (0.13 + lh.min(rh) * 0.36);
        draw_triangle(
            vec2(sw * left, base_y),
            vec2(peak_x, peak_y),
            vec2(sw * right, base_y),
            ridge,
        );
        draw_triangle(
            vec2(peak_x, peak_y),
            vec2(sw * right, base_y),
            vec2(peak_x + sw * 0.04, base_y),
            shade,
        );
    }
}

fn draw_backdrop_sect(center_x: f32, base_y: f32, scale: f32) {
    let roof = Color::new(0.10, 0.07, 0.05, 0.86);
    let wall = Color::new(0.40, 0.30, 0.20, 0.38);
    let trim = Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.32);
    let tier_w = scale;
    let tier_h = scale * 0.26;

    for i in 0..3 {
        let y = base_y - i as f32 * tier_h * 0.90;
        let w = tier_w * (1.0 - i as f32 * 0.20);
        let x = center_x - w * 0.5;
        draw_rectangle(
            x + w * 0.12,
            y - tier_h * 0.55,
            w * 0.76,
            tier_h * 0.55,
            wall,
        );
        draw_triangle(
            vec2(x, y - tier_h * 0.42),
            vec2(center_x, y - tier_h),
            vec2(x + w, y - tier_h * 0.42),
            roof,
        );
        draw_line(
            x + 7.0,
            y - tier_h * 0.43,
            x + w - 7.0,
            y - tier_h * 0.43,
            2.0,
            trim,
        );
    }

    draw_rectangle(
        center_x - tier_w * 0.38,
        base_y,
        tier_w * 0.76,
        tier_h * 0.15,
        Color::new(0.05, 0.04, 0.03, 0.70),
    );
}

fn draw_backdrop_waterfall(x: f32, y: f32, height: f32) {
    for i in 0..5 {
        let offset = (i as f32 - 2.0) * 7.0;
        draw_line(
            x + offset,
            y,
            x - 36.0 + offset * 0.6,
            y + height,
            8.0 - i as f32,
            Color::new(0.64, 0.90, 0.96, 0.040 + i as f32 * 0.010),
        );
    }
}

fn draw_backdrop_clouds(y: f32, alpha: f32) {
    let sw = screen_width();
    let clouds = [
        (0.08, 38.0),
        (0.18, 52.0),
        (0.32, 35.0),
        (0.64, 46.0),
        (0.78, 58.0),
        (0.92, 42.0),
    ];

    for (x_factor, radius) in clouds {
        let x = sw * x_factor;
        draw_circle(
            x,
            y + (x_factor * 31.0).sin() * 15.0,
            radius,
            Color::new(0.86, 0.84, 0.72, alpha * 0.26),
        );
        draw_circle(
            x + radius * 0.55,
            y + 18.0,
            radius * 0.72,
            Color::new(0.86, 0.84, 0.72, alpha * 0.18),
        );
    }
}

pub fn draw_ink_divider(x: f32, y: f32, width: f32) {
    draw_line(
        x,
        y,
        x + width,
        y,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.36),
    );
    draw_line(
        x + width * 0.12,
        y + 3.0,
        x + width * 0.88,
        y + 3.0,
        1.0,
        Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.18),
    );
}

pub fn draw_resource_seal(x: f32, y: f32, label: &str, value: u32, color: Color) -> f32 {
    let text = format!("{} {}", label, value);
    let dims = measure_ui_text(&text, None, FONT_SMALL_SIZE as u16, 1.0);
    let width = dims.width + 24.0;
    draw_rectangle(x, y - 20.0, width, 28.0, Color::new(0.05, 0.04, 0.03, 0.48));
    draw_rectangle_lines(
        x,
        y - 20.0,
        width,
        28.0,
        1.0,
        Color::new(color.r, color.g, color.b, 0.55),
    );
    draw_circle(x + 9.0, y - 6.0, 3.0, color);
    draw_ui_text(&text, x + 18.0, y, FONT_SMALL_SIZE, TEXT_PRIMARY);
    width
}

pub fn draw_wrapped_text(
    text: &str,
    x: f32,
    mut y: f32,
    max_width: f32,
    font_size: f32,
    color: Color,
) -> f32 {
    let mut line = String::new();
    let line_height = font_size + 5.0;

    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", line, word)
        };
        let dims = measure_ui_text(&candidate, None, font_size as u16, 1.0);
        if dims.width > max_width && !line.is_empty() {
            draw_ui_text(&line, x, y, font_size, color);
            y += line_height;
            line = word.to_string();
        } else {
            line = candidate;
        }
    }

    if !line.is_empty() {
        draw_ui_text(&line, x, y, font_size, color);
        y += line_height;
    }

    y
}

pub fn draw_cultivation_circle(
    center: Vec2,
    radius: f32,
    active_nodes: usize,
    total_nodes: usize,
    progress: f32,
    ready: bool,
) {
    let total = total_nodes.max(1);
    let active = active_nodes.min(total);
    let ring_color = if ready { WARNING } else { SECONDARY };

    draw_circle_lines(
        center.x,
        center.y,
        radius,
        2.0,
        Color::new(ring_color.r, ring_color.g, ring_color.b, 0.35),
    );
    draw_circle(
        center.x,
        center.y,
        radius * 0.58,
        Color::new(0.03, 0.025, 0.02, 0.42),
    );

    if ready {
        draw_circle_lines(
            center.x,
            center.y,
            radius + 8.0,
            3.0,
            Color::new(WARNING.r, WARNING.g, WARNING.b, 0.45),
        );
    }

    for i in 0..total {
        let angle =
            -std::f32::consts::FRAC_PI_2 + (i as f32 / total as f32) * std::f32::consts::PI * 2.0;
        let pos = center + vec2(angle.cos() * radius, angle.sin() * radius);
        let filled = i < active;
        let node_color = if filled {
            Color::new(ring_color.r, ring_color.g, ring_color.b, 0.92)
        } else {
            Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.30)
        };
        draw_circle(pos.x, pos.y, if filled { 6.0 } else { 4.5 }, node_color);
        draw_circle_lines(
            pos.x,
            pos.y,
            8.0,
            1.0,
            Color::new(
                PRIMARY.r,
                PRIMARY.g,
                PRIMARY.b,
                if filled { 0.40 } else { 0.14 },
            ),
        );
    }

    let pct = (progress.clamp(0.0, 1.0) * 100.0).round() as u32;
    let pct_text = format!("{}%", pct);
    let dims = measure_ui_text(&pct_text, None, FONT_HEADER_SIZE as u16, 1.0);
    draw_ui_text(
        &pct_text,
        center.x - dims.width / 2.0,
        center.y + 8.0,
        FONT_HEADER_SIZE,
        if ready { WARNING } else { TEXT_HIGHLIGHT },
    );
}

pub fn draw_stat_seal(rect: Rect, label: &str, value: u32, color: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.05, 0.04, 0.03, 0.42),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(color.r, color.g, color.b, 0.42),
    );
    draw_ui_text(
        label,
        rect.x + 10.0,
        rect.y + 20.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_ui_text(
        &value.to_string(),
        rect.x + 10.0,
        rect.y + rect.h - 10.0,
        FONT_HEADER_SIZE,
        color,
    );
}

/// Draw a brush stroke effect - simulates ink brush painting
pub fn draw_brush_stroke(x: f32, y: f32, width: f32, height: f32, color: Color, intensity: f32) {
    macroquad_toolkit::ui::draw_brush_stroke_surface(
        Rect::new(x, y, width, height),
        color,
        intensity,
    );
}

/// Draws bold text with outline for brush stroke effect
fn draw_bold_text(text: &str, x: f32, y: f32, font_size: f32, color: Color, font: Option<&Font>) {
    let font = resolved_font(font);
    // Draw outline/shadow for boldness
    let outline_color = Color::new(0.0, 0.0, 0.0, 0.8);
    let offsets = [
        (-1.5, -1.5),
        (1.5, -1.5),
        (-1.5, 1.5),
        (1.5, 1.5),
        (-1.5, 0.0),
        (1.5, 0.0),
        (0.0, -1.5),
        (0.0, 1.5),
    ];

    for (ox, oy) in offsets {
        macroquad_toolkit::ui::draw_ui_text_ex(
            text,
            x + ox,
            y + oy,
            TextParams {
                font,
                font_size: font_size as u16,
                color: outline_color,
                ..Default::default()
            },
        );
    }

    // Draw main text
    macroquad_toolkit::ui::draw_ui_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size: font_size as u16,
            color,
            ..Default::default()
        },
    );
}

/// Draws a button with brush stroke styling and returns true if clicked.
pub fn draw_button(rect: Rect, text: &str, active: bool) -> bool {
    draw_button_with_font(rect, text, active, None)
}

/// Draws a muted button for de-emphasized UI areas.
pub fn draw_button_muted(rect: Rect, text: &str, active: bool) -> bool {
    let mouse_pos = mouse_position().into();
    let hovered = rect.contains(mouse_pos);
    let is_clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    let stroke_color = if active {
        Color::new(0.12, 0.10, 0.08, 0.55)
    } else {
        Color::new(0.08, 0.06, 0.05, 0.45)
    };

    let text_color = if active {
        Color::new(0.85, 0.82, 0.72, 0.85)
    } else {
        Color::new(0.75, 0.72, 0.68, 0.75)
    };

    let intensity = if hovered { 0.8 } else { 0.6 };
    draw_brush_stroke(rect.x, rect.y, rect.w, rect.h, stroke_color, intensity);

    let font_size = rect.h * 0.42;
    let dimensions = macroquad_toolkit::ui::measure_ui_text(text, None, font_size as u16, 1.0);
    let text_x = rect.x + (rect.w - dimensions.width) / 2.0;
    let text_y = rect.y + (rect.h + dimensions.height) / 2.0;

    macroquad_toolkit::ui::draw_ui_text_ex(
        text,
        text_x,
        text_y,
        TextParams {
            font_size: font_size as u16,
            color: text_color,
            ..Default::default()
        },
    );

    is_clicked
}

/// Draws a button with brush stroke styling using optional custom font.
pub fn draw_button_with_font(rect: Rect, text: &str, active: bool, font: Option<&Font>) -> bool {
    let font = resolved_font(font);
    let mouse_pos = mouse_position().into();
    let hovered = rect.contains(mouse_pos);
    let is_clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    // Brush stroke colors
    let stroke_color = if active {
        Color::new(0.15, 0.12, 0.08, 0.85) // Dark brown/black ink
    } else {
        Color::new(0.08, 0.06, 0.04, 0.7) // Lighter ink
    };

    let text_color = if active {
        Color::new(0.95, 0.9, 0.75, 1.0) // Warm cream/gold
    } else {
        Color::new(0.85, 0.82, 0.75, 1.0) // Lighter cream
    };

    // Hover effect - intensify the brush stroke
    let intensity = if hovered {
        if is_mouse_button_down(MouseButton::Left) {
            1.2
        } else {
            1.0
        }
    } else {
        0.8
    };

    // Draw brush stroke background
    draw_brush_stroke(rect.x, rect.y, rect.w, rect.h, stroke_color, intensity);

    // Add subtle gold accent on hover
    if hovered {
        let accent = Color::new(0.9, 0.8, 0.3, 0.15);
        draw_rectangle(
            rect.x + 2.0,
            rect.y + 2.0,
            rect.w - 4.0,
            rect.h - 4.0,
            accent,
        );
    }

    // Calculate text size - larger, bolder
    let font_size = rect.h * 0.45;
    let dimensions = macroquad_toolkit::ui::measure_ui_text(text, font, font_size as u16, 1.0);
    let text_x = rect.x + (rect.w - dimensions.width) / 2.0;
    let text_y = rect.y + (rect.h + dimensions.height) / 2.0;

    // Draw bold brush-style text
    draw_bold_text(text, text_x, text_y, font_size, text_color, font);

    is_clicked
}

/// Draws a progress bar.
pub fn draw_progress_bar(rect: Rect, progress: f32, color: Color) {
    macroquad_toolkit::ui::progress_bar(rect.x, rect.y, rect.w, rect.h, progress, 1.0, color);
}

/// Draws a floating tooltip at the given position.
pub fn draw_tooltip(pos: Vec2, text: &str) {
    let style = macroquad_toolkit::ui::TooltipStyle {
        background: Color::new(0.05, 0.05, 0.08, 0.92),
        border: PRIMARY,
        text: TEXT_PRIMARY,
        font_size: FONT_SMALL_SIZE,
        ..Default::default()
    };
    macroquad_toolkit::ui::draw_tooltip_styled(text, pos, &style, None);
}

/// Draws a multi-line tooltip box at the given position.
pub fn draw_tooltip_box(x: f32, y: f32, lines: &[String]) {
    let style = macroquad_toolkit::ui::TooltipStyle {
        background: Color::new(0.05, 0.05, 0.08, 0.92),
        border: PRIMARY,
        text: TEXT_PRIMARY,
        font_size: FONT_SMALL_SIZE,
        line_gap: 4.0,
        ..Default::default()
    };
    macroquad_toolkit::ui::draw_tooltip_styled(&lines.join("\n"), vec2(x, y), &style, None);
}

/// Draw a short small-font label backed by a dark ink scrim so colored or light
/// text stays legible over busy or pale backgrounds (e.g. the beige world map).
/// `baseline` is the text's lower-left origin, matching [`draw_ui_text`].
pub fn draw_scrim_label(baseline: Vec2, text: &str, color: Color) {
    let dims = measure_ui_text(text, None, FONT_SMALL_SIZE as u16, 1.0);
    let pad_x = 6.0;
    let pad_y = 3.0;
    draw_rectangle(
        baseline.x - pad_x,
        baseline.y - dims.height - pad_y,
        dims.width + pad_x * 2.0,
        dims.height + pad_y * 2.0,
        Color::new(0.10, 0.07, 0.05, 0.84),
    );
    draw_ui_text(text, baseline.x, baseline.y, FONT_SMALL_SIZE, color);
}
