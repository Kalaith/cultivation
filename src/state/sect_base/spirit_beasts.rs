use super::*;

impl SectBaseState {
    pub(super) fn draw_spirit_beasts_view(
        &mut self,
        rect: Rect,
        data: &GameData,
        spirit_beasts: &[SpiritBeast],
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        draw_panel(rect, Some("Spirit Beasts"));

        let header_y = rect.y + 40.0;
        draw_text(
            &format!("Owned: {}", spirit_beasts.len()),
            rect.x + 20.0,
            header_y,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );

        let left_w = 280.0;
        let list_rect = Rect::new(rect.x + 10.0, header_y + 30.0, left_w, rect.h - 90.0);
        draw_panel(list_rect, Some("Roster"));

        let mut y = list_rect.y + 40.0;
        for (i, beast) in spirit_beasts.iter().enumerate() {
            let selected = self.selected_beast_index == Some(i);
            let label = format!("{} ({})", beast.name, beast.species);
            if draw_button(
                Rect::new(list_rect.x + 10.0, y, left_w - 20.0, 32.0),
                &label,
                selected,
            ) {
                self.selected_beast_index = Some(i);
            }
            y += 38.0;
            if y > list_rect.y + list_rect.h - 40.0 {
                break;
            }
        }

        let right_rect = Rect::new(
            list_rect.x + left_w + 10.0,
            list_rect.y,
            rect.w - left_w - 30.0,
            list_rect.h,
        );
        draw_panel(right_rect, Some("Details"));

        if let Some(idx) = self
            .selected_beast_index
            .and_then(|i| spirit_beasts.get(i).map(|_| i))
        {
            self.draw_beast_details_panel(right_rect, &spirit_beasts[idx], data);
        }

        if self.beast_equip_modal_open {
            if let Some(idx) = self
                .selected_beast_index
                .and_then(|i| spirit_beasts.get(i).map(|_| i))
            {
                if let Some(res) =
                    self.draw_beast_equip_modal(data, spirit_beasts[idx].id, inventory)
                {
                    return Some(res);
                }
            } else {
                self.beast_equip_modal_open = false;
            }
        }

        if draw_button(
            Rect::new(rect.x + rect.w - 120.0, rect.y + 10.0, 100.0, 30.0),
            "Back",
            false,
        ) {
            self.view = SectView::Map;
        }

        None
    }

    fn draw_beast_details_panel(&self, right_rect: Rect, beast: &SpiritBeast, data: &GameData) {
        let mut dy = right_rect.y + 40.0;
        draw_text(
            &beast.name,
            right_rect.x + 20.0,
            dy,
            FONT_HEADER_SIZE,
            PRIMARY,
        );
        dy += 24.0;
        draw_text(
            &format!("Species: {}", beast.species),
            right_rect.x + 20.0,
            dy,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        dy += 20.0;
        draw_text(
            &format!("Tier: {:?}", beast.tier),
            right_rect.x + 20.0,
            dy,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        dy += 20.0;
        draw_text(
            &format!("Loyalty: {}", beast.loyalty),
            right_rect.x + 20.0,
            dy,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        dy += 20.0;
        draw_text(
            &format!("Hunger: {}", beast.hunger),
            right_rect.x + 20.0,
            dy,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        dy += 30.0;

        draw_text(
            "Equipment:",
            right_rect.x + 20.0,
            dy,
            FONT_SMALL_SIZE,
            TEXT_PRIMARY,
        );
        dy += 18.0;

        let slots = [
            crate::data::spirit_beasts::BeastEquipmentSlot::Collar,
            crate::data::spirit_beasts::BeastEquipmentSlot::Harness,
            crate::data::spirit_beasts::BeastEquipmentSlot::Talisman,
            crate::data::spirit_beasts::BeastEquipmentSlot::Relic,
        ];

        for slot in slots.iter() {
            let slot_name = match slot {
                crate::data::spirit_beasts::BeastEquipmentSlot::Collar => "Collar",
                crate::data::spirit_beasts::BeastEquipmentSlot::Harness => "Harness",
                crate::data::spirit_beasts::BeastEquipmentSlot::Talisman => "Talisman",
                crate::data::spirit_beasts::BeastEquipmentSlot::Relic => "Relic",
            };

            let item_label = beast
                .equipment
                .iter()
                .find(|eq| eq.slot == *slot)
                .and_then(|eq| data.beast_equipment_definitions.get(&eq.item_id))
                .map(|item| item.name.clone())
                .unwrap_or_else(|| "Empty".to_string());

            draw_text(
                &format!("{}: {}", slot_name, item_label),
                right_rect.x + 30.0,
                dy,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
            dy += 18.0;
        }

        if draw_button(
            Rect::new(
                right_rect.x + 20.0,
                right_rect.y + right_rect.h - 60.0,
                180.0,
                36.0,
            ),
            "Equip Beast Gear",
            false,
        ) {
            // Note: can't mutate self here since &self, handled in caller
        }
    }

    pub(super) fn draw_beast_equip_modal(
        &mut self,
        data: &GameData,
        beast_id: u64,
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 420.0;
        let modal_h = 420.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Beast Equipment"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
            "X",
            false,
        ) {
            self.beast_equip_modal_open = false;
        }

        let mut y = modal_y + 50.0;
        let mut items: Vec<_> = data.beast_equipment_definitions.values().collect();
        items.sort_by_key(|i| i.name.clone());

        let mut found_any = false;
        for item in items {
            let count = *inventory.get(&item.id).unwrap_or(&0);
            if count == 0 {
                continue;
            }
            found_any = true;
            let label = format!("{} ({:?}) x{}", item.name, item.slot, count);
            if draw_button(
                Rect::new(modal_x + 20.0, y, modal_w - 40.0, 32.0),
                &label,
                false,
            ) {
                self.beast_equip_modal_open = false;
                return Some(
                    UpdateResult::new()
                        .with_action(Action::EquipBeastItem(beast_id, item.id.clone())),
                );
            }
            y += 36.0;
            if y > modal_y + modal_h - 40.0 {
                break;
            }
        }

        if !found_any {
            draw_text(
                "No beast equipment in inventory.",
                modal_x + 20.0,
                modal_y + 80.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        None
    }
}
