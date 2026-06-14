use crate::data::factions::{Faction, FactionType};
use crate::data::loader::GameData;
use crate::data::missions::MapNode;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub struct WorldMapState;

impl WorldMapState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, data: &GameData) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_panel(Rect::new(0.0, 0.0, screen_w, 70.0), None);
        draw_screen_title(
            "Ancient Regional Map",
            "Routes, threats, and powers beyond the mountain gate",
            24.0,
            34.0,
        );

        let map_rect = Rect::new(28.0, 90.0, screen_w - 56.0, screen_h - 158.0);
        self.draw_map_ground(map_rect);

        let player_pos = vec2(
            map_rect.x + map_rect.w * 0.47,
            map_rect.y + map_rect.h * 0.60,
        );
        for node in &data.map_nodes {
            let node_pos = map_node_pos(map_rect, node);
            draw_route(
                player_pos,
                node_pos,
                Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.32),
            );
        }

        for (idx, faction) in data.factions.iter().enumerate() {
            let pos = faction_anchor(map_rect, idx);
            draw_route(
                player_pos,
                pos,
                Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.16),
            );
            self.draw_faction_marker(pos, faction);
        }

        self.draw_player_sect(player_pos);
        for node in &data.map_nodes {
            self.draw_region_node(map_node_pos(map_rect, node), node);
        }

        draw_panel(
            Rect::new(28.0, screen_h - 58.0, screen_w - 176.0, 44.0),
            None,
        );
        draw_ui_text(
            "Every road is a wager: herbs for survival, ruins for legacy, enemies for prestige.",
            44.0,
            screen_h - 30.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );

        if draw_button(
            Rect::new(screen_w - 128.0, screen_h - 58.0, 100.0, 40.0),
            "Back",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    fn draw_map_ground(&self, rect: Rect) {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.66, 0.56, 0.39, 0.92),
        );
        draw_rectangle(
            rect.x + 8.0,
            rect.y + 8.0,
            rect.w - 16.0,
            rect.h - 16.0,
            Color::new(0.78, 0.69, 0.49, 0.70),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, PANEL_BORDER);
        draw_rectangle_lines(
            rect.x + 8.0,
            rect.y + 8.0,
            rect.w - 16.0,
            rect.h - 16.0,
            1.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.42),
        );

        self.draw_coastline(rect);
        self.draw_rivers(rect);
        self.draw_mountain_glyphs(rect);
        self.draw_fog_bank(rect);

        for i in 0..7 {
            let y = rect.y + rect.h * (0.18 + i as f32 * 0.09);
            draw_line(
                rect.x + rect.w * 0.12,
                y,
                rect.x + rect.w * 0.88,
                y + (i as f32 * 7.0).sin() * 10.0,
                1.0,
                Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.08),
            );
        }

        draw_ui_text(
            "Regional Ink Map",
            rect.x + 22.0,
            rect.y + 34.0,
            FONT_HEADER_SIZE,
            Color::new(0.20, 0.12, 0.07, 0.74),
        );
    }

    fn draw_coastline(&self, rect: Rect) {
        let coast_x = rect.x + rect.w * 0.83;
        draw_rectangle(
            coast_x,
            rect.y + 10.0,
            rect.w * 0.15,
            rect.h - 20.0,
            Color::new(0.14, 0.42, 0.50, 0.22),
        );
        for i in 0..12 {
            let y = rect.y + 24.0 + i as f32 * rect.h / 12.0;
            draw_line(
                coast_x + (i as f32 * 1.7).sin() * 12.0,
                y,
                coast_x + 36.0 + (i as f32 * 2.3).cos() * 18.0,
                y + 18.0,
                2.0,
                Color::new(0.08, 0.22, 0.24, 0.18),
            );
        }
    }

    fn draw_rivers(&self, rect: Rect) {
        let rivers = [
            [
                (0.18, 0.18),
                (0.28, 0.32),
                (0.38, 0.45),
                (0.58, 0.52),
                (0.82, 0.58),
            ],
            [
                (0.10, 0.72),
                (0.26, 0.64),
                (0.42, 0.68),
                (0.60, 0.76),
                (0.84, 0.70),
            ],
        ];
        for river in rivers {
            for pair in river.windows(2) {
                let a = vec2(rect.x + rect.w * pair[0].0, rect.y + rect.h * pair[0].1);
                let b = vec2(rect.x + rect.w * pair[1].0, rect.y + rect.h * pair[1].1);
                draw_line(a.x, a.y, b.x, b.y, 9.0, Color::new(0.28, 0.60, 0.64, 0.18));
                draw_line(a.x, a.y, b.x, b.y, 3.0, Color::new(0.13, 0.30, 0.34, 0.28));
            }
        }
    }

    fn draw_mountain_glyphs(&self, rect: Rect) {
        let clusters = [
            (0.16, 0.28, 5),
            (0.34, 0.24, 4),
            (0.48, 0.36, 6),
            (0.24, 0.56, 4),
            (0.66, 0.42, 5),
            (0.58, 0.68, 5),
            (0.74, 0.24, 3),
        ];
        for (x, y, count) in clusters {
            for i in 0..count {
                let px = rect.x + rect.w * x + i as f32 * 14.0;
                let py = rect.y + rect.h * y + ((i % 2) as f32 * 12.0);
                draw_triangle(
                    vec2(px - 13.0, py + 16.0),
                    vec2(px, py - 12.0),
                    vec2(px + 15.0, py + 16.0),
                    Color::new(0.20, 0.14, 0.08, 0.26),
                );
                draw_line(
                    px,
                    py - 10.0,
                    px - 5.0,
                    py + 12.0,
                    1.0,
                    Color::new(0.88, 0.80, 0.62, 0.18),
                );
            }
        }
    }

    fn draw_fog_bank(&self, rect: Rect) {
        let fog = [
            (0.12, 0.14, 38.0),
            (0.30, 0.82, 46.0),
            (0.52, 0.18, 34.0),
            (0.74, 0.78, 54.0),
            (0.88, 0.34, 44.0),
        ];
        for (x, y, r) in fog {
            draw_circle(
                rect.x + rect.w * x,
                rect.y + rect.h * y,
                r,
                Color::new(0.88, 0.84, 0.72, 0.16),
            );
            draw_circle(
                rect.x + rect.w * x + r * 0.72,
                rect.y + rect.h * y + 8.0,
                r * 0.74,
                Color::new(0.88, 0.84, 0.72, 0.10),
            );
        }
    }

    fn draw_player_sect(&self, pos: Vec2) {
        draw_circle(
            pos.x,
            pos.y,
            30.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.26),
        );
        draw_circle_lines(pos.x, pos.y, 36.0, 2.0, PRIMARY);
        draw_circle(pos.x, pos.y, 10.0, PRIMARY);
        draw_ui_text(
            "Your Sect",
            pos.x - 44.0,
            pos.y + 58.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );
        draw_ui_text(
            "Fallen peak",
            pos.x - 30.0,
            pos.y + 78.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_region_node(&self, pos: Vec2, node: &MapNode) {
        let total_danger = node.danger_level + (node.corruption / 10);
        let danger_color = if total_danger >= 6 {
            FAILURE
        } else if total_danger >= 3 {
            WARNING
        } else {
            SECONDARY
        };
        draw_location_banner(pos, &node.name, danger_color);
        draw_circle(pos.x, pos.y + 30.0, 7.0, danger_color);
        draw_ui_text(
            &format!("Threat {} | Corruption {}", total_danger, node.corruption),
            pos.x + 26.0,
            pos.y + 42.0,
            FONT_SMALL_SIZE,
            danger_color,
        );

        let hit = Rect::new(pos.x - 24.0, pos.y - 24.0, 48.0, 48.0);
        if hit.contains(mouse_position().into()) {
            draw_circle_lines(pos.x, pos.y, 32.0, 2.0, TEXT_HIGHLIGHT);
            draw_tooltip(
                mouse_position().into(),
                &format!("{}: threat {}", node.name, total_danger),
            );
        }
    }

    fn draw_faction_marker(&self, pos: Vec2, faction: &Faction) {
        let color = faction_color(&faction.faction_type);
        draw_sect_seal(pos, color);
        draw_ui_text(
            &short_name(&faction.name),
            pos.x - 44.0,
            pos.y + 33.0,
            FONT_SMALL_SIZE,
            Color::new(0.18, 0.10, 0.06, 0.90),
        );

        let hit = Rect::new(pos.x - 52.0, pos.y - 28.0, 104.0, 70.0);
        if hit.contains(mouse_position().into()) {
            draw_rectangle_lines(pos.x - 56.0, pos.y - 32.0, 112.0, 76.0, 2.0, PRIMARY);
            draw_tooltip(
                mouse_position().into(),
                &format!(
                    "{}\nRealm: {}",
                    faction.name,
                    realm_label(&faction.leader_realm)
                ),
            );
        }
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        // Handled in update.
    }
}

