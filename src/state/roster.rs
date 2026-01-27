use crate::data::disciples::Disciple;
use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct DiscipleRosterState {
    selected_index: Option<usize>,
    law_modal_open: bool,
    item_modal_open: bool,
}

impl DiscipleRosterState {
    pub fn new() -> Self {
        Self {
            selected_index: None,
            law_modal_open: false,
            item_modal_open: false,
        }
    }

    pub fn update(&mut self, data: &GameData, disciples: &[Disciple], inventory: &std::collections::HashMap<String, u32>) -> UpdateResult {
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
            let rank_prefix = match disciple.rank {
                crate::data::disciples::DiscipleRank::Outer => "[Outer]",
                crate::data::disciples::DiscipleRank::Inner => "[Inner]",
                crate::data::disciples::DiscipleRank::Elder => "[Elder]",
                crate::data::disciples::DiscipleRank::SectLeader => "[Leader]",
            };
            // Add injury indicator if injured
            let injury_indicator = if disciple.is_injured() { " [INJURED]" } else { "" };
            let label = format!("{} {}{}", rank_prefix, disciple.name, injury_indicator);

            // Color injured disciples differently
            let btn_rect = Rect::new(left_rect.x + 10.0, btn_y, left_w - 20.0, 40.0);
            if disciple.is_injured() {
                // Draw injury tint behind button
                draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, Color::new(0.5, 0.2, 0.2, 0.3));
            }

