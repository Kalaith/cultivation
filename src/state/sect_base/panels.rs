use super::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

impl SectBaseState {
    pub(super) fn draw_header(
        &mut self,
        screen_w: f32,
        header_h: f32,
        spirit_stones: u32,
        herbs: u32,
        influence: u32,
        relics: u32,
        sect_name: &str,
        current_season: &Season,
        season_ticks: u32,
        tutorial: &mut TutorialState,
    ) {
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        let title = if sect_name == "Unnamed Sect" {
            "Fallen Peak Sect"
        } else {
            sect_name
        };
        draw_screen_title(title, "Patriarch's command archive", 20.0, 31.0);

        herbs::draw_season_indicator(275.0, 40.0, current_season, season_ticks);

        let mut res_x = screen_w - 620.0;
        res_x += draw_resource_seal(res_x, 38.0, "Spirit Stones", spirit_stones, PRIMARY) + 8.0;
        res_x += draw_resource_seal(res_x, 38.0, "Medicinal Herbs", herbs, SUCCESS) + 8.0;
        res_x += draw_resource_seal(res_x, 38.0, "Sect Prestige", influence, SECONDARY) + 8.0;
        draw_resource_seal(res_x, 38.0, "Ancient Relics", relics, WARNING);

        if tutorial.active && tutorial.hidden {
            if draw_button(Rect::new(screen_w - 100.0, 10.0, 40.0, 40.0), "?", false) {
                tutorial.hidden = false;
            }
        }

        if draw_button(Rect::new(screen_w - 50.0, 10.0, 40.0, 40.0), "O", false) {
            self.settings_open = !self.settings_open;
        }
    }

    pub(super) fn draw_left_panel(
        &mut self,
        header_h: f32,
        screen_h: f32,
        width: f32,
        data: &GameData,
    ) -> Option<UpdateResult> {
        let rect = Rect::new(16.0, header_h + 16.0, width, screen_h - header_h - 32.0);
        draw_panel(rect, Some("Sect Grounds"));
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.0, 0.0, 0.0, 0.16),
        );

        let mut btn_y = rect.y + 40.0;
        for building in &data.buildings {
            let status_str = match building.status {
                BuildingStatus::Active => "",
                BuildingStatus::Ruined => " (Ruined)",
                BuildingStatus::Constructing => " (Building...)",
            };
            let label = format!("{}{}", building.building_type, status_str);

            if draw_button_muted(
                Rect::new(rect.x + 10.0, btn_y, width - 20.0, 35.0),
                &label,
                false,
            ) {
                self.view = SectView::BuildingDetails(building.id);
            }
            btn_y += 40.0;

            if btn_y > rect.y + rect.h - 315.0 {
                draw_ui_text("...", rect.x + 10.0, btn_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                break;
            }
        }

        let nav_y_start = rect.y + rect.h - 270.0;
        draw_ink_divider(rect.x + 14.0, nav_y_start - 16.0, width - 28.0);
        draw_ui_text(
            "Sect Archives",
            rect.x + 14.0,
            nav_y_start - 24.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start, width - 20.0, 40.0),
            "Character Scrolls",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 50.0, width - 20.0, 40.0),
            "Regional Map",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToWorldMap));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 100.0, width - 20.0, 40.0),
            "Known Powers",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToFactionScreen));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 150.0, width - 20.0, 40.0),
            "Market Pavilion",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToTradeScreen));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 200.0, width - 20.0, 40.0),
            "Raise Halls",
            false,
        ) {
            self.view = SectView::Map;
            self.crafting_modal_open = true;
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 250.0, width - 20.0, 40.0),
            "Spirit Beasts",
            false,
        ) {
            self.view = SectView::SpiritBeasts;
        }

        None
    }

    pub(super) fn draw_right_panel(
        &self,
        header_h: f32,
        screen_h: f32,
        left_w: f32,
        center_w: f32,
        width: f32,
        event_log: &[String],
    ) {
        let rect = Rect::new(
            screen_width() - width - 16.0,
            header_h + 16.0,
            width,
            screen_h - header_h - 32.0,
        );
        self.draw_event_log_panel(rect, event_log, screen_h);
    }

    fn draw_event_log_panel(&self, rect: Rect, event_log: &[String], screen_h: f32) {
        draw_panel(rect, Some("Sect Annals"));
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.0, 0.0, 0.0, 0.22),
        );

        let mut log_y = rect.y + 50.0;
        let max_width = rect.w - 20.0;

        for (idx, event) in event_log.iter().rev().take(20).enumerate() {
            let event_color = self.event_color(event, idx);
            let words: Vec<&str> = event.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };

                if test_line.len() as f32 * 7.0 > max_width {
                    draw_ui_text(
                        &current_line,
                        rect.x + 10.0,
                        log_y,
                        FONT_SMALL_SIZE,
                        event_color,
                    );
                    log_y += 20.0;
                    current_line = word.to_string();
                } else {
                    current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                draw_ui_text(
                    &current_line,
                    rect.x + 10.0,
                    log_y,
                    FONT_SMALL_SIZE,
                    event_color,
                );
                log_y += 20.0;
            }
            log_y += 6.0;
            if log_y > screen_h - 20.0 {
                break;
            }
        }
    }

    fn event_color(&self, event: &str, idx: usize) -> Color {
        let lower = event.to_ascii_lowercase();
        let base_alpha = (0.78 - idx as f32 * 0.035).clamp(0.35, 0.78);
        if lower.contains("breakthrough") || lower.contains("ready") {
            Color::new(WARNING.r, WARNING.g, WARNING.b, base_alpha)
        } else if lower.contains("war") || lower.contains("cannot") || lower.contains("danger") {
            Color::new(FAILURE.r, FAILURE.g, FAILURE.b, base_alpha)
        } else if lower.contains("recovered") || lower.contains("saved") {
            Color::new(SUCCESS.r, SUCCESS.g, SUCCESS.b, base_alpha)
        } else {
            Color::new(
                TEXT_SECONDARY.r,
                TEXT_SECONDARY.g,
                TEXT_SECONDARY.b,
                base_alpha,
            )
        }
    }

    pub(super) fn draw_settings_modal(
        &mut self,
        screen_w: f32,
        screen_h: f32,
    ) -> Option<UpdateResult> {
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

        let modal_w = 300.0;
        let modal_h = 250.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;
        let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);

        draw_panel(modal_rect, Some("Settings"));

        if draw_button(
            Rect::new(modal_x + 50.0, modal_y + 60.0, 200.0, 40.0),
            "Save Game",
            false,
        ) {
            return Some(UpdateResult::new().with_action(Action::SaveGame));
        }

        if draw_button(
            Rect::new(modal_x + 50.0, modal_y + 120.0, 200.0, 40.0),
            "Exit to Menu",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
        }

        if draw_button(
            Rect::new(modal_x + 50.0, modal_y + 180.0, 200.0, 40.0),
            "Close",
            false,
        ) {
            self.settings_open = false;
        }
        None
    }
}
