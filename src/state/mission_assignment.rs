use crate::data::disciples::{Disciple, DiscipleRank};
use crate::data::loader::GameData;
use crate::data::missions::{Mission, MissionType, RelevantStat};
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub struct MissionAssignmentState {
    pub mission_description: String,
    selected_disciples: Vec<usize>,
    roster_scroll: f32,
}

impl MissionAssignmentState {
    pub fn new(mission_description: String) -> Self {
        Self {
            mission_description,
            selected_disciples: Vec::new(),
            roster_scroll: 0.0,
        }
    }

    pub fn update(&mut self, data: &GameData, disciples: &[Disciple]) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let mission = mission_for_description(data, &self.mission_description);
        draw_mountain_sect_backdrop();

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 86.0;
        self.draw_header(screen_w, header_h, mission);

        let content_y = header_h + 16.0;
        let content_h = screen_h - content_y - 22.0;
        let gutter = 18.0;
        let left_w = (screen_w * 0.34).clamp(340.0, 430.0);
        let right_w = 286.0;
        let center_w = screen_w - left_w - right_w - gutter * 4.0;

        let dossier_rect = Rect::new(gutter, content_y, left_w, content_h);
        let roster_rect = Rect::new(
            dossier_rect.x + left_w + gutter,
            content_y,
            center_w,
            content_h,
        );
        let orders_rect = Rect::new(
            roster_rect.x + center_w + gutter,
            content_y,
            right_w,
            content_h,
        );

        self.draw_mission_dossier(dossier_rect, mission);
        self.draw_roster(roster_rect, data, mission, disciples);

        if let Some(result) = self.draw_orders(orders_rect, mission, disciples) {
            return result;
        }

