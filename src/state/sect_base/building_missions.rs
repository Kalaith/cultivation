use super::*;
use crate::data::missions::{Mission, MissionType};
use macroquad_toolkit::ui::draw_ui_text;

impl SectBaseState {
    pub(super) fn draw_mission_list(
        &mut self,
        rect: Rect,
        data: &GameData,
        ongoing_missions: &[OngoingMission],
        completed_missions: &[MissionOutcome],
        completed_history: &[String],
        start_y: f32,
    ) -> Option<UpdateResult> {
        let list_rect = Rect::new(
            rect.x + 20.0,
            start_y,
            rect.w - 40.0,
            rect.h - start_y + rect.y - 95.0,
        );
        let card_h = 74.0;
        let card_gap = 10.0;
        let mut available_missions: Vec<&Mission> = Vec::new();
        let mut selected_available = false;
        let selected_desc = self.selected_mission.clone();
        let mouse = vec2(mouse_position().0, mouse_position().1);

        for mission in &data.missions {
            let is_ongoing = ongoing_missions
                .iter()
                .any(|m| m.mission.description == mission.description);
            let is_pending = completed_missions
                .iter()
                .any(|m| m.description == mission.description);
            let is_historically_complete = completed_history.contains(&mission.description);

            let available = if mission.repeatable {
                !is_ongoing && !is_pending
            } else {
                !is_ongoing && !is_pending && !is_historically_complete
            };

            if available {
                available_missions.push(mission);
            }
        }

        let total_h = available_missions.len() as f32 * (card_h + card_gap);
        if list_rect.contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if total_h > list_rect.h {
                self.mission_scroll -= wheel * 32.0;
                self.mission_scroll = self
                    .mission_scroll
                    .clamp(0.0, (total_h - list_rect.h).max(0.0));
            } else {
                self.mission_scroll = 0.0;
            }
        }

        draw_ui_text(
            "Gate Dispatch Board",
            rect.x + 20.0,
            start_y - 16.0,
            FONT_HEADER_SIZE,
            PRIMARY,
        );

        let mut m_y = list_rect.y - self.mission_scroll;
        for mission in available_missions {
            let card_rect = Rect::new(list_rect.x, m_y, list_rect.w, card_h);
            let is_selected = selected_desc.as_deref() == Some(mission.description.as_str());
            if is_selected {
                selected_available = true;
            }

            if card_rect.y + card_h >= list_rect.y && card_rect.y <= list_rect.y + list_rect.h {
                self.draw_mission_card(card_rect, mission, is_selected);
            }

            if card_rect.contains(mouse.into()) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_mission = Some(mission.description.clone());
            }

            if card_rect.contains(mouse.into()) {
                draw_tooltip(
                    mouse,
                    &format!(
                        "{}\nOmen {} | {} ticks beyond the gate",
                        mission.description, mission.danger_level, mission.duration
                    ),
                );
            }

            m_y += card_h + card_gap;
        }

        if total_h > list_rect.h {
            draw_mission_scrollbar(list_rect, total_h, self.mission_scroll);
        }

        if let Some(selected) = &self.selected_mission {
            if selected_available {
                let btn_rect =
                    Rect::new(rect.x + 20.0, rect.y + rect.h - 60.0, rect.w - 40.0, 40.0);
                if draw_button(btn_rect, "Issue patriarch's dispatch", false) {
                    return Some(
                        UpdateResult::new().with_transition(StateTransition::ToMissionAssignment(
                            selected.clone(),
                        )),
                    );
                }
            } else {
                draw_ui_text(
                    "This dispatch has left the board.",
                    rect.x + 20.0,
                    rect.y + rect.h - 30.0,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );
            }
        }
        None
    }

    fn draw_mission_card(&self, rect: Rect, mission: &Mission, selected: bool) {
        let hover = rect.contains(mouse_position().into());
        let alpha = if selected {
            0.74
        } else if hover {
            0.52
        } else {
            0.34
        };
        let accent = mission_type_color(&mission.mission_type);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.035, 0.028, 0.02, alpha),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if selected { 2.0 } else { 1.0 },
            Color::new(
                accent.r,
                accent.g,
                accent.b,
                if selected { 0.88 } else { 0.38 },
            ),
        );
        draw_rectangle(
            rect.x,
            rect.y,
            5.0,
            rect.h,
            Color::new(accent.r, accent.g, accent.b, 0.72),
        );

        draw_ui_text(
            mission_title(&mission.description),
            rect.x + 16.0,
            rect.y + 24.0,
            FONT_BODY_SIZE,
            if selected {
                TEXT_PRIMARY
            } else {
                Color::new(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b, 0.78)
            },
        );
        draw_ui_text(
            &format!(
                "{} order | Omen {} | {} ticks",
                mission_type_label(&mission.mission_type),
                mission.danger_level,
                mission.duration
            ),
            rect.x + 16.0,
            rect.y + 47.0,
            FONT_SMALL_SIZE,
            Color::new(
                TEXT_SECONDARY.r,
                TEXT_SECONDARY.g,
                TEXT_SECONDARY.b,
                if selected { 0.92 } else { 0.62 },
            ),
        );
        draw_ui_text(
            mission_spoils(&mission.mission_type),
            rect.x + rect.w - 220.0,
            rect.y + 47.0,
            FONT_SMALL_SIZE,
            Color::new(
                PRIMARY.r,
                PRIMARY.g,
                PRIMARY.b,
                if selected { 0.9 } else { 0.48 },
            ),
        );
    }
}

fn draw_mission_scrollbar(list_rect: Rect, total_h: f32, mission_scroll: f32) {
    let track_x = list_rect.x + list_rect.w - 7.0;
    let handle_h = (list_rect.h * list_rect.h / total_h).max(24.0);
    let max_offset = (total_h - list_rect.h).max(1.0);
    let handle_y = list_rect.y + (mission_scroll / max_offset) * (list_rect.h - handle_h);
    draw_rectangle(
        track_x,
        list_rect.y,
        3.0,
        list_rect.h,
        Color::new(0.0, 0.0, 0.0, 0.32),
    );
    draw_rectangle(
        track_x - 1.0,
        handle_y,
        5.0,
        handle_h,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.62),
    );
}

fn mission_title(description: &str) -> &str {
    description.trim_end_matches('.')
}

fn mission_type_label(mission_type: &MissionType) -> &'static str {
    match mission_type {
        MissionType::Exploration => "Scouting",
        MissionType::ResourceGathering => "Provision",
        MissionType::MonsterSuppression => "Suppression",
        MissionType::Diplomacy => "Envoy",
        MissionType::RuinDelve => "Ruin delve",
    }
}

fn mission_spoils(mission_type: &MissionType) -> &'static str {
    match mission_type {
        MissionType::Exploration => "Returns: rumors, herbs",
        MissionType::ResourceGathering => "Returns: ore, stones",
        MissionType::MonsterSuppression => "Returns: hides, prestige",
        MissionType::Diplomacy => "Returns: favor, trade",
        MissionType::RuinDelve => "Returns: relics, doctrine",
    }
}

fn mission_type_color(mission_type: &MissionType) -> Color {
    match mission_type {
        MissionType::Exploration => SECONDARY,
        MissionType::ResourceGathering => PRIMARY,
        MissionType::MonsterSuppression => ACCENT,
        MissionType::Diplomacy => SUCCESS,
        MissionType::RuinDelve => WARNING,
    }
}