fn map_node_pos(rect: Rect, node: &MapNode) -> Vec2 {
    let x = (node.x / 500.0).clamp(0.10, 0.90);
    let y = (node.y / 500.0).clamp(0.12, 0.86);
    vec2(rect.x + rect.w * x, rect.y + rect.h * y)
}

fn faction_anchor(rect: Rect, idx: usize) -> Vec2 {
    const ANCHORS: &[(f32, f32)] = &[
        (0.72, 0.18),
        (0.22, 0.22),
        (0.78, 0.72),
        (0.52, 0.13),
        (0.88, 0.42),
        (0.15, 0.66),
        (0.42, 0.82),
    ];
    let (x, y) = ANCHORS[idx % ANCHORS.len()];
    vec2(rect.x + rect.w * x, rect.y + rect.h * y)
}

fn draw_route(a: Vec2, b: Vec2, color: Color) {
    draw_line(
        a.x,
        a.y,
        b.x,
        b.y,
        5.0,
        Color::new(color.r, color.g, color.b, color.a * 0.28),
    );
    draw_line(a.x, a.y, b.x, b.y, 2.0, color);
    let mid = (a + b) * 0.5;
    draw_circle(
        mid.x,
        mid.y,
        2.5,
        Color::new(color.r, color.g, color.b, color.a + 0.12),
    );
}

