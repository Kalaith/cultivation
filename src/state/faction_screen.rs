use crate::data::factions::{Faction, FactionSpecialty, FactionType};
use crate::data::relations::{FactionRelation, ReputationTier};
use crate::engine::world_sim::WorldSim;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub struct FactionScreenState {
    selected_faction: Option<usize>,
    scroll_offset: f32,
}

impl FactionScreenState {
    pub fn new() -> Self {
        Self {
            selected_faction: None,
            scroll_offset: 0.0,
        }
    }

    pub fn update(&mut self, world_sim: &WorldSim) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        draw_panel(Rect::new(0.0, 0.0, screen_w, 70.0), None);
        draw_screen_title(
            "Regional Power Ledger",
            "Rival sects, courts, caravans, and threats beyond the mountain gate",
            24.0,
            34.0,
        );

        let content_y = 90.0;
        let content_h = screen_h - 110.0;
        let list_w = 380.0;
        let list_rect = Rect::new(18.0, content_y, list_w, content_h);
        let detail_rect = Rect::new(
            list_rect.x + list_w + 18.0,
            content_y,
            screen_w - list_w - 54.0,
            content_h,
        );

        self.draw_faction_list(list_rect, world_sim);
        draw_panel(detail_rect, Some("Rival Power Record"));

        if let Some(idx) = self.selected_faction {
            if let Some(faction) = world_sim.factions.get(idx) {
                self.draw_faction_details(
                    detail_rect,
                    faction,
                    world_sim.get_relation(&faction.id),
                );
            }
        } else {
            draw_ui_text(
                "Choose a diplomatic tablet before issuing an edict.",
                detail_rect.x + 24.0,
                detail_rect.y + 88.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        if draw_button(
            Rect::new(22.0, screen_h - 58.0, 100.0, 40.0),
            "Return",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    fn draw_faction_list(&mut self, rect: Rect, world_sim: &WorldSim) {
        draw_panel(rect, Some("Diplomatic Tablets"));
        let row_h = 74.0;
        let gap = 8.0;
        let list_y = rect.y + 50.0;
        let list_h = rect.h - 64.0;
        let total_h = world_sim.factions.len() as f32 * (row_h + gap);
        let mouse = vec2(mouse_position().0, mouse_position().1);

        if Rect::new(rect.x, list_y, rect.w, list_h).contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if total_h > list_h {
                self.scroll_offset -= wheel * 32.0;
                self.scroll_offset = self.scroll_offset.clamp(0.0, (total_h - list_h).max(0.0));
            } else {
                self.scroll_offset = 0.0;
            }
        }

        let mut y = list_y - self.scroll_offset;
        for (i, faction) in world_sim.factions.iter().enumerate() {
            let row = Rect::new(rect.x + 12.0, y, rect.w - 24.0, row_h);
            if row.y + row_h >= list_y && row.y <= list_y + list_h {
                self.draw_faction_row(row, faction, world_sim.get_relation(&faction.id), i);
                if row.contains(mouse.into()) && is_mouse_button_pressed(MouseButton::Left) {
                    self.selected_faction = Some(i);
                }
            }
            y += row_h + gap;
        }
    }

    fn draw_faction_row(
        &self,
        rect: Rect,
        faction: &Faction,
        relation: Option<&FactionRelation>,
        idx: usize,
    ) {
        let selected = self.selected_faction == Some(idx);
        let hovered = rect.contains(mouse_position().into());
        let color = faction_color(&faction.faction_type);
        let alpha = if selected {
            0.70
        } else if hovered {
            0.48
        } else {
            0.32
        };
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
                color.r,
                color.g,
                color.b,
                if selected { 0.86 } else { 0.44 },
            ),
        );
        draw_ui_text(
            faction_sigill(&faction.faction_type),
            rect.x + 12.0,
            rect.y + 28.0,
            FONT_SMALL_SIZE,
            color,
        );
        draw_ui_text(
            &faction.name,
            rect.x + 74.0,
            rect.y + 25.0,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );
        let tier = relation
            .map(|rel| rel.reputation_tier())
            .unwrap_or(ReputationTier::Neutral);
        draw_ui_text(
            &format!("{} | {}", realm_label(&faction.leader_realm), tier),
            rect.x + 74.0,
            rect.y + 50.0,
            FONT_SMALL_SIZE,
            tier_color(tier),
        );
    }

