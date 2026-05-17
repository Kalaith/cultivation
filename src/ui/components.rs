use crate::ui::theme::*;
use macroquad::prelude::*;

/// Draws a styled panel with a border and optional title.
pub fn draw_panel(rect: Rect, title: Option<&str>) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(PANEL_BG).with_border(2.0, PANEL_BORDER);
    macroquad_toolkit::ui::draw_surface(rect, &surface);

    if let Some(t) = title {
        // Just the header text and underline, no background
        draw_text(
            t,
            rect.x + 15.0,
            rect.y + 28.0,
            FONT_HEADER_SIZE,
            TEXT_PRIMARY,
        );
        draw_line(
            rect.x,
            rect.y + 40.0,
            rect.x + rect.w,
            rect.y + 40.0,
            1.0,
            PANEL_BORDER,
        );
    }
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
        draw_text_ex(
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
    draw_text_ex(
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
    let dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_x = rect.x + (rect.w - dimensions.width) / 2.0;
    let text_y = rect.y + (rect.h + dimensions.height) / 2.0;

    draw_text_ex(
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
    let dimensions = measure_text(text, font, font_size as u16, 1.0);
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