fn draw_location_banner(pos: Vec2, name: &str, color: Color) {
    let w = 110.0;
    let h = 34.0;
    draw_rectangle(
        pos.x - w * 0.5,
        pos.y - h * 0.5,
        w,
        h,
        Color::new(0.18, 0.10, 0.06, 0.78),
    );
    draw_triangle(
        vec2(pos.x - w * 0.5, pos.y + h * 0.5),
        vec2(pos.x - w * 0.5 + 12.0, pos.y + h * 0.5),
        vec2(pos.x - w * 0.5, pos.y + h * 0.5 + 10.0),
        Color::new(0.18, 0.10, 0.06, 0.78),
    );
    draw_rectangle_lines(
        pos.x - w * 0.5,
        pos.y - h * 0.5,
        w,
        h,
        1.0,
        Color::new(color.r, color.g, color.b, 0.82),
    );
    draw_ui_text(
        name,
        pos.x - w * 0.5 + 10.0,
        pos.y + 6.0,
        FONT_SMALL_SIZE,
        TEXT_PRIMARY,
    );
}

fn draw_sect_seal(pos: Vec2, color: Color) {
    draw_circle(
        pos.x,
        pos.y,
        22.0,
        Color::new(color.r, color.g, color.b, 0.34),
    );
    draw_circle_lines(pos.x, pos.y, 25.0, 2.0, Color::new(0.18, 0.10, 0.06, 0.76));
    draw_circle_lines(
        pos.x,
        pos.y,
        16.0,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.62),
    );
    draw_line(pos.x - 13.0, pos.y, pos.x + 13.0, pos.y, 2.0, color);
    draw_line(pos.x, pos.y - 13.0, pos.x, pos.y + 13.0, 2.0, color);
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

fn short_name(name: &str) -> String {
    name.split_whitespace()
        .take(2)
        .collect::<Vec<_>>()
        .join(" ")
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
