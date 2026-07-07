use crate::data::economy::{EconomyNode, TradeRoute};
use crate::engine::world_sim::WorldSim;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};
use std::collections::HashMap;

pub struct TradeScreenState {
    selected_node: Option<usize>,
    market_scroll: f32,
    goods_scroll: f32,
}

impl TradeScreenState {
    pub fn new() -> Self {
        Self {
            selected_node: None,
            market_scroll: 0.0,
            goods_scroll: 0.0,
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

        draw_mountain_sect_backdrop();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 86.0;
        draw_trade_header(screen_w, header_h, spirit_stones, world_sim);

        let content_y = header_h + 16.0;
        let content_h = screen_h - content_y - 24.0;
        let gutter = 18.0;
        let left_w = (screen_w * 0.30).clamp(320.0, 400.0);
        let right_w = (screen_w * 0.28).clamp(300.0, 360.0);
        let center_w = screen_w - left_w - right_w - gutter * 4.0;

        let markets_rect = Rect::new(gutter, content_y, left_w, content_h);
        let goods_rect = Rect::new(
            markets_rect.x + left_w + gutter,
            content_y,
            center_w,
            content_h,
        );
        let route_rect = Rect::new(
            goods_rect.x + center_w + gutter,
            content_y,
            right_w,
            content_h,
        );

        self.draw_market_ledgers(markets_rect, world_sim);
        self.draw_market_details(goods_rect, world_sim, inventory);
        draw_route_security(route_rect, world_sim);

        if draw_button(
            Rect::new(screen_w - 132.0, screen_h - 62.0, 108.0, 42.0),
            "Return",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    fn draw_market_ledgers(&mut self, rect: Rect, world_sim: &WorldSim) {
        draw_panel(rect, Some("Caravan Tablets"));
        let list_rect = Rect::new(rect.x + 14.0, rect.y + 56.0, rect.w - 28.0, rect.h - 82.0);
        let row_h = 92.0;
        let gap = 10.0;
        let total_h = world_sim.economy.nodes.len() as f32 * (row_h + gap);
        let mouse = vec2(mouse_position().0, mouse_position().1);

        if list_rect.contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if total_h > list_rect.h {
                self.market_scroll = (self.market_scroll - wheel * 42.0)
                    .clamp(0.0, (total_h - list_rect.h).max(0.0));
            } else {
                self.market_scroll = 0.0;
            }
        }

        let mut y = list_rect.y - self.market_scroll;
        for (idx, node) in world_sim.economy.nodes.iter().enumerate() {
            let row = Rect::new(list_rect.x, y, list_rect.w, row_h);
            if row.y + row.h >= list_rect.y && row.y <= list_rect.y + list_rect.h {
                let selected = self.selected_node == Some(idx);
                draw_market_card(row, world_sim, node, selected);
                if row.contains(mouse.into()) && is_mouse_button_pressed(MouseButton::Left) {
                    self.selected_node = Some(idx);
                    self.goods_scroll = 0.0;
                }
            }
            y += row_h + gap;
        }

        if total_h > list_rect.h {
            draw_scrollbar(list_rect, self.market_scroll, total_h);
        }

        draw_ui_text(
            "Choose where the sect's supplies are bargained.",
            rect.x + 18.0,
            rect.y + rect.h - 16.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_market_details(
        &mut self,
        rect: Rect,
        world_sim: &WorldSim,
        inventory: &HashMap<String, u32>,
    ) {
        draw_panel(rect, Some("Caravan Ledger"));

        let Some(idx) = self.selected_node else {
            draw_wrapped_text(
                "Select a caravan tablet. The patriarch needs prices, stock, and route risk before sending supplies away from the mountain.",
                rect.x + 28.0,
                rect.y + 82.0,
                rect.w - 56.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            return;
        };
        let Some(node) = world_sim.economy.nodes.get(idx) else {
            return;
        };

        let x = rect.x + 24.0;
        let mut y = rect.y + 66.0;
        draw_ui_text(&node.name, x, y, FONT_TITLE_SIZE, PRIMARY);
        y += 34.0;
        draw_ui_text(
            &format!(
                "Held by: {}",
                controller_label(world_sim, &node.controller_faction_id)
            ),
            x,
            y,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        y += 34.0;
        draw_ink_divider(x, y, rect.w - 48.0);
        y += 34.0;

        let goods_rect = Rect::new(x, y, rect.w - 48.0, (rect.h * 0.50).max(190.0));
        self.draw_goods_table(goods_rect, world_sim, node);

        let lower_y = goods_rect.y + goods_rect.h + 26.0;
        draw_inventory_and_season(
            Rect::new(x, lower_y, rect.w - 48.0, rect.y + rect.h - lower_y - 18.0),
            world_sim,
            inventory,
        );
    }

    fn draw_goods_table(&mut self, rect: Rect, world_sim: &WorldSim, node: &EconomyNode) {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.035, 0.026, 0.018, 0.46),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.34),
        );
        draw_ui_text(
            "Goods for Rebuilding",
            rect.x + 12.0,
            rect.y + 26.0,
            FONT_BODY_SIZE,
            TEXT_HIGHLIGHT,
        );

        let list_rect = Rect::new(rect.x + 12.0, rect.y + 42.0, rect.w - 24.0, rect.h - 54.0);
        let row_h = 30.0;
        let mut goods: Vec<(&String, &u32)> = node.base_prices.iter().collect();
        goods.sort_by(|a, b| item_name(a.0).cmp(&item_name(b.0)));
        let total_h = goods.len() as f32 * row_h;

        if list_rect.contains(mouse_position().into()) {
            let wheel = mouse_wheel().1;
            if total_h > list_rect.h {
                self.goods_scroll =
                    (self.goods_scroll - wheel * 26.0).clamp(0.0, (total_h - list_rect.h).max(0.0));
            } else {
                self.goods_scroll = 0.0;
            }
        }

        draw_goods_header(list_rect);
        let mut y = list_rect.y + 26.0 - self.goods_scroll;
        for (item_id, base_price) in goods {
            if y + row_h >= list_rect.y + 24.0 && y <= list_rect.y + list_rect.h {
                draw_goods_row(
                    list_rect.x,
                    y,
                    list_rect.w,
                    world_sim,
                    node,
                    item_id,
                    *base_price,
                );
            }
            y += row_h;
        }

        if total_h > list_rect.h - 26.0 {
            draw_scrollbar(
                Rect::new(
                    list_rect.x + list_rect.w - 4.0,
                    list_rect.y + 26.0,
                    4.0,
                    list_rect.h - 26.0,
                ),
                self.goods_scroll,
                total_h,
            );
        }
    }

    pub fn draw(&self, _world_sim: &WorldSim) {
        // Handled in update.
    }
}

fn draw_trade_header(screen_w: f32, header_h: f32, spirit_stones: u32, world_sim: &WorldSim) {
    draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
    draw_screen_title(
        "Caravan Ledger",
        "Supply lines, market omens, and the cost of rebuilding the mountain sect",
        24.0,
        38.0,
    );

    let active_routes = world_sim
        .economy
        .routes
        .iter()
        .filter(|route| route.active)
        .count();
    let disrupted = world_sim
        .economy
        .routes
        .iter()
        .filter(|route| route.is_disrupted())
        .count();
    let mut seal_x = screen_w - 482.0;
    seal_x += draw_resource_seal(seal_x, 56.0, "Stones", spirit_stones, PRIMARY) + 8.0;
    seal_x += draw_resource_seal(
        seal_x,
        56.0,
        "Posts",
        world_sim.economy.nodes.len() as u32,
        SECONDARY,
    ) + 8.0;
    seal_x += draw_resource_seal(seal_x, 56.0, "Routes", active_routes as u32, WARNING) + 8.0;
    draw_resource_seal(seal_x, 56.0, "Unsafe", disrupted as u32, FAILURE);
}

fn draw_market_card(rect: Rect, world_sim: &WorldSim, node: &EconomyNode, selected: bool) {
    let hovered = rect.contains(mouse_position().into());
    let safety = average_route_safety(world_sim, &node.id);
    let color = safety_color(safety);
    let alpha = if selected {
        0.66
    } else if hovered {
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
            color.r,
            color.g,
            color.b,
            if selected { 0.84 } else { 0.46 },
        ),
    );
    draw_circle(
        rect.x + 24.0,
        rect.y + 30.0,
        9.0,
        Color::new(color.r, color.g, color.b, 0.50),
    );
    draw_ui_text(
        &node.name,
        rect.x + 44.0,
        rect.y + 28.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &controller_label(world_sim, &node.controller_faction_id),
        rect.x + 44.0,
        rect.y + 52.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_ui_text(
        &format!(
            "Goods {} | Road omen {}",
            node.base_prices.len(),
            safety_label(safety)
        ),
        rect.x + 44.0,
        rect.y + 76.0,
        FONT_SMALL_SIZE,
        color,
    );
}

fn draw_goods_header(rect: Rect) {
    draw_ui_text(
        "Goods",
        rect.x,
        rect.y + 16.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_ui_text(
        "Stones",
        rect.x + rect.w * 0.44,
        rect.y + 16.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_ui_text(
        "Supply",
        rect.x + rect.w * 0.64,
        rect.y + 16.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_ui_text(
        "Demand",
        rect.x + rect.w * 0.80,
        rect.y + 16.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_line(
        rect.x,
        rect.y + 24.0,
        rect.x + rect.w,
        rect.y + 24.0,
        1.0,
        PANEL_BORDER,
    );
}

fn draw_goods_row(
    x: f32,
    y: f32,
    width: f32,
    world_sim: &WorldSim,
    node: &EconomyNode,
    item_id: &str,
    base_price: u32,
) {
    let supply = node.supply.get(item_id).copied().unwrap_or(0);
    let demand = node.demand.get(item_id).copied().unwrap_or(1.0);
    let price = world_sim
        .get_item_price(&node.id, item_id)
        .unwrap_or(base_price);
    let supply_color = if supply > 100 {
        SUCCESS
    } else if supply > 30 {
        WARNING
    } else {
        FAILURE
    };
    let demand_color = if demand > 1.2 {
        FAILURE
    } else if demand < 0.9 {
        SUCCESS
    } else {
        TEXT_SECONDARY
    };

    draw_ui_text(
        &item_name(item_id),
        x,
        y + 20.0,
        FONT_SMALL_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!("{} SS", price),
        x + width * 0.44,
        y + 20.0,
        FONT_SMALL_SIZE,
        WARNING,
    );
    draw_ui_text(
        &supply.to_string(),
        x + width * 0.64,
        y + 20.0,
        FONT_SMALL_SIZE,
        supply_color,
    );
    draw_ui_text(
        &format!("{:.1}x", demand),
        x + width * 0.80,
        y + 20.0,
        FONT_SMALL_SIZE,
        demand_color,
    );
}

fn draw_inventory_and_season(rect: Rect, world_sim: &WorldSim, inventory: &HashMap<String, u32>) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.026, 0.018, 0.40),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.30),
    );
    let x = rect.x + 14.0;
    let mut y = rect.y + 28.0;
    draw_ui_text("Sect Stores", x, y, FONT_BODY_SIZE, TEXT_HIGHLIGHT);
    y += 28.0;

    let mut shown = 0;
    for (item_id, count) in inventory.iter().filter(|(_, count)| **count > 0).take(4) {
        draw_ui_text(
            &format!("{} x{}", item_name(item_id), count),
            x,
            y,
            FONT_SMALL_SIZE,
            TEXT_PRIMARY,
        );
        y += 20.0;
        shown += 1;
    }
    if shown == 0 {
        draw_ui_text(
            "No stored trade goods.",
            x,
            y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        y += 20.0;
    }

    y += 14.0;
    draw_ink_divider(x, y, rect.w - 28.0);
    y += 26.0;
    let trade_activity = world_sim.seasonal_modifiers.trade_activity_mod;
    let color = if trade_activity >= 1.0 {
        SUCCESS
    } else {
        FAILURE
    };
    draw_ui_text(
        &format!("Seasonal caravan activity: {:.0}%", trade_activity * 100.0),
        x,
        y,
        FONT_SMALL_SIZE,
        color,
    );
}

fn draw_route_security(rect: Rect, world_sim: &WorldSim) {
    draw_panel(rect, Some("Route Omens"));
    let x = rect.x + 22.0;
    let mut y = rect.y + 64.0;
    draw_wrapped_text(
        "Every supply line is another way disciples can return with medicine, ore, and debt. Unsafe roads bleed the sect before battle begins.",
        x,
        y,
        rect.w - 44.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    y += 118.0;
    draw_ink_divider(x, y, rect.w - 44.0);
    y += 34.0;

    for route in world_sim.economy.routes.iter().take(6) {
        let row = Rect::new(x, y - 18.0, rect.w - 44.0, 64.0);
        draw_route_row(row, world_sim, route);
        y += 76.0;
        if y > rect.y + rect.h - 74.0 {
            break;
        }
    }
}

fn draw_route_row(rect: Rect, world_sim: &WorldSim, route: &TradeRoute) {
    let color = safety_color(route.safety_rating);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.026, 0.018, 0.44),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(color.r, color.g, color.b, 0.46),
    );
    draw_ui_text(
        &format!(
            "{} -> {}",
            node_name(world_sim, &route.from_node),
            node_name(world_sim, &route.to_node)
        ),
        rect.x + 12.0,
        rect.y + 24.0,
        FONT_SMALL_SIZE,
        TEXT_PRIMARY,
    );
    let status = if route.active {
        safety_label(route.safety_rating)
    } else {
        "Sealed"
    };
    draw_ui_text(
        &format!(
            "{} | {}",
            status,
            crate::data::time::days_label(route.travel_ticks)
        ),
        rect.x + 12.0,
        rect.y + 48.0,
        FONT_SMALL_SIZE,
        color,
    );
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

fn controller_label(world_sim: &WorldSim, controller_id: &str) -> String {
    if controller_id.is_empty() {
        "Independent caravan court".to_string()
    } else {
        world_sim
            .get_faction(controller_id)
            .map(|faction| faction.name.clone())
            .unwrap_or_else(|| controller_id.replace('_', " "))
    }
}

fn average_route_safety(world_sim: &WorldSim, node_id: &str) -> f32 {
    let mut total = 0.0;
    let mut count = 0.0;
    for route in &world_sim.economy.routes {
        if route.from_node == node_id || route.to_node == node_id {
            total += if route.active {
                route.safety_rating
            } else {
                0.0
            };
            count += 1.0;
        }
    }
    if count == 0.0 {
        1.0
    } else {
        total / count
    }
}

fn safety_color(safety: f32) -> Color {
    if safety >= 0.75 {
        SUCCESS
    } else if safety >= 0.45 {
        WARNING
    } else {
        FAILURE
    }
}

fn safety_label(safety: f32) -> &'static str {
    if safety >= 0.75 {
        "Auspicious"
    } else if safety >= 0.45 {
        "Risky"
    } else {
        "Disrupted"
    }
}

fn node_name(world_sim: &WorldSim, node_id: &str) -> String {
    world_sim
        .economy
        .get_node(node_id)
        .map(|node| node.name.clone())
        .unwrap_or_else(|| node_id.replace('_', " "))
}

fn item_name(item_id: &str) -> String {
    item_id
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
