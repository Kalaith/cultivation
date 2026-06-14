use super::*;

impl DiscipleRosterState {
    pub(super) fn draw_law_modal(&mut self, data: &GameData, idx: usize) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 440.0;
        let modal_h = 420.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Jade Law Cabinet"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
            "X",
            false,
        ) {
            self.law_modal_open = false;
        }

        let mut m_y = modal_y + 58.0;
        for law in data.laws.values() {
            if draw_button(
                Rect::new(modal_x + 20.0, m_y, modal_w - 40.0, 36.0),
                &format!("{} [{:?}]", law.name, law.element),
                false,
            ) {
                self.law_modal_open = false;
                return Some(UpdateResult::new().with_action(
                    crate::engine::actions::Action::AssignLaw(idx, law.id.clone()),
                ));
            }
            m_y += 44.0;

            draw_wrapped_text(
                &law.description,
                modal_x + 26.0,
                m_y,
                modal_w - 52.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
            m_y += 36.0;
            if m_y > modal_y + modal_h - 40.0 {
                break;
            }
        }

        None
    }

    pub(super) fn draw_item_modal(
        &mut self,
        data: &GameData,
        idx: usize,
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 420.0;
        let modal_h = 400.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Pill and Talisman Shelf"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
            "X",
            false,
        ) {
            self.item_modal_open = false;
        }

        let mut i_y = modal_y + 58.0;
        let mut found_any = false;

        for (item_id, count) in inventory {
            if *count == 0 {
                continue;
            }
            let Some(item) = data.items.get(item_id) else {
                continue;
            };
            if item.equipment.is_some() {
                continue;
            }

            found_any = true;
            let row = Rect::new(modal_x + 20.0, i_y, modal_w - 40.0, 36.0);
            if draw_button(row, &format!("{} (x{})", item.name, count), false) {
                self.item_modal_open = false;
                return Some(UpdateResult::new().with_action(
                    crate::engine::actions::Action::UseItem(item_id.clone(), idx),
                ));
            }

            if row.contains(mouse_position().into()) {
                draw_tooltip(mouse_position().into(), &item.description);
            }

            i_y += 42.0;
            if i_y > modal_y + modal_h - 32.0 {
                break;
            }
        }

        if !found_any {
            draw_ui_text(
                "No usable pills, talismans, or supplies.",
                modal_x + 20.0,
                i_y,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        None
    }

    pub(super) fn draw_equip_modal(
        &mut self,
        data: &GameData,
        idx: usize,
        inventory: &std::collections::HashMap<String, u32>,
        disciples: &[Disciple],
    ) -> Option<UpdateResult> {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        let modal_w = 540.0;
        let modal_h = 520.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Artifact Binding"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
            "X",
            false,
        ) {
            self.equip_modal_open = false;
        }

        let Some(disciple) = disciples.get(idx) else {
            return None;
        };

        let mut y = modal_y + 58.0;
        draw_ui_text(
            "Bound Artifacts",
            modal_x + 20.0,
            y,
            FONT_BODY_SIZE,
            TEXT_HIGHLIGHT,
        );
        y += 22.0;

        for slot in equipment_slots().iter() {
            let slot_name = equipment_slot_name(slot);
            if let Some(item_id) = disciple.equipment.get(slot) {
                let item_name = data
                    .items
                    .get(item_id)
                    .map(|i| i.name.as_str())
                    .unwrap_or("Unknown");
                draw_ui_text(
                    &format!("{}: {}", slot_name, item_name),
                    modal_x + 20.0,
                    y,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );

                if draw_button(
                    Rect::new(modal_x + modal_w - 110.0, y - 14.0, 80.0, 24.0),
                    "Unbind",
                    false,
                ) {
                    self.equip_modal_open = false;
                    return Some(UpdateResult::new().with_action(
                        crate::engine::actions::Action::UnequipItem(slot.clone(), idx),
                    ));
                }
            } else {
                draw_ui_text(
                    &format!("{}: Empty", slot_name),
                    modal_x + 20.0,
                    y,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );
            }

            y += 22.0;
            if y > modal_y + 218.0 {
                break;
            }
        }

        y = modal_y + 250.0;
        draw_ui_text(
            "Unbound Inventory",
            modal_x + 20.0,
            y,
            FONT_BODY_SIZE,
            TEXT_HIGHLIGHT,
        );
        y += 22.0;

        let mut found_any = false;
        for (item_id, count) in inventory {
            if *count == 0 {
                continue;
            }
            let Some(item) = data.items.get(item_id) else {
                continue;
            };
            let Some(eq) = item.equipment.as_ref() else {
                continue;
            };

            found_any = true;
            let slot_name = equipment_slot_name(&eq.slot);
            let row = Rect::new(modal_x + 20.0, y, modal_w - 40.0, 30.0);
            if draw_button(
                row,
                &format!("{} ({} slot) x{}", item.name, slot_name, count),
                false,
            ) {
                self.equip_modal_open = false;
                return Some(UpdateResult::new().with_action(
                    crate::engine::actions::Action::EquipItem(item_id.clone(), idx),
                ));
            }

            if row.contains(mouse_position().into()) {
                draw_tooltip(mouse_position().into(), &item.description);
            }

            y += 36.0;
            if y > modal_y + modal_h - 32.0 {
                break;
            }
        }

        if !found_any {
            draw_ui_text(
                "No artifacts are ready for binding.",
                modal_x + 20.0,
                y,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        None
    }
}

fn equipment_slots() -> [crate::data::items::EquipmentSlot; 10] {
    [
        crate::data::items::EquipmentSlot::Weapon,
        crate::data::items::EquipmentSlot::OffHand,
        crate::data::items::EquipmentSlot::Chest,
        crate::data::items::EquipmentSlot::Legs,
        crate::data::items::EquipmentSlot::Arms,
        crate::data::items::EquipmentSlot::Head,
        crate::data::items::EquipmentSlot::Boots,
        crate::data::items::EquipmentSlot::Ring,
        crate::data::items::EquipmentSlot::Amulet,
        crate::data::items::EquipmentSlot::Belt,
    ]
}

fn equipment_slot_name(slot: &crate::data::items::EquipmentSlot) -> &'static str {
    match slot {
        crate::data::items::EquipmentSlot::Weapon => "Weapon",
        crate::data::items::EquipmentSlot::OffHand => "Off-hand",
        crate::data::items::EquipmentSlot::Chest => "Robe",
        crate::data::items::EquipmentSlot::Legs => "Leggings",
        crate::data::items::EquipmentSlot::Arms => "Bracers",
        crate::data::items::EquipmentSlot::Head => "Crown",
        crate::data::items::EquipmentSlot::Boots => "Cloud Boots",
        crate::data::items::EquipmentSlot::Ring => "Ring",
        crate::data::items::EquipmentSlot::Amulet => "Amulet",
        crate::data::items::EquipmentSlot::Belt => "Belt",
    }
}
