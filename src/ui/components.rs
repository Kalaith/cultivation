use macroquad::prelude::*;
use crate::ui::theme::*;

/// Draws a styled panel with a border and optional title.
pub fn draw_panel(rect: Rect, title: Option<&str>) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, PANEL_BG);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, PANEL_BORDER);

    if let Some(t) = title {
        // Draw a header bar for the title
        draw_rectangle(rect.x, rect.y, rect.w, 40.0, Color::new(0.15, 0.17, 0.2, 1.0));
        draw_rectangle_lines(rect.x, rect.y, rect.w, 40.0, 1.0, PANEL_BORDER);
        draw_text(t, rect.x + 15.0, rect.y + 28.0, FONT_HEADER_SIZE, TEXT_PRIMARY);
    }
}

/// Draws a button and returns true if clicked.
pub fn draw_button(rect: Rect, text: &str, active: bool) -> bool {
    let mouse_pos = mouse_position().into();
    let hovered = rect.contains(mouse_pos);
    let is_clicked = hovered && is_mouse_button_pressed(MouseButton::Left);
    
    // Logic: Active means "highlighted" or "toggled on". 
    // Even if not active, it should react to hover.
    
    let mut color = if active { PANEL_BG } else { Color::new(0.05, 0.05, 0.05, 1.0) };
    let mut border_color = if active { PRIMARY } else { PANEL_BORDER }; // Active = Gold border
    let mut text_color = if active { TEXT_HIGHLIGHT } else { TEXT_SECONDARY };

    if hovered {
        color = Color::new(0.15, 0.17, 0.2, 1.0); // Lighter on hover
        border_color = PRIMARY; // Gold border on hover
        text_color = TEXT_PRIMARY;
        
        if is_mouse_button_down(MouseButton::Left) {
            color = Color::new(0.1, 0.12, 0.15, 1.0); // Darker on press
        }
    }

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, border_color);

    // Center text
    let dimensions = measure_text(text, None, (rect.h * 0.4) as u16, 1.0); // Slightly smaller text
    let text_x = rect.x + (rect.w - dimensions.width) * 0.5;
    let text_y = rect.y + (rect.h + dimensions.height) * 0.5;
    
    draw_text(text, text_x, text_y, rect.h * 0.4, text_color);

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

     draw_rectangle(x, y, w, h, Color::new(0.05, 0.05, 0.08, 0.95));
     draw_rectangle_lines(x, y, w, h, 1.0, PRIMARY);
     draw_text(text, x + pad, y + h - pad, font_size, TEXT_PRIMARY);
}
