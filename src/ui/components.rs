use macroquad::prelude::*;
use crate::ui::theme::*;

/// Draws a styled panel with a border and optional title.
pub fn draw_panel(rect: Rect, title: Option<&str>) {
    // Semi-transparent panel background for visibility
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, PANEL_BG);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, PANEL_BORDER);

    if let Some(t) = title {
        // Just the header text and underline, no background
        draw_text(t, rect.x + 15.0, rect.y + 28.0, FONT_HEADER_SIZE, TEXT_PRIMARY);
        draw_line(rect.x, rect.y + 40.0, rect.x + rect.w, rect.y + 40.0, 1.0, PANEL_BORDER);
    }
}

/// Draw a brush stroke effect - simulates ink brush painting
pub fn draw_brush_stroke(x: f32, y: f32, width: f32, height: f32, color: Color, intensity: f32) {
    // Use deterministic "randomness" based on position for consistent look
    let seed = (x * 100.0 + y * 50.0) as i32;

    // Main horizontal brush stroke - thick center fading at edges
    let stroke_height = height * 0.7;
    let stroke_y = y + (height - stroke_height) / 2.0;

    // Draw multiple overlapping strokes for brush texture
    for i in 0..5 {
        let offset_y = ((seed + i * 17) % 7) as f32 - 3.0;
        let offset_x = ((seed + i * 23) % 5) as f32 - 2.0;
        let thickness = stroke_height * (0.6 + (i as f32 * 0.1));
        let alpha = intensity * (0.3 + (i as f32 * 0.15));

        let stroke_color = Color::new(color.r, color.g, color.b, alpha.min(color.a));

        // Tapered stroke - thinner at ends
        let taper_left = width * 0.05;
        let taper_right = width * 0.05;

        // Main body
        draw_rectangle(
            x + taper_left + offset_x,
            stroke_y + offset_y,
            width - taper_left - taper_right,
            thickness,
            stroke_color,
        );

        // Left taper (triangle-ish)
        for t in 0..3 {
            let t_ratio = t as f32 / 3.0;
            let t_width = taper_left * (1.0 - t_ratio);
            let t_height = thickness * (0.3 + t_ratio * 0.7);
            draw_rectangle(
                x + t_ratio * taper_left + offset_x,
                stroke_y + (thickness - t_height) / 2.0 + offset_y,
                t_width,
                t_height,
                stroke_color,
            );
        }

        // Right taper
        for t in 0..3 {
            let t_ratio = t as f32 / 3.0;
            let t_width = taper_right * (1.0 - t_ratio);
            let t_height = thickness * (0.3 + t_ratio * 0.7);
            draw_rectangle(
                x + width - taper_right + t_ratio * taper_right + offset_x,
                stroke_y + (thickness - t_height) / 2.0 + offset_y,
                t_width,
                t_height,
                stroke_color,
            );
        }
    }
}

/// Draws bold text with outline for brush stroke effect
fn draw_bold_text(text: &str, x: f32, y: f32, font_size: f32, color: Color, font: Option<&Font>) {
    // Draw outline/shadow for boldness
    let outline_color = Color::new(0.0, 0.0, 0.0, 0.8);
    let offsets = [
        (-1.5, -1.5), (1.5, -1.5), (-1.5, 1.5), (1.5, 1.5),
        (-1.5, 0.0), (1.5, 0.0), (0.0, -1.5), (0.0, 1.5),
    ];

    for (ox, oy) in offsets {
        draw_text_ex(text, x + ox, y + oy, TextParams {
            font,
            font_size: font_size as u16,
            color: outline_color,
            ..Default::default()
        });
    }

    // Draw main text
    draw_text_ex(text, x, y, TextParams {
        font,
        font_size: font_size as u16,
        color,
        ..Default::default()
    });
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
        draw_rectangle(rect.x + 2.0, rect.y + 2.0, rect.w - 4.0, rect.h - 4.0, accent);
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
    // Background
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, PANEL_BORDER);

    // Fill
    let fill_w = (rect.w * progress).clamp(0.0, rect.w);
    draw_rectangle(rect.x, rect.y, fill_w, rect.h, color);
}

/// Draws a floating tooltip at the given position.
pub fn draw_tooltip(pos: Vec2, text: &str) {
     let font_size = FONT_SMALL_SIZE;
     let dimensions = measure_text(text, None, font_size as u16, 1.0);
     let pad = 8.0;
     let w = dimensions.width + pad * 2.0;
     let h = dimensions.height + pad * 2.0;

     // Ensure tooltip stays on screen
     let mut x = pos.x + 15.0;
     let mut y = pos.y + 15.0;
     if x + w > screen_width() { x -= w + 30.0; }
     if y + h > screen_height() { y -= h + 30.0; }

     draw_rectangle(x, y, w, h, Color::new(0.05, 0.05, 0.08, 0.92));
     draw_rectangle_lines(x, y, w, h, 1.0, PRIMARY);
     draw_text(text, x + pad, y + h - pad, font_size, TEXT_PRIMARY);
}

/// Draws a multi-line tooltip box at the given position.
pub fn draw_tooltip_box(x: f32, y: f32, lines: &[String]) {
    let font_size = FONT_SMALL_SIZE;
    let pad = 8.0;
    let line_height = font_size + 4.0;

    // Calculate dimensions
    let mut max_width: f32 = 0.0;
    for line in lines {
        let dims = measure_text(line, None, font_size as u16, 1.0);
        max_width = max_width.max(dims.width);
    }

    let w = max_width + pad * 2.0;
    let h = (lines.len() as f32) * line_height + pad * 2.0;

    // Ensure tooltip stays on screen
    let mut tx = x;
    let mut ty = y;
    if tx + w > screen_width() { tx = screen_width() - w - 5.0; }
    if ty + h > screen_height() { ty = screen_height() - h - 5.0; }
    if tx < 0.0 { tx = 5.0; }
    if ty < 0.0 { ty = 5.0; }

    // Draw background
    draw_rectangle(tx, ty, w, h, Color::new(0.05, 0.05, 0.08, 0.92));
    draw_rectangle_lines(tx, ty, w, h, 1.0, PRIMARY);

    // Draw lines
    let mut ly = ty + pad + font_size;
    for line in lines {
        draw_text(line, tx + pad, ly, font_size, TEXT_PRIMARY);
        ly += line_height;
    }
}