        UpdateResult::new()
    }

    fn draw_header(&self, screen_w: f32, header_h: f32, mission: Option<&Mission>) {
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_screen_title(
            "Dispatch Beyond the Gate",
            "Choose who carries the sect's mandate into danger",
            24.0,
            36.0,
        );

        let danger = mission.map(|m| m.danger_level).unwrap_or(0);
        let accent = danger_color(danger);
        let mut seal_x = screen_w - 430.0;
        seal_x += draw_resource_seal(
            seal_x,
            54.0,
            "Chosen",
            self.selected_disciples.len() as u32,
            PRIMARY,
        ) + 8.0;
        seal_x += draw_resource_seal(seal_x, 54.0, "Risk", danger, accent) + 8.0;
        draw_resource_seal(
            seal_x,
            54.0,
            "Ticks",
            mission.map(|m| m.duration).unwrap_or(0),
            SECONDARY,
        );
    }

    fn draw_mission_dossier(&self, rect: Rect, mission: Option<&Mission>) {
        draw_panel(rect, Some("Dispatch Dossier"));

        let title = mission
            .map(|m| m.description.as_str())
            .unwrap_or(self.mission_description.as_str());
        let after_title_y = draw_wrapped_text(
            title,
            rect.x + 22.0,
            rect.y + 68.0,
            rect.w - 44.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );
        let divider_y = after_title_y + 8.0;
        draw_ink_divider(rect.x + 22.0, divider_y, rect.w - 44.0);

        if let Some(mission) = mission {
            let stat = mission.mission_type.get_relevant_stat();
            let danger = mission.danger_level;
            let accent = danger_color(danger);
            let mut y = divider_y + 42.0;

            self.draw_dossier_row(
                rect.x + 24.0,
                y,
                "Order",
                mission_type_label(&mission.mission_type),
                SECONDARY,
            );
            y += 42.0;
            self.draw_dossier_row(
                rect.x + 24.0,
                y,
                "Primary Test",
                relevant_stat_label(stat),
                PRIMARY,
            );
            y += 42.0;
            self.draw_dossier_row(rect.x + 24.0, y, "Threat", danger_label(danger), accent);
            y += 42.0;
            self.draw_dossier_row(
                rect.x + 24.0,
                y,
                "Journey",
                &format!("{} ticks beyond the gate", mission.duration),
                TEXT_PRIMARY,
            );

            draw_ink_divider(rect.x + 22.0, rect.y + rect.h - 166.0, rect.w - 44.0);
            draw_wrapped_text(
                &dossier_text(mission),
                rect.x + 24.0,
                rect.y + rect.h - 128.0,
                rect.w - 48.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        } else {
            draw_wrapped_text(
                "The dispatch scroll is incomplete. The patriarch can still issue orders, but the omens are obscured.",
                rect.x + 24.0,
                rect.y + 158.0,
                rect.w - 48.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }
    }

    fn draw_dossier_row(&self, x: f32, y: f32, label: &str, value: &str, color: Color) {
        draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
        draw_ui_text(value, x + 116.0, y, FONT_BODY_SIZE, color);
    }

    fn draw_roster(
        &mut self,
        rect: Rect,
        data: &GameData,
        mission: Option<&Mission>,
        disciples: &[Disciple],
    ) {
        draw_panel(rect, Some("Disciple Oath Slips"));

        let list_rect = Rect::new(rect.x + 14.0, rect.y + 56.0, rect.w - 28.0, rect.h - 78.0);
        let card_h = 86.0;
        let gap = 10.0;
        let total_h = disciples.len() as f32 * (card_h + gap);

        if list_rect.contains(mouse_position().into()) {
            let wheel = mouse_wheel().1;
            if total_h > list_rect.h {
                self.roster_scroll = (self.roster_scroll - wheel * 42.0)
                    .clamp(0.0, (total_h - list_rect.h).max(0.0));
            } else {
                self.roster_scroll = 0.0;
            }
        }

        let mut y = list_rect.y - self.roster_scroll;
        for (idx, disciple) in disciples.iter().enumerate() {
            let card = Rect::new(list_rect.x, y, list_rect.w, card_h);
            if card.y + card.h >= list_rect.y && card.y <= list_rect.y + list_rect.h {
                let selected = self.selected_disciples.contains(&idx);
                let clicked = draw_disciple_dispatch_card(card, data, mission, disciple, selected);
                if clicked && !disciple.is_injured() {
                    self.toggle_disciple(idx);
                }
            }
            y += card_h + gap;
        }

        if total_h > list_rect.h {
            draw_scrollbar(list_rect, self.roster_scroll, total_h);
        }

        draw_ui_text(
            "The gate accepts no more than three names.",
            rect.x + 18.0,
            rect.y + rect.h - 12.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn toggle_disciple(&mut self, idx: usize) {
        if let Some(pos) = self
            .selected_disciples
            .iter()
            .position(|&selected| selected == idx)
        {
            self.selected_disciples.remove(pos);
        } else if self.selected_disciples.len() < 3 {
            self.selected_disciples.push(idx);
        }
    }

    fn draw_orders(
        &self,
        rect: Rect,
        mission: Option<&Mission>,
        disciples: &[Disciple],
    ) -> Option<UpdateResult> {
        draw_panel(rect, Some("Patriarch's Order"));

        let active = !self.selected_disciples.is_empty();
        let team_stat = team_mission_stat(&self.selected_disciples, mission, disciples);
        let required = mission_required_stat(mission);
        let prospect = mission_prospect(team_stat, required);

        draw_ui_text(
            "Chosen Disciples",
            rect.x + 18.0,
            rect.y + 68.0,
            FONT_BODY_SIZE,
            TEXT_HIGHLIGHT,
        );
        let mut y = rect.y + 98.0;
        for (slot, idx) in self.selected_disciples.iter().enumerate() {
            let label = disciples
                .get(*idx)
                .map(|d| format!("{}. {}", slot + 1, d.name))
                .unwrap_or_else(|| format!("{}. Unknown", slot + 1));
            draw_ui_text(&label, rect.x + 22.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
            y += 30.0;
        }
        while y < rect.y + 188.0 {
            draw_ui_text(
                "- empty oath slip",
                rect.x + 22.0,
                y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
            y += 30.0;
        }

        draw_ink_divider(rect.x + 18.0, rect.y + 206.0, rect.w - 36.0);
        draw_order_metric(
            rect.x + 20.0,
            rect.y + 246.0,
            "Team Merit",
            team_stat,
            PRIMARY,
        );
        draw_order_metric(
            rect.x + 20.0,
            rect.y + 292.0,
            "Omen Threshold",
            required,
            danger_color(mission.map(|m| m.danger_level).unwrap_or(0)),
        );

        draw_wrapped_text(
            prospect,
            rect.x + 20.0,
            rect.y + 340.0,
            rect.w - 40.0,
            FONT_BODY_SIZE,
            if active { TEXT_PRIMARY } else { TEXT_SECONDARY },
        );

        let depart_rect = Rect::new(rect.x + 18.0, rect.y + rect.h - 118.0, rect.w - 36.0, 50.0);
        let cancel_rect = Rect::new(rect.x + 18.0, rect.y + rect.h - 58.0, rect.w - 36.0, 40.0);

        if draw_button(
            depart_rect,
            if active {
                "Seal the Order"
            } else {
                "Awaiting Names"
            },
            active,
        ) && active
        {
            return Some(
                UpdateResult::new()
                    .with_action(Action::StartMission(
                        self.mission_description.clone(),
                        self.selected_disciples.clone(),
                    ))
                    .with_transition(StateTransition::ToSectBase),
            );
        }

        if draw_button_muted(cancel_rect, "Return to the Hall", true) {
            return Some(UpdateResult::new().with_transition(StateTransition::ToSectBase));
        }

        None
    }

    pub fn draw(&self, _data: &GameData, _disciples: &[Disciple], _spirit_stones: u32) {
        // Handled in update.
    }
}

fn mission_for_description<'a>(data: &'a GameData, description: &str) -> Option<&'a Mission> {
    data.missions.iter().find(|m| m.description == description)
}

fn draw_disciple_dispatch_card(
    rect: Rect,
    data: &GameData,
    mission: Option<&Mission>,
    disciple: &Disciple,
    selected: bool,
) -> bool {
    let hover = rect.contains(mouse_position().into());
    let injured = disciple.is_injured();
    let accent = if injured {
        FAILURE
    } else if selected {
        PRIMARY
    } else if disciple.can_attempt_breakthrough() {
        WARNING
    } else {
        SECONDARY
    };
    let alpha = if selected {
        0.66
    } else if hover {
        0.48
    } else {
        0.34
    };

    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.026, 0.018, alpha),
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
            if selected { 0.86 } else { 0.50 },
        ),
    );

    draw_circle(
        rect.x + 24.0,
        rect.y + 30.0,
        11.0,
        Color::new(accent.r, accent.g, accent.b, 0.46),
    );
    draw_ui_text(
        &disciple.name,
        rect.x + 44.0,
        rect.y + 28.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "{} | {}",
            rank_label(&disciple.rank),
            realm_label(data, disciple)
        ),
        rect.x + 44.0,
        rect.y + 52.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );

    let stat = disciple_mission_stat(mission, disciple);
    let stat_label = mission
        .map(|m| relevant_stat_label(m.mission_type.get_relevant_stat()))
        .unwrap_or("Merit");
    draw_ui_text(
        &format!("{} {}", stat_label, stat),
        rect.x + 44.0,
        rect.y + 76.0,
        FONT_SMALL_SIZE,
        accent,
    );

    let status = if injured {
        "Recovering"
    } else if selected {
        "Oath sealed"
    } else if disciple.can_attempt_breakthrough() {
        "Near ascension"
    } else {
        "Available"
    };
    let dims = measure_ui_text(status, None, FONT_SMALL_SIZE as u16, 1.0);
    draw_ui_text(
        status,
        rect.x + rect.w - dims.width - 14.0,
        rect.y + 30.0,
        FONT_SMALL_SIZE,
        accent,
    );

    hover && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_scrollbar(rect: Rect, offset: f32, total_h: f32) {
    let handle_h = (rect.h * rect.h / total_h).max(24.0);
    let max_offset = (total_h - rect.h).max(1.0);
    let handle_y = rect.y + (offset / max_offset) * (rect.h - handle_h);
    draw_rectangle(
        rect.x + rect.w - 5.0,
        rect.y,
        3.0,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.28),
    );
    draw_rectangle(
        rect.x + rect.w - 6.0,
        handle_y,
        5.0,
        handle_h,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.62),
    );
}

