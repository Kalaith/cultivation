use super::*;
use crate::data::spirit_beasts::{BeastEquipmentSlot, BeastStats};
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

impl SectBaseState {
    pub(super) fn draw_spirit_beasts_view(
        &mut self,
        rect: Rect,
        data: &GameData,
        spirit_beasts: &[SpiritBeast],
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        draw_panel(rect, Some("Mountain Guardian Registry"));
        self.draw_beast_registry_header(rect, spirit_beasts);

        let list_w = 310.0;
        let list_rect = Rect::new(rect.x + 18.0, rect.y + 92.0, list_w, rect.h - 116.0);
        let detail_rect = Rect::new(
            list_rect.x + list_w + 18.0,
            list_rect.y,
            rect.w - list_w - 54.0,
            list_rect.h,
        );

        self.draw_beast_roster(list_rect, spirit_beasts);
        draw_panel(detail_rect, Some("Guardian Bond"));

        if let Some(idx) = self
            .selected_beast_index
            .and_then(|i| spirit_beasts.get(i).map(|_| i))
        {
            if self.draw_beast_details_panel(detail_rect, &spirit_beasts[idx], data) {
                self.beast_equip_modal_open = true;
            }
        } else {
            self.draw_empty_beast_detail(detail_rect, spirit_beasts);
        }

        if self.beast_equip_modal_open {
            if let Some(idx) = self
                .selected_beast_index
                .and_then(|i| spirit_beasts.get(i).map(|_| i))
            {
                if let Some(res) = self.draw_beast_equip_modal(data, &spirit_beasts[idx], inventory)
                {
                    return Some(res);
                }
            } else {
                self.beast_equip_modal_open = false;
            }
        }

        if draw_button(
            Rect::new(rect.x + rect.w - 120.0, rect.y + 10.0, 100.0, 30.0),
            "Return",
            false,
        ) {
            self.view = SectView::Map;
        }

        None
    }

