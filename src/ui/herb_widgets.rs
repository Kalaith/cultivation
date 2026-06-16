use crate::data::herbs::Season;
use crate::ui::components::draw_progress_bar;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub fn draw_season_indicator(x: f32, y: f32, current_season: &Season, ticks_remaining: u32) {
    let season_color = match current_season {
        Season::Spring => Color::new(0.5, 0.9, 0.5, 1.0),
        Season::Summer => Color::new(0.9, 0.7, 0.2, 1.0),
        Season::Autumn => Color::new(0.9, 0.5, 0.2, 1.0),
        Season::Winter => Color::new(0.7, 0.8, 0.95, 1.0),
    };

    draw_ui_text(
        &format!("{}", current_season),
        x,
        y,
        FONT_BODY_SIZE,
        season_color,
    );

    let progress = 1.0 - (ticks_remaining as f32 / 3600.0);
    draw_progress_bar(
        Rect::new(x + 70.0, y - 12.0, 80.0, 10.0),
        progress,
        season_color,
    );
}