fn draw_order_metric(x: f32, y: f32, label: &str, value: u32, color: Color) {
    draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
    draw_ui_text(&value.to_string(), x + 152.0, y, FONT_BODY_SIZE, color);
}

fn mission_type_label(mission_type: &MissionType) -> &'static str {
    match mission_type {
        MissionType::Exploration => "Exploration",
        MissionType::ResourceGathering => "Resource Gathering",
        MissionType::MonsterSuppression => "Monster Hunt",
        MissionType::Diplomacy => "Diplomacy",
        MissionType::RuinDelve => "Ruin Delve",
    }
}

fn relevant_stat_label(stat: RelevantStat) -> &'static str {
    match stat {
        RelevantStat::Body => "Body",
        RelevantStat::Mind => "Mind",
        RelevantStat::Spirit => "Spirit",
    }
}

fn danger_label(danger: u32) -> &'static str {
    match danger {
        0..=1 => "Auspicious",
        2..=3 => "Uncertain",
        4..=5 => "Perilous",
        _ => "Deadly",
    }
}

fn danger_color(danger: u32) -> Color {
    match danger {
        0..=1 => SECONDARY,
        2..=3 => PRIMARY,
        4..=5 => WARNING,
        _ => FAILURE,
    }
}

fn dossier_text(mission: &Mission) -> String {
    match mission.mission_type {
        MissionType::Exploration => "Send sharp eyes and steady spirits. The mountain needs paths, contacts, and forgotten places more than brute force.".to_string(),
        MissionType::ResourceGathering => "The sect cannot rebuild on vows alone. The chosen disciples must return with herbs, stones, and useful salvage.".to_string(),
        MissionType::MonsterSuppression => "Beasts and corrupted things test the body first. Send disciples who can stand between the sect and clawed disaster.".to_string(),
        MissionType::Diplomacy => "Words can spare blood or invite it. Mind, restraint, and face matter more than raw power beyond this gate.".to_string(),
        MissionType::RuinDelve => "Old ruins remember old deaths. Body carries the expedition through traps before spirit can claim the prize.".to_string(),
    }
}

