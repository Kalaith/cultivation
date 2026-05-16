use super::*;

impl SectBaseState {
    pub(super) fn draw_header(
        &mut self,
        screen_w: f32,
        header_h: f32,
        spirit_stones: u32,
        herbs: u32,
        influence: u32,
        relics: u32,
        current_season: &Season,
        season_ticks: u32,
        tutorial: &mut TutorialState,
    ) {
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("SECT MANAGEMENT", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);

        herbs::draw_season_indicator(250.0, 40.0, current_season, season_ticks);

        let res_text = format!("SS: {}  Herbs: {}  Infl: {}  Relics: {}", spirit_stones, herbs, influence, relics);
        let res_dims = measure_text(&res_text, None, FONT_HEADER_SIZE as u16, 1.0);

        draw_text(&res_text, screen_w - res_dims.width - 60.0, 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        if tutorial.active && tutorial.hidden {
            if draw_button(Rect::new(screen_w - 100.0, 10.0, 40.0, 40.0), "?", false) {
                tutorial.hidden = false;
            }
        }

        if draw_button(Rect::new(screen_w - 50.0, 10.0, 40.0, 40.0), "O", false) {
            self.settings_open = !self.settings_open;
        }
    }

    pub(super) fn draw_left_panel(&mut self, header_h: f32, screen_h: f32, width: f32, data: &GameData) -> Option<UpdateResult> {
        let rect = Rect::new(0.0, header_h, width, screen_h - header_h);
        draw_panel(rect, Some("Buildings"));

        let mut btn_y = rect.y + 40.0;
        for building in &data.buildings {
            let status_str = match building.status {
                BuildingStatus::Active => "",
                BuildingStatus::Ruined => " (Ruined)",
                BuildingStatus::Constructing => " (Building...)",
            };
            let label = format!("{}{}", building.building_type, status_str);

            if draw_button_muted(Rect::new(rect.x + 10.0, btn_y, width - 20.0, 35.0), &label, false) {
                self.view = SectView::BuildingDetails(building.id);
            }
            btn_y += 40.0;

            if btn_y > rect.y + rect.h - 200.0 {
                draw_text("...", rect.x + 10.0, btn_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                break;
            }
        }

        let nav_y_start = rect.y + rect.h - 270.0;
        if draw_button_muted(Rect::new(rect.x + 10.0, nav_y_start, width - 20.0, 40.0), "Disciples", false) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster));
        }
        if draw_button_muted(Rect::new(rect.x + 10.0, nav_y_start + 50.0, width - 20.0, 40.0), "World Map", false) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToWorldMap));
        }
        if draw_button_muted(Rect::new(rect.x + 10.0, nav_y_start + 100.0, width - 20.0, 40.0), "Factions", false) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToFactionScreen));
        }
        if draw_button_muted(Rect::new(rect.x + 10.0, nav_y_start + 150.0, width - 20.0, 40.0), "Trade", false) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToTradeScreen));
        }
        if draw_button_muted(Rect::new(rect.x + 10.0, nav_y_start + 200.0, width - 20.0, 40.0), "Construction", false) {
            self.view = SectView::Map;
            self.crafting_modal_open = true;
        }
        if draw_button_muted(Rect::new(rect.x + 10.0, nav_y_start + 250.0, width - 20.0, 40.0), "Spirit Beasts", false) {
            self.view = SectView::SpiritBeasts;
        }

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.12));
        None
    }

    pub(super) fn draw_right_panel(&self, header_h: f32, screen_h: f32, left_w: f32, center_w: f32, width: f32, event_log: &[String]) {
        let rect = Rect::new(left_w + center_w, header_h, width, screen_h - header_h);
        self.draw_event_log_panel(rect, event_log, screen_h);
    }

    fn draw_event_log_panel(&self, rect: Rect, event_log: &[String], screen_h: f32) {
        draw_panel(rect, Some("Event Log"));
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.35));

        let mut log_y = rect.y + 50.0;
        let max_width = rect.w - 20.0;

        for event in event_log.iter().rev().take(20) {
            let words: Vec<&str> = event.split_whitespace().collect();
            let mut current_line = String::new();

            for word in words {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };

                if test_line.len() as f32 * 7.0 > max_width {
                    draw_text(&current_line, rect.x + 10.0, log_y, FONT_SMALL_SIZE, Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.55));
                    log_y += 20.0;
                    current_line = word.to_string();
                } else {
                    current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                draw_text(&current_line, rect.x + 10.0, log_y, FONT_SMALL_SIZE, Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.55));
                log_y += 20.0;
            }
            log_y += 6.0;
            if log_y > screen_h - 20.0 { break; }
        }
    }

    pub(super) fn draw_settings_modal(&mut self, screen_w: f32, screen_h: f32) -> Option<UpdateResult> {
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

        let modal_w = 300.0;
        let modal_h = 250.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;
        let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);

        draw_panel(modal_rect, Some("Settings"));

        if draw_button(Rect::new(modal_x + 50.0, modal_y + 60.0, 200.0, 40.0), "Save Game", false) {
            return Some(UpdateResult::new().with_action(Action::SaveGame));
        }

        if draw_button(Rect::new(modal_x + 50.0, modal_y + 120.0, 200.0, 40.0), "Exit to Menu", false) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
        }

        if draw_button(Rect::new(modal_x + 50.0, modal_y + 180.0, 200.0, 40.0), "Close", false) {
            self.settings_open = false;
        }
        None
    }
}
