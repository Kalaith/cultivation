use crate::engine::world_sim::WorldSim;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct TradeScreenState {
    selected_node: Option<usize>,
    scroll_offset: f32,
}

impl TradeScreenState {
    pub fn new() -> Self {
        Self {
            selected_node: None,
            scroll_offset: 0.0,
        }
    }

    pub fn update(
        &mut self,
        world_sim: &WorldSim,
        spirit_stones: u32,
        inventory: &HashMap<String, u32>,
    ) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Header
        draw_panel(Rect::new(0.0, 0.0, screen_w, 60.0), None);
        draw_text("TRADE & ECONOMY", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        draw_text(
            &format!("Spirit Stones: {}", spirit_stones),
            screen_w - 250.0,
            40.0,
            FONT_BODY_SIZE,
            WARNING,
        );

        // Two-column layout
        let list_width = 300.0;
        let detail_x = list_width + 20.0;
        let content_y = 80.0;
        let content_height = screen_h - 100.0;

        // Market nodes list
        draw_panel(
            Rect::new(10.0, content_y, list_width, content_height),
            Some("Markets"),
        );

        let mut y = content_y + 50.0;
        for (i, node) in world_sim.economy.nodes.iter().enumerate() {
            let item_rect = Rect::new(20.0, y, list_width - 20.0, 50.0);
            let is_selected = self.selected_node == Some(i);
            let is_hovered = item_rect.contains(mouse_position().into());

            let bg_color = if is_selected {
                PANEL_DARK
            } else if is_hovered {
                Color::new(0.2, 0.22, 0.25, 1.0)
            } else {
                PANEL
            };
            draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, bg_color);

            draw_text(
                &node.name,
                item_rect.x + 10.0,
                item_rect.y + 22.0,
                FONT_HEADER_SIZE,
                TEXT_PRIMARY,
            );

            // Controller info
            let controller = if node.controller_faction_id.is_empty() {
                "Independent".to_string()
            } else {
                world_sim
                    .get_faction(&node.controller_faction_id)
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| node.controller_faction_id.clone())
            };
            draw_text(
                &format!("Controlled by: {}", controller),
                item_rect.x + 10.0,
                item_rect.y + 40.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );

            if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_node = Some(i);
            }

            y += 55.0;
        }

        // Trade Routes section
        y += 20.0;
        draw_text("Trade Routes", 20.0, y, FONT_HEADER_SIZE, ACCENT);
        y += 25.0;

        for route in &world_sim.economy.routes {
            let safety_color = if route.safety_rating > 0.7 {
                SUCCESS
            } else if route.safety_rating > 0.4 {
                WARNING
            } else {
                FAILURE
            };

            let status = if route.active { "Active" } else { "Inactive" };
            draw_text(
                &format!(
                    "{} <-> {} [{}] Safety: {:.0}%",
                    route.from_node,
                    route.to_node,
                    status,
                    route.safety_rating * 100.0
                ),
                20.0,
                y,
                FONT_SMALL_SIZE,
                safety_color,
            );
            y += 18.0;
        }

        // Detail Panel - Market prices
        let detail_width = screen_w - detail_x - 20.0;
        draw_panel(
            Rect::new(detail_x, content_y, detail_width, content_height),
            Some("Market Prices"),
        );

        if let Some(idx) = self.selected_node {
            if let Some(node) = world_sim.economy.nodes.get(idx) {
                let mut dy = content_y + 60.0;
                let dx = detail_x + 20.0;

                draw_text(&node.name, dx, dy, FONT_TITLE_SIZE, PRIMARY);
                dy += 40.0;

                // Show prices
                draw_text("Available Goods:", dx, dy, FONT_BODY_SIZE, TEXT_PRIMARY);
                dy += 25.0;

                // Header
                let col1 = dx;
                let col2 = dx + 150.0;
                let col3 = dx + 250.0;
                let col4 = dx + 350.0;

                draw_text("Item", col1, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                draw_text("Base Price", col2, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                draw_text("Supply", col3, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                draw_text("Demand", col4, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                dy += 20.0;

                // Line separator
                draw_line(dx, dy, dx + 400.0, dy, 1.0, BORDER);
                dy += 10.0;

                for (item_id, base_price) in &node.base_prices {
                    let supply = node.supply.get(item_id).copied().unwrap_or(0);
                    let demand = node.demand.get(item_id).copied().unwrap_or(1.0);

                    // Calculate current price
                    let price = world_sim
                        .get_item_price(&node.id, item_id)
                        .unwrap_or(*base_price);

                    // Format item name
                    let item_name = item_id.replace('_', " ");
                    let item_name = item_name
                        .split_whitespace()
                        .map(|word| {
                            let mut chars: Vec<char> = word.chars().collect();
                            if let Some(first) = chars.first_mut() {
                                *first = first.to_ascii_uppercase();
                            }
                            chars.into_iter().collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join(" ");

                    draw_text(&item_name, col1, dy, FONT_SMALL_SIZE, TEXT_PRIMARY);
                    draw_text(&format!("{} SS", price), col2, dy, FONT_SMALL_SIZE, WARNING);

                    let supply_color = if supply > 100 {
                        SUCCESS
                    } else if supply > 30 {
                        WARNING
                    } else {
                        FAILURE
                    };
                    draw_text(&format!("{}", supply), col3, dy, FONT_SMALL_SIZE, supply_color);

                    let demand_color = if demand > 1.2 {
                        FAILURE
                    } else if demand < 0.9 {
                        SUCCESS
                    } else {
                        TEXT_SECONDARY
                    };
                    draw_text(&format!("{:.1}x", demand), col4, dy, FONT_SMALL_SIZE, demand_color);

                    dy += 20.0;
                }

                dy += 20.0;

                // Your inventory section
                draw_text("Your Inventory:", dx, dy, FONT_BODY_SIZE, ACCENT);
                dy += 25.0;

                if inventory.is_empty() {
                    draw_text("No items in inventory", dx, dy, FONT_SMALL_SIZE, TEXT_SECONDARY);
                } else {
                    for (item_id, count) in inventory {
                        if *count > 0 {
                            let item_name = item_id.replace('_', " ");
                            draw_text(
                                &format!("{}: {}", item_name, count),
                                dx,
                                dy,
                                FONT_SMALL_SIZE,
                                TEXT_PRIMARY,
                            );
                            dy += 18.0;
                        }
                    }
                }

                dy += 30.0;

                // Seasonal info
                let season_mod = &world_sim.seasonal_modifiers;
                draw_text("Seasonal Modifiers:", dx, dy, FONT_BODY_SIZE, ACCENT);
                dy += 25.0;
                draw_text(
                    &format!("Trade Activity: {:.0}%", season_mod.trade_activity_mod * 100.0),
                    dx,
                    dy,
                    FONT_SMALL_SIZE,
                    if season_mod.trade_activity_mod > 1.0 {
                        SUCCESS
                    } else {
                        FAILURE
                    },
                );
            }
        } else {
            draw_text(
                "Select a market to view prices",
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