    fn draw_faction_details(
        &self,
        rect: Rect,
        faction: &Faction,
        relation: Option<&FactionRelation>,
    ) {
        let x = rect.x + 28.0;
        let mut y = rect.y + 68.0;
        let tier = relation
            .map(|rel| rel.reputation_tier())
            .unwrap_or(ReputationTier::Neutral);

        draw_ui_text(&faction.name, x, y, FONT_TITLE_SIZE, PRIMARY);
        y += 34.0;
        draw_ui_text(
            faction_type_title(&faction.faction_type),
            x,
            y,
            FONT_BODY_SIZE,
            SECONDARY,
        );
        y += 34.0;

        let columns = [
            ("Seat Holder", leader_title(faction)),
            ("Realm", realm_label(&faction.leader_realm)),
            ("Sect Bearing", tier.to_string()),
            (
                "Known Strength",
                specialty_label(&faction.specialty).to_string(),
            ),
        ];
        for (label, value) in columns {
            draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
            draw_ui_text(&value, x + 150.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
            y += 28.0;
        }

        y += 10.0;
        draw_ink_divider(x, y, rect.w - 56.0);
        y += 32.0;

        draw_ui_text("Signature Art", x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
        draw_ui_text(
            signature_technique(faction),
            x + 150.0,
            y,
            FONT_BODY_SIZE,
            TEXT_HIGHLIGHT,
        );
        y += 38.0;

        if !faction.description.is_empty() {
            y = draw_wrapped_text(
                &faction.description,
                x,
                y,
                rect.w - 60.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            y += 18.0;
        }

        let relation_label = relation
            .map(|rel| {
                if rel.at_war {
                    "War banner raised".to_string()
                } else if let Some(treaty) = &rel.treaty {
                    format!("Oath tablet: {}", treaty.name())
                } else {
                    format!("Sect standing: {} ({})", rel.reputation, tier)
                }
            })
            .unwrap_or_else(|| "Sect standing: Unknown".to_string());
        draw_ui_text(&relation_label, x, y, FONT_BODY_SIZE, tier_color(tier));
        y += 40.0;

        draw_ui_text(
            &format!(
                "Visible treasury: {} spirit stones | Regional power: {}",
                faction.wealth, faction.power_level
            ),
            x,
            y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        y += 46.0;

        if draw_button(Rect::new(x, y, 150.0, 36.0), "Send Tribute", false) {
            // Hooked later through diplomacy actions.
        }
        if draw_button(Rect::new(x + 164.0, y, 170.0, 36.0), "Request Oath", false) {
            // Hooked later through diplomacy actions.
        }
    }

    pub fn draw(&self, _world_sim: &WorldSim) {
        // Handled in update.
    }
}

fn faction_color(faction_type: &FactionType) -> Color {
    match faction_type {
        FactionType::Sect => SECONDARY,
        FactionType::MerchantGuild => PRIMARY,
        FactionType::DemonCult => ACCENT,
        FactionType::ImperialCourt => WARNING,
        FactionType::BanditClan => FAILURE,
        FactionType::BeastHorde => SUCCESS,
    }
}

fn faction_sigill(faction_type: &FactionType) -> &'static str {
    match faction_type {
        FactionType::Sect => "SECT",
        FactionType::MerchantGuild => "GUILD",
        FactionType::DemonCult => "CULT",
        FactionType::ImperialCourt => "COURT",
        FactionType::BanditClan => "CLAN",
        FactionType::BeastHorde => "HORDE",
    }
}

fn faction_type_title(faction_type: &FactionType) -> &'static str {
    match faction_type {
        FactionType::Sect => "Orthodox Cultivation Sect",
        FactionType::MerchantGuild => "Merchant Cultivator Guild",
        FactionType::DemonCult => "Forbidden Moon Cult",
        FactionType::ImperialCourt => "Imperial Cultivation Court",
        FactionType::BanditClan => "Rogue Mountain Clan",
        FactionType::BeastHorde => "Spirit Beast Horde",
    }
}

fn specialty_label(specialty: &FactionSpecialty) -> &'static str {
    match specialty {
        FactionSpecialty::Trade => "Caravan and auction houses",
        FactionSpecialty::Combat => "Sword and body cultivation",
        FactionSpecialty::Alchemy => "Pill refining",
        FactionSpecialty::Formations => "Arrays and seals",
        FactionSpecialty::Intelligence => "Secrets and shadows",
        FactionSpecialty::Agriculture => "Spirit herb gardens",
    }
}

fn signature_technique(faction: &Faction) -> &'static str {
    match faction.specialty {
        FactionSpecialty::Trade => "Golden Scale Exchange Art",
        FactionSpecialty::Combat => "Crimson Mountain Breaking Fist",
        FactionSpecialty::Alchemy => "Azure Cauldron Returning Breath",
        FactionSpecialty::Formations => "Nine Palace Imperial Seal",
        FactionSpecialty::Intelligence => "Shadow Moon Veil",
        FactionSpecialty::Agriculture => "Jade Lotus Verdant Method",
    }
}

fn leader_title(faction: &Faction) -> String {
    let title = match faction.faction_type {
        FactionType::Sect => "Elder",
        FactionType::MerchantGuild => "Guildmaster",
        FactionType::DemonCult => "Moon Hierarch",
        FactionType::ImperialCourt => "Imperial Envoy",
        FactionType::BanditClan => "Mountain Chief",
        FactionType::BeastHorde => "Beast Sovereign",
    };
    format!("{} {}", title, leader_name_seed(&faction.name))
}

fn leader_name_seed(name: &str) -> &'static str {
    if name.contains("Azure") {
        "Blue Crane"
    } else if name.contains("Golden") {
        "Gold Abacus"
    } else if name.contains("Crimson") {
        "Red Fang"
    } else if name.contains("Shadow") {
        "Silent Moon"
    } else if name.contains("Imperial") {
        "Iron Seal"
    } else if name.contains("Lotus") {
        "Jade Rain"
    } else {
        "Broken Blade"
    }
}

fn realm_label(id: &str) -> String {
    if id.is_empty() {
        return "Unknown".to_string();
    }
    id.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn tier_color(tier: ReputationTier) -> Color {
    let (r, g, b) = tier.color();
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}
