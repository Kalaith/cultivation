use crate::data::disciples::{Disciple, DiscipleRank};
use crate::data::loader::GameData;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

mod details;
mod modals;

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
            RosterFilter::Inner => matches!(
                disciple.rank,
                DiscipleRank::Inner | DiscipleRank::Elder | DiscipleRank::SectLeader
            ),
            RosterFilter::Injured => disciple.is_injured(),
            RosterFilter::Ready => disciple.can_attempt_breakthrough(),
        }
    }
}

pub struct DiscipleRosterState {
    /// Index into the filtered list, not the full disciples array.
    selected_filtered_index: Option<usize>,
    law_modal_open: bool,
    item_modal_open: bool,
    equip_modal_open: bool,
    scroll_offset: f32,
    filter: RosterFilter,
}

impl DiscipleRosterState {
    pub fn new() -> Self {
        Self {
            selected_filtered_index: None,
            law_modal_open: false,
            item_modal_open: false,
            equip_modal_open: false,
            scroll_offset: 0.0,
            filter: RosterFilter::All,
        }
    }

    pub fn update(
        &mut self,
        data: &GameData,
        disciples: &[Disciple],
        inventory: &std::collections::HashMap<String, u32>,
    ) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;
        let filtered: Vec<(usize, &Disciple)> = disciples
            .iter()
            .enumerate()
            .filter(|(_, d)| self.filter.matches(d))
            .collect();
        let selected_actual_index = self
            .selected_filtered_index
            .and_then(|fi| filtered.get(fi))
            .map(|(actual_idx, _)| *actual_idx);

        self.draw_header(screen_w, header_h, disciples);

        let content_y = header_h + 10.0;
        let content_h = screen_h - content_y - 10.0;
        let left_w = 320.0;
        let left_rect = Rect::new(10.0, content_y, left_w, content_h);
        self.draw_roster_list(left_rect, content_h, disciples, &filtered);

        let right_rect = Rect::new(
            left_w + 20.0,
            content_y,
            screen_w - left_w - 30.0,
            content_h,
        );
        draw_panel(
            right_rect,
            Some(if selected_actual_index.is_some() {
                "Character Scroll"
            } else {
                "Unopened Scroll"
            }),
        );

