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
        draw_screen_title(title, "Mountain command ledger", 20.0, 31.0);

        let title_w = measure_ui_text(title, None, FONT_TITLE_SIZE as u16, 1.0).width;
        herbs::draw_season_indicator(20.0 + title_w + 28.0, 40.0, current_season, season_ticks);

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

        if draw_button(Rect::new(screen_w - 50.0, 10.0, 40.0, 40.0), "S", false) {
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

        self.draw_mountain_mandate(rect, data);

        let mut btn_y = rect.y + 162.0;
        for building in &data.buildings {
            let status_str = match building.status {
                BuildingStatus::Active => "",
                BuildingStatus::Ruined => " (Ruined)",
                BuildingStatus::Constructing => " (Raising...)",
            };
            let label = format!("{}{}", building.building_type, status_str);

            let btn_rect = Rect::new(rect.x + 10.0, btn_y, width - 20.0, 35.0);
            if matches!(self.view, SectView::Map) {
                match building.building_type {
                    BuildingType::SectHall => {
                        self.register_tutorial_target(0, btn_rect);
                        self.register_tutorial_target(1, btn_rect);
                        self.register_tutorial_target(3, btn_rect);
                    }
                    BuildingType::MissionBoard => {
                        self.register_tutorial_target(4, btn_rect);
                    }
                    _ => {}
                }
            }
            if draw_button_muted(btn_rect, &label, false) {
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
            "Patriarch's Ledgers",
            rect.x + 14.0,
            nav_y_start - 24.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start, width - 20.0, 40.0),
            "Disciple Scrolls",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 50.0, width - 20.0, 40.0),
            "Outside Routes",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToWorldMap));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 100.0, width - 20.0, 40.0),
            "Regional Powers",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToFactionScreen));
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 150.0, width - 20.0, 40.0),
            "Caravan Ledger",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToTradeScreen));
        }
        let raise_rect = Rect::new(rect.x + 10.0, nav_y_start + 200.0, width - 20.0, 40.0);
        if !self.crafting_modal_open {
            self.register_tutorial_target(2, raise_rect);
        }
        if draw_button_muted(raise_rect, "Raise Halls", false) {
            self.view = SectView::Map;
            self.crafting_modal_open = true;
        }
        if draw_button_muted(
            Rect::new(rect.x + 10.0, nav_y_start + 250.0, width - 20.0, 40.0),
            "Mountain Guardians",
            false,
        ) {
            self.view = SectView::SpiritBeasts;
        }

        None
    }

    fn draw_mountain_mandate(&self, rect: Rect, data: &GameData) {
        let active = data
            .buildings
            .iter()
            .filter(|b| b.status == BuildingStatus::Active)
            .count() as u32;
        let ruined = data
            .buildings
            .iter()
            .filter(|b| b.status == BuildingStatus::Ruined)
            .count() as u32;
        let raising = data
            .buildings
            .iter()
            .filter(|b| b.status == BuildingStatus::Constructing)
            .count() as u32;
        let y = rect.y + 52.0;

        draw_ui_text(
            "Mountain Mandate",
            rect.x + 14.0,
            y,
            FONT_SMALL_SIZE,
            TEXT_HIGHLIGHT,
        );
        draw_ink_divider(rect.x + 14.0, y + 10.0, rect.w - 28.0);
        draw_small_mandate_stat(rect.x + 14.0, y + 38.0, "Active", active, SUCCESS);
        draw_small_mandate_stat(rect.x + 88.0, y + 38.0, "Ruined", ruined, FAILURE);
        draw_small_mandate_stat(rect.x + 162.0, y + 38.0, "Raising", raising, WARNING);
        draw_ui_text(
            "Restore halls before ambition outruns shelter.",
            rect.x + 14.0,
            y + 88.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
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

        draw_ui_text(
            "Recent Edicts and Omens",
            rect.x + 14.0,
            rect.y + 52.0,
            FONT_SMALL_SIZE,
            TEXT_HIGHLIGHT,
        );
        draw_ink_divider(rect.x + 14.0, rect.y + 62.0, rect.w - 28.0);

        if event_log.is_empty() {
            draw_wrapped_text(
                "The annals are waiting for the patriarch's first decree.",
                rect.x + 14.0,
                rect.y + 98.0,
                rect.w - 28.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            return;
        }

        let mut log_y = rect.y + 96.0;
        let max_width = rect.w - 36.0;

        for (idx, event) in event_log.iter().rev().take(20).enumerate() {
            let event_color = self.event_color(event, idx);
            draw_circle(
                rect.x + 13.0,
                log_y - 6.0,
                3.0,
                Color::new(event_color.r, event_color.g, event_color.b, event_color.a),
            );
            log_y = draw_wrapped_text(
                event,
                rect.x + 22.0,
                log_y,
                max_width,
                FONT_SMALL_SIZE,
                event_color,
            );
            log_y += 6.0;
            if log_y > screen_h - 20.0 {
                break;
            }
        }
    }

    fn event_color(&self, event: &str, idx: usize) -> Color {
        let lower = event.to_ascii_lowercase();
        let base_alpha = (0.95 - idx as f32 * 0.02).clamp(0.60, 0.95);
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

        draw_panel(modal_rect, Some("Patriarch's Seal"));

        if draw_button(
            Rect::new(modal_x + 50.0, modal_y + 60.0, 200.0, 40.0),
            "Inscribe Chronicle",
            false,
        ) {
            return Some(UpdateResult::new().with_action(Action::SaveGame));
        }

        if draw_button(
            Rect::new(modal_x + 50.0, modal_y + 120.0, 200.0, 40.0),
            "Return to Title",
            false,
        ) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
        }

        if draw_button(
            Rect::new(modal_x + 50.0, modal_y + 180.0, 200.0, 40.0),
            "Close Seal",
            false,
        ) {
            self.settings_open = false;
        }
        None
    }
}

fn draw_small_mandate_stat(x: f32, y: f32, label: &str, value: u32, color: Color) {
    draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
    draw_ui_text(
        &value.to_string(),
        x,
        y + 20.0,
        FONT_BODY_SIZE,
        Color::new(color.r, color.g, color.b, 0.92),
    );
}
