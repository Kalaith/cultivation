use super::super::Game;
use crate::data::spirit_beasts::{BeastEquipmentSlot, EquippedBeastItem};

impl Game {
    pub(in crate::game) fn handle_use_item(&mut self, item_id: String, disciple_idx: usize) {
        let count = *self.inventory.get(&item_id).unwrap_or(&0);
        if count == 0 {
            self.event_log
                .push("Item not found in inventory.".to_string());
            return;
        }

        let Some(disciple) = self.disciples.get_mut(disciple_idx) else {
            return;
        };
        let Some(item) = self.data.items.get(&item_id).cloned() else {
            return;
        };

        if item.equipment.is_some() {
            self.event_log.push(format!(
                "{} is equipment. Equip it from the roster.",
                item.name
            ));
            return;
        }

        let is_herb = item.item_type == crate::data::items::ItemType::Resource
            && self.data.herbs.contains_key(&item_id);
        let efficiency = if is_herb {
            crate::data::herbs::DIRECT_CONSUMPTION_EFFICIENCY
        } else {
            1.0
        };

        let mut logs: Vec<String> = Vec::new();
        for effect in &item.effects {
            logs.extend(Game::apply_item_effect(
                disciple, effect, &item, efficiency, is_herb,
            ));
        }

        if let Some(c) = self.inventory.get_mut(&item_id) {
            *c -= 1;
        }

        if let Some(crate::data::disciples::Bottleneck::UseItem(ref bn_id)) =
            disciple.breakthrough_bottleneck
        {
            if *bn_id == item_id {
                logs.push(format!(
                    "{} overcame their bottleneck: Use {}!",
                    disciple.name, item.name
                ));
                disciple.breakthrough_bottleneck = None;
            }
        }

        for log in logs {
            self.event_log.push(log);
        }
    }

    pub(in crate::game) fn handle_equip_item(&mut self, item_id: String, disciple_idx: usize) {
        if let Some(beast_item) = self.data.beast_equipment_definitions.get(&item_id) {
            if beast_item.beast_only {
                self.event_log
                    .push("Cannot equip beast-only items on disciples.".to_string());
                return;
            }
        }

        let count = *self.inventory.get(&item_id).unwrap_or(&0);
        if count == 0 {
            self.event_log
                .push("Cannot equip: item not in inventory.".to_string());
            return;
        }

        let Some(item) = self.data.items.get(&item_id).cloned() else {
            self.event_log
                .push("Cannot equip: unknown item.".to_string());
            return;
        };

        let Some(equipment) = item.equipment.clone() else {
            self.event_log
                .push(format!("{} cannot be equipped.", item.name));
            return;
        };

        let Some(disciple) = self.disciples.get_mut(disciple_idx) else {
            return;
        };

        if let Some(existing_id) = disciple.equipment.get(&equipment.slot).cloned() {
            if let Some(existing_item) = self.data.items.get(&existing_id) {
                if let Some(existing_eq) = existing_item.equipment.as_ref() {
                    Self::apply_equipment_modifiers(
                        &mut disciple.attributes,
                        &existing_eq.modifiers,
                        -1,
                    );
                }
            }
            *self.inventory.entry(existing_id.clone()).or_insert(0) += 1;
            disciple.equipment.remove(&equipment.slot);
        }

        Self::apply_equipment_modifiers(&mut disciple.attributes, &equipment.modifiers, 1);
        disciple
            .equipment
            .insert(equipment.slot.clone(), item_id.clone());

        if let Some(c) = self.inventory.get_mut(&item_id) {
            *c -= 1;
        }

        self.event_log
            .push(format!("{} equipped {}.", disciple.name, item.name));
    }

    pub(in crate::game) fn handle_unequip_item(
        &mut self,
        slot: crate::data::items::EquipmentSlot,
        disciple_idx: usize,
    ) {
        let Some(disciple) = self.disciples.get_mut(disciple_idx) else {
            return;
        };
        let Some(item_id) = disciple.equipment.get(&slot).cloned() else {
            return;
        };
        let Some(item) = self.data.items.get(&item_id) else {
            return;
        };

        if let Some(eq) = item.equipment.as_ref() {
            Self::apply_equipment_modifiers(&mut disciple.attributes, &eq.modifiers, -1);
        }
        *self.inventory.entry(item_id.clone()).or_insert(0) += 1;
        disciple.equipment.remove(&slot);
        self.event_log
            .push(format!("{} unequipped {}.", disciple.name, item.name));
    }

    pub(in crate::game) fn handle_repair_item(&mut self, item_id: String, disciple_idx: usize) {
        let Some(disciple) = self.disciples.get(disciple_idx) else {
            return;
        };
        let Some(item) = self.data.items.get(&item_id) else {
            self.event_log.push("Unknown item.".to_string());
            return;
        };

        if item.equipment.is_none() {
            self.event_log
                .push("Item has no durability to repair.".to_string());
            return;
        }

        if !disciple.equipment.values().any(|id| id == &item_id) {
            self.event_log.push("Item is not equipped.".to_string());
            return;
        }

        let quality_tier = item.quality.tier();
        let repair_cost = 10 + quality_tier * 10;

        if self.spirit_stones < repair_cost {
            self.event_log.push(format!(
                "Not enough Spirit Stones to repair ({} required).",
                repair_cost
            ));
            return;
        }

        self.spirit_stones -= repair_cost;
        self.event_log.push(format!(
            "Repaired {} for {} Spirit Stones.",
            item.name, repair_cost
        ));
    }

    pub(in crate::game) fn handle_equip_beast_item(&mut self, beast_id: u64, item_id: String) {
        let count = *self.inventory.get(&item_id).unwrap_or(&0);
        if count == 0 {
            self.event_log
                .push("Beast equipment not in inventory.".to_string());
            return;
        }

        let Some(item) = self.data.beast_equipment_definitions.get(&item_id) else {
            self.event_log
                .push("Unknown beast equipment item.".to_string());
            return;
        };

        if !item.beast_only {
            self.event_log
                .push("Item is not beast-only equipment.".to_string());
            return;
        }

        let Some(beast) = self.spirit_beasts.iter_mut().find(|b| b.id == beast_id) else {
            self.event_log.push("Spirit beast not found.".to_string());
            return;
        };

        if !beast.can_equip(item) {
            self.event_log
                .push("Beast cannot equip this item yet.".to_string());
            return;
        }

        if let Some(existing) = beast
            .equipment
            .iter()
            .find(|eq| eq.slot == item.slot)
            .cloned()
        {
            *self.inventory.entry(existing.item_id).or_insert(0) += 1;
            beast.equipment.retain(|eq| eq.slot != item.slot);
        }
        beast.equipment.push(EquippedBeastItem {
            item_id: item.id.clone(),
            slot: item.slot.clone(),
        });

        if let Some(inv) = self.inventory.get_mut(&item_id) {
            *inv = inv.saturating_sub(1);
        }

        let slot_label = match item.slot {
            BeastEquipmentSlot::Collar => "Collar",
            BeastEquipmentSlot::Harness => "Harness",
            BeastEquipmentSlot::Talisman => "Talisman",
            BeastEquipmentSlot::Relic => "Relic",
        };

        self.event_log.push(format!(
            "{} equipped {} ({})",
            beast.name, item.name, slot_label
        ));
    }
}
