use crate::data::disciples::{Disciple, DiscipleRank};
use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RosterFilter {
    All,
    Outer,
    Inner,
    Injured,
    Ready,
}

impl RosterFilter {
    fn label(&self) -> &'static str {
        match self {
            RosterFilter::All => "All",
            RosterFilter::Outer => "Outer",
            RosterFilter::Inner => "Inner",
            RosterFilter::Injured => "Injured",
            RosterFilter::Ready => "Ready",
        }
    }

    fn matches(&self, disciple: &Disciple) -> bool {
        match self {
            RosterFilter::All => true,
            RosterFilter::Outer => disciple.rank == DiscipleRank::Outer,
            RosterFilter::Inner => matches!(disciple.rank, DiscipleRank::Inner | DiscipleRank::Elder | DiscipleRank::SectLeader),
            RosterFilter::Injured => disciple.is_injured(),
            RosterFilter::Ready => disciple.can_attempt_breakthrough(),
        }
    }
}

pub struct DiscipleRosterState {
    /// Index into the FILTERED list (not the full disciples array)
    selected_filtered_index: Option<usize>,
    law_modal_open: bool,
    item_modal_open: bool,
    scroll_offset: f32,
    filter: RosterFilter,
}

impl DiscipleRosterState {
    pub fn new() -> Self {
        Self {
            selected_filtered_index: None,
            law_modal_open: false,
            item_modal_open: false,
            scroll_offset: 0.0,
            filter: RosterFilter::All,
        }
    }

    pub fn update(&mut self, data: &GameData, disciples: &[Disciple], inventory: &std::collections::HashMap<String, u32>) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;

        // Build filtered list with original indices
        let filtered: Vec<(usize, &Disciple)> = disciples
            .iter()
            .enumerate()
            .filter(|(_, d)| self.filter.matches(d))
            .collect();

        // Get selected disciple's actual index
        let selected_actual_index = self.selected_filtered_index
            .and_then(|fi| filtered.get(fi))
            .map(|(actual_idx, _)| *actual_idx);

        // --- Header ---
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("DISCIPLE ROSTER", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);

        // Count stats for header
        let injured_count = disciples.iter().filter(|d| d.is_injured()).count();
        let ready_count = disciples.iter().filter(|d| d.can_attempt_breakthrough()).count();
        draw_text(
            &format!("Total: {} | Injured: {} | Ready: {}", disciples.len(), injured_count, ready_count),
            screen_w - 350.0, 40.0, FONT_BODY_SIZE, TEXT_HIGHLIGHT
        );

        // --- Layout ---
        let content_y = header_h + 10.0;
        let content_h = screen_h - content_y - 10.0;
        let left_w = 320.0;
        let right_w = screen_w - left_w - 30.0;

        // --- Left Panel: Disciple List ---
        let left_rect = Rect::new(10.0, content_y, left_w, content_h);
        draw_panel(left_rect, Some("Disciples"));

        // Filter buttons
        let filter_y = left_rect.y + 40.0;
        let filters = [RosterFilter::All, RosterFilter::Outer, RosterFilter::Inner, RosterFilter::Injured, RosterFilter::Ready];
        let btn_width = (left_w - 30.0) / filters.len() as f32;

        for (i, f) in filters.iter().enumerate() {
            let btn_x = left_rect.x + 10.0 + (i as f32 * btn_width);
            let is_active = self.filter == *f;
            if draw_button(Rect::new(btn_x, filter_y, btn_width - 5.0, 25.0), f.label(), is_active) {
                if self.filter != *f {
                    self.filter = *f;
                    self.selected_filtered_index = None;
                    self.scroll_offset = 0.0;
                }
            }
        }

        // List area
        let list_y = filter_y + 35.0;
        let list_h = content_h - 85.0;
        let list_rect = Rect::new(left_rect.x + 5.0, list_y, left_w - 10.0, list_h);

        let item_height = 45.0;
        let total_height = filtered.len() as f32 * item_height;

        // Handle scroll wheel
        if list_rect.contains(mouse_position().into()) {
            let wheel = mouse_wheel().1;
            if total_height > list_h {
                self.scroll_offset -= wheel * 30.0;
                self.scroll_offset = self.scroll_offset.clamp(0.0, (total_height - list_h).max(0.0));
            } else {
                self.scroll_offset = 0.0;
            }
        }

