use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct DiscipleRosterState {
    selected_index: Option<usize>,
}

impl DiscipleRosterState {
    pub fn new() -> Self {
        Self {
            selected_index: None,
        }
    }

    pub fn update(&mut self, disciples: &[Disciple]) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;

        // --- Header ---
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("DISCIPLE ROSTER", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        draw_text(&format!("Total Disciples: {}", disciples.len()), screen_w - 250.0, 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        // --- Layout ---
        let content_y = header_h + 10.0;
        let content_h = screen_h - content_y - 10.0;
        let left_w = 300.0;
        let right_w = screen_w - left_w - 30.0;

        // --- Left Panel: Disciple List ---
        let left_rect = Rect::new(10.0, content_y, left_w, content_h);
        draw_panel(left_rect, Some("Disciples"));

        let mut btn_y = left_rect.y + 50.0;
        for (i, disciple) in disciples.iter().enumerate() {
            let active = Some(i) == self.selected_index;
            let label = format!("{} ({:?})", disciple.name, disciple.realm);
            
            if draw_button(Rect::new(left_rect.x + 10.0, btn_y, left_w - 20.0, 40.0), &label, active) {
                self.selected_index = Some(i);
            }
            btn_y += 50.0;
        }

        // --- Right Panel: Details ---
        let right_rect = Rect::new(left_w + 20.0, content_y, right_w, content_h);
        draw_panel(right_rect, Some(if self.selected_index.is_some() { "Disciple Attributes" } else { "Select a Disciple" }));

        if let Some(idx) = self.selected_index {
            if let Some(disciple) = disciples.get(idx) {
                let x = right_rect.x + 20.0;
                let mut y = right_rect.y + 60.0;

                draw_text(&disciple.name, x, y, FONT_TITLE_SIZE, PRIMARY);
                y += 40.0;
                
                draw_text(&format!("Realm: {:?}", disciple.realm), x, y, FONT_HEADER_SIZE, TEXT_PRIMARY);
                y += 30.0;
                draw_text(&format!("Talent: {:?}", disciple.talent), x, y, FONT_BODY_SIZE, TEXT_SECONDARY);
                y += 30.0;

                // Stats
                draw_text("Attributes:", x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
                y += 30.0;
                
                let stats = [
                    ("Body", disciple.attributes.body, "Physical prowess & combat."),
                    ("Mind", disciple.attributes.mind, "Learning & diplomacy."),
                    ("Spirit", disciple.attributes.spirit, "Qi gathering & magic."),
                ];

                for (name, val, desc) in stats {
                    let text = format!("{}: {}", name, val);
                    draw_text(&text, x + 20.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
                    
                    // Simple hover check
                    let dims = measure_text(&text, None, FONT_BODY_SIZE as u16, 1.0);
                    let rect = Rect::new(x + 20.0, y - dims.height, dims.width, dims.height);
                    if rect.contains(mouse_position().into()) {
                        draw_tooltip(mouse_position().into(), desc);
                    }
                    y += 25.0;
                }
                y += 10.0;

                // Traits
                draw_text("Fate Traits:", x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
                y += 30.0;
                for trait_ in &disciple.fate_traits {
                    draw_text(&format!("- {}: {}", trait_.name, trait_.description), x + 20.0, y, FONT_BODY_SIZE, TEXT_SECONDARY);
                    y += 25.0;
                }

                // Experience Bar
                y += 20.0;
                draw_text("Cultivation Progress:", x, y, FONT_BODY_SIZE, TEXT_PRIMARY);
                y += 10.0;
                let progress = disciple.exp as f32 / disciple.exp_to_next_level as f32;
                draw_progress_bar(Rect::new(x, y, 400.0, 20.0), progress, SECONDARY);
            }
        }

        // Back Button
        if draw_button(Rect::new(screen_w - 120.0, screen_h - 50.0, 100.0, 40.0), "Back", false) {
             return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _disciples: &[Disciple], _spirit_stones: u32) {
        // Handled in update
    }
}
