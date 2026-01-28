use crate::data::relations::ReputationTier;
use crate::engine::world_sim::WorldSim;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

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

        // Header
        draw_panel(Rect::new(0.0, 0.0, screen_w, 60.0), None);
        draw_text("FACTIONS & DIPLOMACY", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        draw_text(
            "Press ESC to return",
            screen_w - 200.0,
            40.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );

        // Two-column layout: Faction list on left, details on right
        let list_width = 350.0;
        let detail_x = list_width + 20.0;
        let content_y = 80.0;
        let content_height = screen_h - 100.0;

        // Faction List Panel
        draw_panel(
            Rect::new(10.0, content_y, list_width, content_height),
            Some("Known Factions"),
        );

        let mut y = content_y + 50.0;
        for (i, faction) in world_sim.factions.iter().enumerate() {
            let relation = world_sim.get_relation(&faction.id);
            let reputation = relation.map(|r| r.reputation).unwrap_or(0);
            let tier = relation
                .map(|r| r.reputation_tier())
                .unwrap_or(ReputationTier::Neutral);

            let item_rect = Rect::new(20.0, y, list_width - 20.0, 60.0);
            let is_selected = self.selected_faction == Some(i);
            let is_hovered = item_rect.contains(mouse_position().into());

            // Background for item
            let bg_color = if is_selected {
                PANEL_DARK
            } else if is_hovered {
                Color::new(0.2, 0.22, 0.25, 1.0)
            } else {
                PANEL
            };
            draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, bg_color);

            // Faction name
            draw_text(
                &faction.name,
                item_rect.x + 10.0,
                item_rect.y + 22.0,
                FONT_HEADER_SIZE,
                TEXT_PRIMARY,
            );

            // Faction type and reputation
            let type_text = format!("{}", faction.faction_type);
            draw_text(
                &type_text,
                item_rect.x + 10.0,
                item_rect.y + 42.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );

            // Reputation bar
            let bar_x = item_rect.x + 180.0;
            let bar_y = item_rect.y + 35.0;
            let bar_width = 100.0;
            let bar_height = 12.0;

            // Background
            draw_rectangle(bar_x, bar_y, bar_width, bar_height, PANEL_DARK);

            // Fill based on reputation (-100 to +100 mapped to 0-100%)
            let fill_percent = ((reputation + 100) as f32 / 200.0).clamp(0.0, 1.0);
            let fill_color = if reputation > 25 {
                SUCCESS
            } else if reputation < -25 {
                FAILURE
            } else {
                WARNING
            };
            draw_rectangle(bar_x, bar_y, bar_width * fill_percent, bar_height, fill_color);

            // Reputation text
            draw_text(
                &format!("{}", tier),
                bar_x + bar_width + 5.0,
                bar_y + 10.0,
                FONT_SMALL_SIZE,
                fill_color,
            );

            // Handle click
            if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_faction = Some(i);
            }

            y += 65.0;
        }

        // Detail Panel
        let detail_width = screen_w - detail_x - 20.0;
        draw_panel(
            Rect::new(detail_x, content_y, detail_width, content_height),
            Some("Faction Details"),
        );

        if let Some(idx) = self.selected_faction {
            if let Some(faction) = world_sim.factions.get(idx) {
                let relation = world_sim.get_relation(&faction.id);

                let mut dy = content_y + 60.0;
                let dx = detail_x + 20.0;

                // Faction name and type
                draw_text(&faction.name, dx, dy, FONT_TITLE_SIZE, PRIMARY);
                dy += 30.0;

                draw_text(
                    &format!("Type: {}", faction.faction_type),
                    dx,
                    dy,
                    FONT_BODY_SIZE,
                    TEXT_SECONDARY,
                );
                dy += 25.0;

                draw_text(
                    &format!("Specialty: {}", faction.specialty),
                    dx,
                    dy,
                    FONT_BODY_SIZE,
                    TEXT_SECONDARY,
                );
                dy += 25.0;

                draw_text(
                    &format!("Power Level: {}", faction.power_level),
                    dx,
                    dy,
                    FONT_BODY_SIZE,
                    TEXT_SECONDARY,
                );
                dy += 25.0;

                draw_text(
                    &format!("Wealth: {} SS", faction.wealth),
                    dx,
                    dy,
                    FONT_BODY_SIZE,
                    TEXT_SECONDARY,
                );
                dy += 35.0;

                // Description
                if !faction.description.is_empty() {
                    draw_text("Description:", dx, dy, FONT_BODY_SIZE, TEXT_PRIMARY);
                    dy += 20.0;

                    // Word wrap the description
                    let max_width = detail_width - 40.0;
                    let words: Vec<&str> = faction.description.split_whitespace().collect();
                    let mut line = String::new();
                    for word in words {
                        let test_line = if line.is_empty() {
                            word.to_string()
                        } else {
                            format!("{} {}", line, word)
                        };
                        let text_width = test_line.len() as f32 * 7.0; // Approximate
                        if text_width > max_width && !line.is_empty() {
                            draw_text(&line, dx, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                            dy += 18.0;
                            line = word.to_string();
                        } else {
                            line = test_line;
                        }
                    }
                    if !line.is_empty() {
                        draw_text(&line, dx, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                        dy += 25.0;
                    }
                }
                dy += 10.0;

                // Relation info
                if let Some(rel) = relation {
                    draw_text("--- Relations ---", dx, dy, FONT_BODY_SIZE, ACCENT);
                    dy += 25.0;

                    let tier = rel.reputation_tier();
                    let (r, g, b) = tier.color();
                    let tier_color = Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);

                    draw_text(
                        &format!("Reputation: {} ({})", rel.reputation, tier),
                        dx,
                        dy,
                        FONT_BODY_SIZE,
                        tier_color,
                    );
                    dy += 25.0;

                    if rel.at_war {
                        draw_text("STATUS: AT WAR", dx, dy, FONT_HEADER_SIZE, FAILURE);
                        dy += 25.0;
                    }

                    if let Some(treaty) = &rel.treaty {
                        draw_text(
                            &format!("Active Treaty: {}", treaty.name()),
                            dx,
                            dy,
                            FONT_BODY_SIZE,
                            SUCCESS,
                        );
                        dy += 25.0;
                    }

                    draw_text(
                        &format!(
                            "Friendly Actions: {} | Hostile Actions: {}",
                            rel.friendly_actions, rel.hostile_actions
                        ),
                        dx,
                        dy,
                        FONT_SMALL_SIZE,
                        TEXT_SECONDARY,
                    );
                    dy += 35.0;

                    // Diplomatic action buttons (placeholder - would need Action system)
                    draw_text("--- Diplomatic Actions ---", dx, dy, FONT_BODY_SIZE, ACCENT);
                    dy += 30.0;

                    let btn_width = 150.0;
                    let btn_height = 35.0;

                    if draw_button(
                        Rect::new(dx, dy, btn_width, btn_height),
                        "Send Gift",
                        false,
                    ) {
                        // Would trigger Action::SendDiplomat
                    }

                    if draw_button(
                        Rect::new(dx + btn_width + 10.0, dy, btn_width, btn_height),
                        "Request Treaty",
                        rel.treaty.is_some(),
                    ) {
                        // Would trigger treaty request
                    }
                }
            }
        } else {
            draw_text(
                "Select a faction to view details",
                detail_x + 20.0,
                content_y + 100.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        // Back button
        if draw_button(
            Rect::new(20.0, screen_h - 60.0, 100.0, 40.0),
            "Back",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _world_sim: &WorldSim) {
        // Handled in update
    }
}