        // Draw filtered disciple list
        let mut btn_y = list_y - self.scroll_offset;
        for (filtered_idx, (actual_idx, disciple)) in filtered.iter().enumerate() {
            // Skip if outside visible area
            if btn_y + item_height < list_y {
                btn_y += item_height;
                continue;
            }
            if btn_y > list_y + list_h {
                break;
            }

            let is_selected = self.selected_filtered_index == Some(filtered_idx);
            let rank_prefix = match disciple.rank {
                DiscipleRank::Outer => "[O]",
                DiscipleRank::Inner => "[I]",
                DiscipleRank::Elder => "[E]",
                DiscipleRank::SectLeader => "[L]",
            };

            // Status indicators
            let mut status = String::new();
            if disciple.is_injured() {
                status.push_str(" !INJ");
            }
            if disciple.can_attempt_breakthrough() {
                status.push_str(" *RDY");
            }

            let label = format!("{} {}{}", rank_prefix, disciple.name, status);
            let btn_rect = Rect::new(left_rect.x + 10.0, btn_y, left_w - 30.0, item_height - 5.0);

            // Background tint for status
            if disciple.is_injured() {
                draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, Color::new(0.5, 0.2, 0.2, 0.3));
            } else if disciple.can_attempt_breakthrough() {
                draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, Color::new(0.2, 0.4, 0.3, 0.3));
            }

            if draw_button(btn_rect, &label, is_selected) {
                self.selected_filtered_index = Some(filtered_idx);
            }
            btn_y += item_height;
        }

        // Scroll indicator
        if total_height > list_h {
            let track_x = left_rect.x + left_w - 8.0;
            let track_h = list_h;
            draw_rectangle(track_x, list_y, 4.0, track_h, PANEL_BORDER);

            let handle_h = (list_h * list_h / total_height).max(20.0);
            let max_offset = (total_height - list_h).max(1.0);
            let handle_y = list_y + (self.scroll_offset / max_offset) * (track_h - handle_h);
            draw_rectangle(track_x - 1.0, handle_y, 6.0, handle_h, TEXT_HIGHLIGHT);
        }

        // Show filtered count
        draw_text(
            &format!("Showing {} of {}", filtered.len(), disciples.len()),
            left_rect.x + 10.0, left_rect.y + content_h - 10.0, FONT_SMALL_SIZE, TEXT_SECONDARY
        );

        // --- Right Panel: Details ---
        let right_rect = Rect::new(left_w + 20.0, content_y, right_w, content_h);
        draw_panel(right_rect, Some(if selected_actual_index.is_some() { "Disciple Details" } else { "Select a Disciple" }));

        if let Some(idx) = selected_actual_index {
            if let Some(disciple) = disciples.get(idx) {
                self.draw_disciple_details(data, disciple, idx, &right_rect, inventory);

                // Check for actions from details panel
                if let Some(result) = self.handle_detail_actions(data, disciples, idx, inventory) {
                    return result;
                }
            }
        }

        // --- Modals ---
        if self.law_modal_open {
            if let Some(idx) = selected_actual_index {
                if let Some(result) = self.draw_law_modal(data, idx) {
                    return result;
                }
            } else {
                self.law_modal_open = false;
            }
        }

        if self.item_modal_open {
            if let Some(idx) = selected_actual_index {
                if let Some(result) = self.draw_item_modal(data, idx, inventory) {
                    return result;
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

    fn draw_disciple_details(&mut self, data: &GameData, disciple: &Disciple, _idx: usize, right_rect: &Rect, _inventory: &std::collections::HashMap<String, u32>) {
        let x = right_rect.x + 20.0;
        let mut y = right_rect.y + 50.0;

        draw_text(&disciple.name, x, y, FONT_TITLE_SIZE, PRIMARY);
        y += 30.0;

        let rank_str = match disciple.rank {
            DiscipleRank::Outer => "Outer Disciple",
            DiscipleRank::Inner => "Inner Disciple",
            DiscipleRank::Elder => "Elder",
            DiscipleRank::SectLeader => "Sect Leader",
        };
        draw_text(&format!("Rank: {}", rank_str), x, y, FONT_HEADER_SIZE, SECONDARY);
        y += 35.0;

        let stage = data.stages.get(&disciple.realm);
        let realm_name = stage.map(|s| s.name.as_str()).unwrap_or(&disciple.realm);
        let sub_stage_name = stage
            .and_then(|s| s.sub_stages.get(disciple.sub_stage))
            .map(|ss| format!(" - {}", ss.name))
            .unwrap_or_default();

        draw_text(&format!("Realm: {}{}", realm_name, sub_stage_name), x, y, FONT_BODY_SIZE, TEXT_PRIMARY);
        y += 25.0;
        draw_text(&format!("Talent: {:?}", disciple.talent), x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
        y += 25.0;

        // Injury Status
        if let Some(ref injury) = disciple.injury {
            draw_rectangle(x - 5.0, y - 5.0, 400.0, 60.0, Color::new(0.5, 0.2, 0.2, 0.3));
            draw_text(&format!("INJURED: {} {}", injury.severity_str(), injury.injury_type), x, y, FONT_BODY_SIZE, FAILURE);
            y += 20.0;

            // Recovery ticks = seconds (healing happens once per cultivation tick, ~1/sec)
            let heal_rate = crate::data::disciples::Injury::get_recovery_rate(disciple.attributes.body);
            let recovery_secs = injury.recovery_ticks_remaining / heal_rate.max(1);
            let display = if recovery_secs >= 60 {
                format!("~{} min remaining (heal rate: {}/sec)", recovery_secs / 60, heal_rate)
            } else {
                format!("~{} sec remaining (heal rate: {}/sec)", recovery_secs, heal_rate)
            };
            draw_text(&display, x + 10.0, y, FONT_SMALL_SIZE, WARNING);
            y += 30.0;
        }

        // Attributes (compact)
        draw_text(
            &format!("Body: {}  Mind: {}  Spirit: {}",
                disciple.attributes.body, disciple.attributes.mind, disciple.attributes.spirit),
            x, y, FONT_BODY_SIZE, TEXT_PRIMARY
        );
        y += 30.0;

        // Qi (for non-mortals)
        if disciple.rank != DiscipleRank::Outer && disciple.realm != "Mortal" {
            draw_text(&format!("Qi: {} / {}", disciple.qi, disciple.max_qi), x, y, FONT_BODY_SIZE, TEXT_HIGHLIGHT);
            y += 25.0;

            // Law
            let law_name = disciple.law_id.as_ref()
                .and_then(|id| data.laws.get(id))
                .map(|l| l.name.as_str())
                .unwrap_or("None");
            draw_text(&format!("Law: {}", law_name), x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
            y += 25.0;
        }

        // Bloodline (compact)
        if let Some(bloodline_id) = &disciple.bloodline.bloodline_id {
            if let Some(bloodline) = data.bloodlines.get(bloodline_id) {
                let rarity_color = match bloodline.rarity {
                    crate::data::bloodlines::BloodlineRarity::Mortal => TEXT_SECONDARY,
                    crate::data::bloodlines::BloodlineRarity::Spirit => SUCCESS,
                    crate::data::bloodlines::BloodlineRarity::Ancient => Color::new(0.4, 0.4, 1.0, 1.0),
                    crate::data::bloodlines::BloodlineRarity::Primordial => Color::new(0.8, 0.4, 0.8, 1.0),
                    crate::data::bloodlines::BloodlineRarity::Mythic => WARNING,
                };
                draw_text(&format!("Bloodline: {} ({:.0}%)", bloodline.name, disciple.bloodline.awakening_progress * 100.0), x, y, FONT_SMALL_SIZE, rarity_color);
                y += 25.0;
            }
        }

        // Traits (compact, max 2 shown)
        if !disciple.fate_traits.is_empty() {
            let traits_str: String = disciple.fate_traits.iter()
                .take(2)
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let suffix = if disciple.fate_traits.len() > 2 { "..." } else { "" };
            draw_text(&format!("Traits: {}{}", traits_str, suffix), x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
            y += 25.0;
        }

        // Experience Bar
        y += 10.0;
        draw_text("Cultivation Progress:", x, y, FONT_SMALL_SIZE, TEXT_PRIMARY);
        y += 15.0;
        let progress = disciple.exp as f32 / disciple.exp_to_next_level as f32;
        draw_progress_bar(Rect::new(x, y, 350.0, 15.0), progress.min(1.0), SECONDARY);

        // Show overflow if ready
        if disciple.exp >= disciple.exp_to_next_level {
            let overflow_pct = ((disciple.exp as f32 / disciple.exp_to_next_level as f32 - 1.0) * 100.0) as u32;
            draw_text(&format!("+{}% overflow", overflow_pct), x + 360.0, y + 12.0, FONT_SMALL_SIZE, SUCCESS);
        }
    }

    fn handle_detail_actions(&mut self, data: &GameData, disciples: &[Disciple], idx: usize, _inventory: &std::collections::HashMap<String, u32>) -> Option<UpdateResult> {
        let disciple = disciples.get(idx)?;
        let right_x = 350.0;
        let screen_h = screen_height();
        let btn_y = screen_h - 180.0;

        // Action buttons at bottom of detail panel
        let btn_w = 140.0;
        let btn_h = 35.0;
        let spacing = 10.0;

        // Row 1: Breakthrough (if ready)
        if disciple.can_attempt_breakthrough() {
            let readiness_bonus = (disciple.get_readiness_bonus() * 100.0) as u32;
            if draw_button(
                Rect::new(right_x, btn_y, btn_w * 2.0 + spacing, btn_h),
                &format!("Attempt Breakthrough (+{}%)", readiness_bonus),
                false
            ) {
                return Some(UpdateResult::new().with_action(crate::engine::actions::Action::AttemptBreakthrough(idx)));
            }
        }

        // Row 2: Use Item, Change Law
        let row2_y = btn_y + btn_h + spacing;

        if draw_button(Rect::new(right_x, row2_y, btn_w, btn_h), "Use Item", false) {
            self.item_modal_open = true;
        }

        if disciple.rank != DiscipleRank::Outer && disciple.realm != "Mortal" {
            if draw_button(Rect::new(right_x + btn_w + spacing, row2_y, btn_w, btn_h), "Change Law", false) {
                self.law_modal_open = true;
            }
        }

        // Row 3: Promote (if applicable)
        let row3_y = row2_y + btn_h + spacing;

        if disciple.rank == DiscipleRank::Outer && disciple.realm != "Mortal" {
            if draw_button(Rect::new(right_x, row3_y, btn_w * 2.0 + spacing, btn_h), "Promote to Inner (100 SS)", false) {
                return Some(UpdateResult::new().with_action(crate::engine::actions::Action::PromoteDisciple(idx)));
            }
        } else if disciple.rank == DiscipleRank::Outer && disciple.realm == "Mortal" {
            draw_text("Reach Qi Refinement to promote", right_x, row3_y + 20.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
        }

        None
    }

    fn draw_law_modal(&mut self, data: &GameData, idx: usize) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 400.0;
        let modal_h = 400.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Select Cultivation Law"));

        if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "X", false) {
            self.law_modal_open = false;
        }

        let mut m_y = modal_y + 50.0;
        for law in data.laws.values() {
            if draw_button(Rect::new(modal_x + 20.0, m_y, modal_w - 40.0, 35.0), &format!("{} [{:?}]", law.name, law.element), false) {
                self.law_modal_open = false;
                return Some(UpdateResult::new().with_action(crate::engine::actions::Action::AssignLaw(idx, law.id.clone())));
            }
            m_y += 40.0;

            draw_text(&law.description, modal_x + 25.0, m_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
            m_y += 25.0;
        }

        None
    }

    fn draw_item_modal(&mut self, data: &GameData, idx: usize, inventory: &std::collections::HashMap<String, u32>) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 400.0;
        let modal_h = 400.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Use Item"));

        if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "X", false) {
            self.item_modal_open = false;
        }

        let mut i_y = modal_y + 50.0;
        let mut found_any = false;

        for (item_id, count) in inventory {
            if *count > 0 {
                if let Some(item) = data.items.get(item_id) {
                    found_any = true;
                    if draw_button(Rect::new(modal_x + 20.0, i_y, modal_w - 40.0, 35.0), &format!("{} (x{})", item.name, count), false) {
                        self.item_modal_open = false;
                        return Some(UpdateResult::new().with_action(crate::engine::actions::Action::UseItem(item_id.clone(), idx)));
                    }

                    if Rect::new(modal_x + 20.0, i_y, modal_w - 40.0, 35.0).contains(mouse_position().into()) {
                        draw_tooltip(mouse_position().into(), &item.description);
                    }

                    i_y += 40.0;
                }
            }
        }

        if !found_any {
            draw_text("Inventory Empty", modal_x + 20.0, i_y, FONT_BODY_SIZE, TEXT_SECONDARY);
        }

        None
    }

    pub fn draw(&self, _data: &GameData, _disciples: &[Disciple], _spirit_stones: u32) {
        // Handled in update
    }
}
