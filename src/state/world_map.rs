use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct WorldMapState;

impl WorldMapState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, data: &GameData) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        // --- Draw Map ---
        // For map, we might not use a panel for the whole thing, but maybe for a header/legend
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        // Header
        draw_panel(Rect::new(0.0, 0.0, screen_w, 60.0), None);
        draw_text("WORLD MAP", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        draw_text("Press ESC to return to Sect Base", screen_w - 300.0, 40.0, FONT_BODY_SIZE, TEXT_SECONDARY);

        // Nodes
        for node in &data.map_nodes {
            draw_circle(node.x, node.y, 15.0, ACCENT);
            let total_danger = node.danger_level + (node.corruption / 10);
            
            draw_text(&node.name, node.x + 20.0, node.y, FONT_HEADER_SIZE, TEXT_PRIMARY);
            
            let color = if node.corruption > 20 { FAILURE } else { TEXT_SECONDARY };
            draw_text(
                &format!("Danger: {} (Corr: {})", total_danger, node.corruption),
                node.x + 20.0,
                node.y + 25.0,
                FONT_SMALL_SIZE,
                color,
            );
            
            // Interaction: showing tooltip or click?
            let mouse_pos = mouse_position().into();
            let node_rect = Rect::new(node.x - 15.0, node.y - 15.0, 30.0, 30.0);
            if node_rect.contains(mouse_pos) {
                  draw_circle_lines(node.x, node.y, 20.0, 2.0, PRIMARY);
                  draw_tooltip(mouse_pos, &format!("{} (Danger: {})", node.name, total_danger));
            }
        }
        
        // Back Button (Floating)
        if draw_button(Rect::new(20.0, screen_h - 60.0, 100.0, 40.0), "Back", false) {
             return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        // Handled in update
    }
}