fn rank_label(rank: &DiscipleRank) -> &'static str {
    match rank {
        DiscipleRank::Outer => "Outer",
        DiscipleRank::Inner => "Inner",
        DiscipleRank::Elder => "Elder",
        DiscipleRank::SectLeader => "Patriarch",
    }
}

fn realm_label(data: &GameData, disciple: &Disciple) -> String {
    let realm = data
        .stages
        .get(&disciple.realm)
        .map(|stage| stage.name.as_str())
        .unwrap_or("Unknown Realm");
    let sub = data
        .stages
        .get(&disciple.realm)
        .and_then(|stage| stage.sub_stages.get(disciple.sub_stage))
        .map(|sub| sub.name.as_str())
        .unwrap_or("");

    if sub.is_empty() {
        realm.to_string()
    } else {
        format!("{} {}", sub, realm)
    }
}

fn disciple_mission_stat(mission: Option<&Mission>, disciple: &Disciple) -> u32 {
    match mission.map(|m| m.mission_type.get_relevant_stat()) {
        Some(RelevantStat::Body) => disciple.attributes.body,
        Some(RelevantStat::Mind) => disciple.attributes.mind,
        Some(RelevantStat::Spirit) => disciple.attributes.spirit,
        None => {
            (disciple.attributes.body + disciple.attributes.mind + disciple.attributes.spirit) / 3
        }
    }
}

fn team_mission_stat(selected: &[usize], mission: Option<&Mission>, disciples: &[Disciple]) -> u32 {
    selected
        .iter()
        .filter_map(|idx| disciples.get(*idx))
        .map(|disciple| disciple_mission_stat(mission, disciple))
        .sum()
}

fn mission_required_stat(mission: Option<&Mission>) -> u32 {
    mission
        .map(|m| 18 + m.danger_level.saturating_mul(14))
        .unwrap_or(36)
}

fn mission_prospect(team_stat: u32, required: u32) -> &'static str {
    if team_stat == 0 {
        return "No names have been written on the dispatch order.";
    }
    if team_stat >= required + 28 {
        "The omens favor this expedition. The sect may gain more than it risks."
    } else if team_stat >= required {
        "The order is sound, but danger still waits beyond the gate."
    } else {
        "This order asks too much of the chosen disciples. Injury or failure is likely."
    }
}