        if let Some(idx) = selected_actual_index {
            if let Some(disciple) = disciples.get(idx) {
                self.draw_disciple_details(data, disciple, idx, &right_rect, inventory);
                if let Some(result) = self.handle_detail_actions(data, disciples, idx, inventory) {
                    return result;
                }
            }
        } else {
            draw_ui_text(
                "Select a disciple tablet to read their cultivation record.",
                right_rect.x + 24.0,
                right_rect.y + 88.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        if let Some(result) =
            self.draw_open_modal(data, selected_actual_index, inventory, disciples)
        {
            return result;
        }

        if draw_button(
            Rect::new(screen_w - 120.0, screen_h - 50.0, 100.0, 40.0),
            "Back",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    fn draw_header(&self, screen_w: f32, header_h: f32, disciples: &[Disciple]) {
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_screen_title(
            "Character Scrolls",
            "Records of bodies, spirit roots, and unfinished destinies",
            20.0,
            31.0,
        );

        let injured_count = disciples.iter().filter(|d| d.is_injured()).count();
        let ready_count = disciples
            .iter()
            .filter(|d| d.can_attempt_breakthrough())
            .count();
        let mut seal_x = screen_w - 360.0;
        seal_x +=
            draw_resource_seal(seal_x, 38.0, "Disciples", disciples.len() as u32, PRIMARY) + 8.0;
        seal_x += draw_resource_seal(seal_x, 38.0, "Injured", injured_count as u32, FAILURE) + 8.0;
        draw_resource_seal(seal_x, 38.0, "Breakthroughs", ready_count as u32, WARNING);
    }

    fn draw_roster_list(
        &mut self,
        left_rect: Rect,
        content_h: f32,
        disciples: &[Disciple],
        filtered: &[(usize, &Disciple)],
    ) {
        draw_panel(left_rect, Some("Sect Tablets"));

        let filter_y = left_rect.y + 40.0;
        let filters = [
            RosterFilter::All,
            RosterFilter::Outer,
            RosterFilter::Inner,
            RosterFilter::Injured,
            RosterFilter::Ready,
        ];
        let btn_width = (left_rect.w - 30.0) / filters.len() as f32;
        for (i, f) in filters.iter().enumerate() {
            let btn_x = left_rect.x + 10.0 + (i as f32 * btn_width);
            if draw_button(
                Rect::new(btn_x, filter_y, btn_width - 5.0, 25.0),
                f.label(),
                self.filter == *f,
            ) && self.filter != *f
            {
                self.filter = *f;
                self.selected_filtered_index = None;
                self.scroll_offset = 0.0;
            }
        }

        let list_y = filter_y + 35.0;
        let list_h = content_h - 85.0;
        let list_rect = Rect::new(left_rect.x + 5.0, list_y, left_rect.w - 10.0, list_h);
        let item_height = 45.0;
        let total_height = filtered.len() as f32 * item_height;

        if list_rect.contains(mouse_position().into()) {
            let wheel = mouse_wheel().1;
            if total_height > list_h {
                self.scroll_offset -= wheel * 30.0;
                self.scroll_offset = self
                    .scroll_offset
                    .clamp(0.0, (total_height - list_h).max(0.0));
            } else {
                self.scroll_offset = 0.0;
            }
        }

        let mut btn_y = list_y - self.scroll_offset;
        for (filtered_idx, (_actual_idx, disciple)) in filtered.iter().enumerate() {
            if btn_y + item_height < list_y {
                btn_y += item_height;
                continue;
            }
            if btn_y > list_y + list_h {
                break;
            }

            let is_selected = self.selected_filtered_index == Some(filtered_idx);
            let label = disciple_tablet_label(disciple);
            let btn_rect = Rect::new(
                left_rect.x + 10.0,
                btn_y,
                left_rect.w - 30.0,
                item_height - 5.0,
            );

            if disciple.is_injured() {
                draw_rectangle(
                    btn_rect.x,
                    btn_rect.y,
                    btn_rect.w,
                    btn_rect.h,
                    Color::new(0.50, 0.14, 0.12, 0.26),
                );
            } else if disciple.can_attempt_breakthrough() {
                draw_rectangle(
                    btn_rect.x,
                    btn_rect.y,
                    btn_rect.w,
                    btn_rect.h,
                    Color::new(0.80, 0.55, 0.18, 0.24),
                );
            }

            if draw_button(btn_rect, &label, is_selected) {
                self.selected_filtered_index = Some(filtered_idx);
            }
            btn_y += item_height;
        }

        if total_height > list_h {
            let track_x = left_rect.x + left_rect.w - 8.0;
            let handle_h = (list_h * list_h / total_height).max(20.0);
            let max_offset = (total_height - list_h).max(1.0);
            let handle_y = list_y + (self.scroll_offset / max_offset) * (list_h - handle_h);
            draw_rectangle(track_x, list_y, 4.0, list_h, PANEL_BORDER);
            draw_rectangle(track_x - 1.0, handle_y, 6.0, handle_h, TEXT_HIGHLIGHT);
        }

        draw_ui_text(
            &format!("Records: {} / {}", filtered.len(), disciples.len()),
            left_rect.x + 10.0,
            left_rect.y + content_h - 10.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_open_modal(
        &mut self,
        data: &GameData,
        selected_actual_index: Option<usize>,
        inventory: &std::collections::HashMap<String, u32>,
        disciples: &[Disciple],
    ) -> Option<UpdateResult> {
        if self.law_modal_open {
            if let Some(idx) = selected_actual_index {
                return self.draw_law_modal(data, idx);
            }
            self.law_modal_open = false;
        }

        if self.item_modal_open {
            if let Some(idx) = selected_actual_index {
                return self.draw_item_modal(data, idx, inventory);
            }
            self.item_modal_open = false;
        }

        if self.equip_modal_open {
            if let Some(idx) = selected_actual_index {
                return self.draw_equip_modal(data, idx, inventory, disciples);
            }
            self.equip_modal_open = false;
        }

        None
    }

    pub fn draw(&self, _data: &GameData, _disciples: &[Disciple], _spirit_stones: u32) {
        // Handled in update.
    }
}

fn disciple_tablet_label(disciple: &Disciple) -> String {
    let rank = match disciple.rank {
        DiscipleRank::Outer => "Outer",
        DiscipleRank::Inner => "Inner",
        DiscipleRank::Elder => "Elder",
        DiscipleRank::SectLeader => "Patriarch",
    };

    let mut status = String::new();
    if disciple.is_injured() {
        status.push_str(" | Injured");
    }
    if disciple.can_attempt_breakthrough() {
        status.push_str(" | Breakthrough");
    }

    format!("{} - {}{}", disciple.name, rank, status)
}