    fn draw_beast_registry_header(&self, rect: Rect, spirit_beasts: &[SpiritBeast]) {
        let bonded = spirit_beasts.len() as u32;
        let loyal = spirit_beasts
            .iter()
            .filter(|beast| beast.loyalty >= 50)
            .count() as u32;
        let hungry = spirit_beasts
            .iter()
            .filter(|beast| beast.hunger > 50)
            .count() as u32;
        let y = rect.y + 50.0;

        draw_ui_text(
            "Ancient pacts bind fang, claw, and spirit to the mountain paths.",
            rect.x + 20.0,
            y,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        let mut seal_x = rect.x + rect.w - 360.0;
        seal_x += draw_resource_seal(seal_x, y + 2.0, "Bonded", bonded, PRIMARY) + 8.0;
        seal_x += draw_resource_seal(seal_x, y + 2.0, "Loyal", loyal, SUCCESS) + 8.0;
        draw_resource_seal(seal_x, y + 2.0, "Hungry", hungry, FAILURE);
    }

    fn draw_beast_roster(&mut self, rect: Rect, spirit_beasts: &[SpiritBeast]) {
        draw_panel(rect, Some("Bonded Beasts"));

        if spirit_beasts.is_empty() {
            draw_wrapped_text(
                "No spirit beast guards the sect yet. Recruit one to watch the paths disciples cannot.",
                rect.x + 20.0,
                rect.y + 70.0,
                rect.w - 40.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            return;
        }

        let mut y = rect.y + 52.0;
        for (i, beast) in spirit_beasts.iter().enumerate() {
            let card = Rect::new(rect.x + 12.0, y, rect.w - 24.0, 70.0);
            let selected = self.selected_beast_index == Some(i);
            draw_beast_roster_card(card, beast, selected);
            if card.contains(mouse_position().into()) && is_mouse_button_pressed(MouseButton::Left)
            {
                self.selected_beast_index = Some(i);
            }
            y += 80.0;
            if y > rect.y + rect.h - 56.0 {
                break;
            }
        }
    }

    fn draw_empty_beast_detail(&self, rect: Rect, spirit_beasts: &[SpiritBeast]) {
        let message = if spirit_beasts.is_empty() {
            "The guardian altar is quiet. A bonded beast would make the mountain feel less abandoned."
        } else {
            "Choose a beast tablet to inspect its bond, hunger, and guardian gear."
        };
        draw_wrapped_text(
            message,
            rect.x + 24.0,
            rect.y + 82.0,
            rect.w - 48.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
    }

    fn draw_beast_details_panel(
        &self,
        right_rect: Rect,
        beast: &SpiritBeast,
        data: &GameData,
    ) -> bool {
        let x = right_rect.x + 24.0;
        let mut y = right_rect.y + 62.0;
        draw_ui_text(&beast.name, x, y, FONT_TITLE_SIZE, PRIMARY);
        y += 34.0;
        draw_ui_text(
            &format!(
                "{} guardian | {:?} tier | {:?} aspect",
                beast.species, beast.tier, beast.element
            ),
            x,
            y,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        y += 38.0;
        draw_ink_divider(x, y, right_rect.w - 48.0);
        y += 36.0;

        draw_bond_meter(x, y, right_rect.w - 48.0, "Loyalty", beast.loyalty, SUCCESS);
        y += 44.0;
        draw_bond_meter(
            x,
            y,
            right_rect.w - 48.0,
            "Hunger",
            beast.hunger,
            hunger_color(beast.hunger),
        );
        y += 54.0;

        draw_beast_stats_grid(Rect::new(x, y, right_rect.w - 48.0, 92.0), &beast.stats);
        y += 118.0;

        draw_ui_text("Guardian Gear", x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
        y += 30.0;
        for slot in beast_slots() {
            draw_equipment_line(x, y, right_rect.w - 48.0, beast, data, slot);
            y += 24.0;
        }

        let button_rect = Rect::new(x, right_rect.y + right_rect.h - 58.0, 210.0, 38.0);
        draw_button(button_rect, "Equip Guardian Gear", false)
    }

    pub(super) fn draw_beast_equip_modal(
        &mut self,
        data: &GameData,
        beast: &SpiritBeast,
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(
            0.0,
            0.0,
            screen_w,
            screen_h,
            Color::new(0.0, 0.0, 0.0, 0.78),
        );
        let modal_w = 500.0;
        let modal_h = 450.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Guardian Gear Altar"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 70.0, modal_y + 10.0, 58.0, 30.0),
            "Seal",
            false,
        ) {
            self.beast_equip_modal_open = false;
        }

        draw_wrapped_text(
            "Choose a beast-only item from sect stores. Gear that fails the beast's tier requirement remains sealed.",
            modal_x + 24.0,
            modal_y + 66.0,
            modal_w - 48.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );

        let mut y = modal_y + 132.0;
        let mut items: Vec<_> = data.beast_equipment_definitions.values().collect();
        items.sort_by_key(|i| i.name.clone());

        let mut found_any = false;
        for item in items {
            let count = *inventory.get(&item.id).unwrap_or(&0);
            if count == 0 {
                continue;
            }
            found_any = true;
            let can_equip = beast.can_equip(item);
            let label = format!(
                "{} ({:?}) x{}{}",
                item.name,
                item.slot,
                count,
                if can_equip { "" } else { " - sealed" }
            );
            if draw_button(
                Rect::new(modal_x + 24.0, y, modal_w - 48.0, 36.0),
                &label,
                can_equip,
            ) && can_equip
            {
                self.beast_equip_modal_open = false;
                return Some(
                    UpdateResult::new()
                        .with_action(Action::EquipBeastItem(beast.id, item.id.clone())),
                );
            }
            y += 44.0;
            if y > modal_y + modal_h - 48.0 {
                break;
            }
        }

        if !found_any {
            draw_wrapped_text(
                "No beast equipment rests in the sect stores.",
                modal_x + 24.0,
                modal_y + 154.0,
                modal_w - 48.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        None
    }
}

fn draw_beast_roster_card(rect: Rect, beast: &SpiritBeast, selected: bool) {
    let hovered = rect.contains(mouse_position().into());
    let color = beast_element_color(&beast.element);
    let alpha = if selected {
        0.68
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
        rect.y + 28.0,
        11.0,
        Color::new(color.r, color.g, color.b, 0.48),
    );
    draw_ui_text(
        &beast.name,
        rect.x + 44.0,
        rect.y + 26.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "{} | {:?} | Loyalty {}",
            beast.species, beast.tier, beast.loyalty
        ),
        rect.x + 44.0,
        rect.y + 52.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
}

fn draw_bond_meter(x: f32, y: f32, width: f32, label: &str, value: i32, color: Color) {
    let pct = (value as f32 / 100.0).clamp(0.0, 1.0);
    draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
    draw_progress_bar(
        Rect::new(x + 92.0, y - 12.0, width - 140.0, 14.0),
        pct,
        color,
    );
    draw_ui_text(
        &value.to_string(),
        x + width - 34.0,
        y,
        FONT_SMALL_SIZE,
        color,
    );
}

fn draw_beast_stats_grid(rect: Rect, stats: &BeastStats) {
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
    let stats = [
        ("Health", stats.health),
        ("Attack", stats.attack),
        ("Defense", stats.defense),
        ("Speed", stats.speed),
        ("Qi", stats.qi),
    ];
    for (i, (label, value)) in stats.iter().enumerate() {
        let col = i % 3;
        let row = i / 3;
        let x = rect.x + 14.0 + col as f32 * (rect.w / 3.0);
        let y = rect.y + 28.0 + row as f32 * 34.0;
        draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
        draw_ui_text(
            &value.to_string(),
            x + 66.0,
            y,
            FONT_BODY_SIZE,
            TEXT_HIGHLIGHT,
        );
    }
}

fn draw_equipment_line(
    x: f32,
    y: f32,
    width: f32,
    beast: &SpiritBeast,
    data: &GameData,
    slot: BeastEquipmentSlot,
) {
    let slot_name = beast_slot_label(&slot);
    let item_label = beast
        .equipment
        .iter()
        .find(|eq| eq.slot == slot)
        .and_then(|eq| data.beast_equipment_definitions.get(&eq.item_id))
        .map(|item| item.name.clone())
        .unwrap_or_else(|| "Empty".to_string());

    draw_ui_text(slot_name, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
    draw_ui_text(
        &item_label,
        x + width * 0.32,
        y,
        FONT_SMALL_SIZE,
        if item_label == "Empty" {
            TEXT_SECONDARY
        } else {
            TEXT_PRIMARY
        },
    );
}

fn beast_slots() -> [BeastEquipmentSlot; 4] {
    [
        BeastEquipmentSlot::Collar,
        BeastEquipmentSlot::Harness,
        BeastEquipmentSlot::Talisman,
        BeastEquipmentSlot::Relic,
    ]
}

fn beast_slot_label(slot: &BeastEquipmentSlot) -> &'static str {
    match slot {
        BeastEquipmentSlot::Collar => "Collar",
        BeastEquipmentSlot::Harness => "Harness",
        BeastEquipmentSlot::Talisman => "Talisman",
        BeastEquipmentSlot::Relic => "Relic",
    }
}

fn hunger_color(hunger: i32) -> Color {
    if hunger > 70 {
        FAILURE
    } else if hunger > 35 {
        WARNING
    } else {
        SUCCESS
    }
}

fn beast_element_color(element: &crate::data::elements::Element) -> Color {
    match element {
        crate::data::elements::Element::Wood => SUCCESS,
        crate::data::elements::Element::Fire => ACCENT,
        crate::data::elements::Element::Earth => PRIMARY,
        crate::data::elements::Element::Metal => WARNING,
        crate::data::elements::Element::Water => SECONDARY,
        crate::data::elements::Element::None => TEXT_HIGHLIGHT,
    }
}