            if draw_button(btn_rect, &label, active) {
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
                y += 30.0;
                
                let rank_str = match disciple.rank {
                    crate::data::disciples::DiscipleRank::Outer => "Outer Disciple",
                    crate::data::disciples::DiscipleRank::Inner => "Inner Disciple",
                    crate::data::disciples::DiscipleRank::Elder => "Elder",
                    crate::data::disciples::DiscipleRank::SectLeader => "Sect Leader",
                };
                draw_text(&format!("Rank: {}", rank_str), x, y, FONT_HEADER_SIZE, SECONDARY);
                y += 40.0;
                
                let stage = data.stages.get(&disciple.realm);
                let realm_name = stage.map(|s| s.name.as_str()).unwrap_or(&disciple.realm);

                let sub_stage_name = stage
                    .and_then(|s| s.sub_stages.get(disciple.sub_stage))
                    .map(|ss| format!(" - {}", ss.name))
                    .unwrap_or_default();
                    
                draw_text(&format!("Realm: {}{}", realm_name, sub_stage_name), x, y, FONT_HEADER_SIZE, TEXT_PRIMARY);
                y += 30.0;
                draw_text(&format!("Talent: {:?}", disciple.talent), x, y, FONT_BODY_SIZE, TEXT_SECONDARY);
                y += 30.0;

                // Injury Status
                if let Some(ref injury) = disciple.injury {
                    draw_rectangle(x - 5.0, y - 5.0, 420.0, 70.0, Color::new(0.5, 0.2, 0.2, 0.3));
                    draw_text(&format!("INJURED: {} {}", injury.severity_str(), injury.injury_type), x, y, FONT_HEADER_SIZE, FAILURE);
                    y += 25.0;
                    draw_text(&injury.description, x + 10.0, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                    y += 20.0;

                    let recovery_secs = injury.recovery_ticks_remaining / 60;
                    let recovery_rate = crate::data::disciples::Injury::get_recovery_rate(disciple.attributes.body);
                    draw_text(
                        &format!("Recovery: {} ticks (~{} sec) | Heal rate: {}/tick (Body bonus)",
                            injury.recovery_ticks_remaining, recovery_secs, recovery_rate),
                        x + 10.0, y, FONT_SMALL_SIZE, WARNING
                    );
                    y += 30.0;
                }

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

                // Qi and Laws (Non-Mortals only, usually Inner+)
                if disciple.rank != crate::data::disciples::DiscipleRank::Outer && disciple.realm != "Mortal" {
                     let qi_text = format!("Qi: {} / {}", disciple.qi, disciple.max_qi);
                     draw_text(&qi_text, x + 20.0, y, FONT_BODY_SIZE, TEXT_HIGHLIGHT);
                     y += 25.0;
                     
                     // Law Section
                     draw_text("Cultivation Law:", x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
                     y += 30.0;
                     let law_name = disciple.law_id.as_ref()
                         .and_then(|id| data.laws.get(id))
                         .map(|l| l.name.as_str())
                         .unwrap_or("None");
                     
                     draw_text(law_name, x + 20.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
                     
                     if draw_button(Rect::new(x + 200.0, y - 5.0, 150.0, 30.0), "Change Law", false) {
                         self.law_modal_open = true;
                     }
                     y += 25.0;
                } else if disciple.realm == "Mortal" {
                     // Generic text for Mortals (including starting Patriarch)
                     draw_text("Cultivation Method:", x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
                     y += 30.0;
                     draw_text("Basic Breathing (Mortal)", x + 20.0, y, FONT_BODY_SIZE, TEXT_SECONDARY);
                     y += 30.0;
                }
                
                // Item Usage
                y += 20.0;
                if draw_button(Rect::new(x, y, 150.0, 30.0), "Use Item", false) {
                    self.item_modal_open = true;
                }
                y += 30.0;

                // Bloodline
                if let Some(bloodline_id) = &disciple.bloodline.bloodline_id {
                    if let Some(bloodline) = data.bloodlines.get(bloodline_id) {
                        draw_text("Bloodline:", x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
                        y += 30.0;
                        
                        let rarity_color = match bloodline.rarity {
                            crate::data::bloodlines::BloodlineRarity::Mortal => Color::new(0.8, 0.8, 0.8, 1.0),
                            crate::data::bloodlines::BloodlineRarity::Spirit => Color::new(0.4, 0.8, 0.4, 1.0), // Green
                            crate::data::bloodlines::BloodlineRarity::Ancient => Color::new(0.4, 0.4, 1.0, 1.0), // Blue
                            crate::data::bloodlines::BloodlineRarity::Primordial => Color::new(0.8, 0.4, 0.8, 1.0), // Purple
                            crate::data::bloodlines::BloodlineRarity::Mythic => Color::new(1.0, 0.8, 0.2, 1.0), // Gold
                        };
                        
                        draw_text(&format!("{} [{:?}]", bloodline.name, bloodline.rarity), x + 20.0, y, FONT_BODY_SIZE, rarity_color);
                        y += 25.0;
                        draw_text(&format!("State: {:?} ({:.0}%)", disciple.bloodline.awakening_state, disciple.bloodline.awakening_progress * 100.0), x + 20.0, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                        y += 20.0;
                        
                        // Tooltip for effects
                        let dims = measure_text(&bloodline.name, None, FONT_BODY_SIZE as u16, 1.0);
                        let rect = Rect::new(x + 20.0, y - 45.0, dims.width, 50.0);
                         if rect.contains(mouse_position().into()) {
                             let mut lines = Vec::new();
                             lines.push(bloodline.description.clone());
                             if bloodline.passive_effects.cultivation_speed_modifier != 0.0 { lines.push(format!("Cultivation: {:+.0}%", bloodline.passive_effects.cultivation_speed_modifier * 100.0)); }
                             if bloodline.passive_effects.breakthrough_modifier != 0.0 { lines.push(format!("Breakthrough: {:+.0}%", bloodline.passive_effects.breakthrough_modifier * 100.0)); }
                             if bloodline.passive_effects.combat_modifier != 0.0 { lines.push(format!("Combat: {:+.0}%", bloodline.passive_effects.combat_modifier * 100.0)); }
                             if bloodline.passive_effects.survivor { lines.push("Survivor: Cannot die from failure.".to_string()); }
                            
                            let mp = mouse_position();
                            draw_tooltip_box(mp.0, mp.1, &lines);
                        }
                        
                        y += 10.0;
                    }
                }

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
                
                // Promotion (Outer -> Inner)
                if disciple.rank == crate::data::disciples::DiscipleRank::Outer {
                    y += 40.0;
                    if disciple.realm != "Mortal" {
                         if draw_button(Rect::new(x, y, 200.0, 40.0), "Promote to Inner (100 SS)", false) {
                             return UpdateResult::new().with_action(crate::engine::actions::Action::PromoteDisciple(idx));
                         }
                    } else {
                         draw_text("Reach Qi Refinement to Promote", x, y, FONT_BODY_SIZE, TEXT_SECONDARY);
                    }
                } else {
                    // Salary Info for Inner+
                    y += 40.0;
                    draw_text("Salary: 1 SS / day", x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                }
            }
        }
        
        // --- Law Selection Modal ---
        if self.law_modal_open {
            if let Some(idx) = self.selected_index {
                draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
                let modal_w = 400.0;
                let modal_h = 500.0;
                let modal_x = (screen_w - modal_w) / 2.0;
                let modal_y = (screen_h - modal_h) / 2.0;
                let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);
                
                draw_panel(modal_rect, Some("Select Cultivation Law"));
                
                let mut m_y = modal_y + 50.0;
                
                if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
                    self.law_modal_open = false;
                }

                for law in data.laws.values() {
                    // Title Button
                    if draw_button(Rect::new(modal_x + 20.0, m_y, modal_w - 40.0, 40.0), &format!("{} [{:?}]", law.name, law.element), false) {
                        self.law_modal_open = false;
                        return UpdateResult::new().with_action(crate::engine::actions::Action::AssignLaw(idx, law.id.clone()));
                    }
                    m_y += 45.0;
                    
                    // Description (Small)
                    draw_text(&law.description, modal_x + 25.0, m_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                    m_y += 20.0;
                    
                    // Stats
                    draw_text(&format!("Boosts: Body x{}, Mind x{}, Spirit x{}", law.stat_growth_modifiers.body, law.stat_growth_modifiers.mind, law.stat_growth_modifiers.spirit), modal_x + 25.0, m_y, FONT_SMALL_SIZE, TEXT_HIGHLIGHT);
                    m_y += 25.0;
                }
            } else {
                self.law_modal_open = false; 
            }
        }
        
        // --- Item Use Modal ---
        if self.item_modal_open {
             if let Some(idx) = self.selected_index {
                 draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
                 let modal_w = 400.0;
                 let modal_h = 500.0;
                 let modal_x = (screen_w - modal_w) / 2.0;
                 let modal_y = (screen_h - modal_h) / 2.0;
                 let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);
                 
                 draw_panel(modal_rect, Some("Use Item"));
                 
                 if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
                     self.item_modal_open = false;
                 }
                 
                 let mut i_y = modal_y + 50.0;
                 let mut found_any = false;
                 
                 for (item_id, count) in inventory {
                     if *count > 0 {
                         if let Some(item) = data.items.get(item_id) {
                             found_any = true;
                             if draw_button(Rect::new(modal_x + 20.0, i_y, modal_w - 40.0, 40.0), &format!("{} (x{})", item.name, count), false) {
                                 // Use logic
                                 return UpdateResult::new().with_action(crate::engine::actions::Action::UseItem(item_id.clone(), idx));
                             }
                             // Tooltip
                             if Rect::new(modal_x + 20.0, i_y, modal_w - 40.0, 40.0).contains(mouse_position().into()) {
                                 draw_tooltip(mouse_position().into(), &item.description);
                             }
                             
                             i_y += 45.0;
                         }
                     }
                 }
                 
                 if !found_any {
                     draw_text("Inventory Empty", modal_x + 20.0, i_y, FONT_BODY_SIZE, TEXT_SECONDARY);
                 }
             } else {
                 self.item_modal_open = false;
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
