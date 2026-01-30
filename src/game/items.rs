use super::Game;
use crate::data::disciples::Disciple;

impl Game {
    pub(super) fn apply_item_effect(
        disciple: &mut Disciple,
        effect: &crate::data::items::ItemEffect,
        item: &crate::data::items::Item,
        efficiency: f32,
        is_herb: bool,
    ) -> Vec<String> {
        let mut logs = Vec::new();

        match effect {
            crate::data::items::ItemEffect::Heal(amt) => {
                let effective_amt = ((*amt as f32) * efficiency) as u32;
                if disciple.is_injured() {
                    let before_ticks = disciple
                        .injury
                        .as_ref()
                        .map(|i| i.recovery_ticks_remaining)
                        .unwrap_or(0);
                    disciple.apply_healing(effective_amt);
                    if disciple.is_injured() {
                        let after_ticks = disciple
                            .injury
                            .as_ref()
                            .map(|i| i.recovery_ticks_remaining)
                            .unwrap_or(0);
                        logs.push(format!(
                            "{} used {} on {}. Recovery time reduced by {} ticks ({} remaining).",
                            item.name,
                            effective_amt,
                            disciple.name,
                            before_ticks - after_ticks,
                            after_ticks
                        ));
                    } else {
                        logs.push(format!(
                            "{} fully healed {} with {}!",
                            disciple.name,
                            disciple.name,
                            item.name
                        ));
                    }
                } else {
                    logs.push(format!(
                        "{} is not injured. {} had no effect.",
                        disciple.name,
                        item.name
                    ));
                }
            }
            crate::data::items::ItemEffect::BoostQi(amt) => {
                let effective_amt = ((*amt as f32) * efficiency) as u32;
                disciple.exp += effective_amt;
                if is_herb {
                    logs.push(format!(
                        "{} gained {} Cultivation XP from {} (50% herb efficiency).",
                        disciple.name,
                        effective_amt,
                        item.name
                    ));
                } else {
                    logs.push(format!(
                        "{} gained {} Cultivation XP from {}.",
                        disciple.name,
                        effective_amt,
                        item.name
                    ));
                }
            }
            crate::data::items::ItemEffect::BoostBody(amt) => {
                logs.push(Self::apply_attribute_boost(disciple, "Body", *amt, efficiency));
            }
            crate::data::items::ItemEffect::BoostMind(amt) => {
                logs.push(Self::apply_attribute_boost(disciple, "Mind", *amt, efficiency));
            }
            crate::data::items::ItemEffect::BoostSpirit(amt) => {
                logs.push(Self::apply_attribute_boost(disciple, "Spirit", *amt, efficiency));
            }
            crate::data::items::ItemEffect::TemporaryBuff { stat, value, duration_ticks } => {
                // For now, apply as a permanent stat boost (temporary buff system would need a tick tracker)
                let effective_amt = ((*value as f32) * efficiency).max(1.0) as u32;
                logs.push(Self::apply_attribute_boost(disciple, stat, effective_amt, 1.0));
                logs.push(format!(
                    "{} received a temporary {} buff (+{} for {} ticks).",
                    disciple.name, stat, value, duration_ticks
                ));
            }
            crate::data::items::ItemEffect::DamageShield(amt) => {
                logs.push(format!(
                    "{} activated a damage shield ({} points) from {}!",
                    disciple.name, amt, item.name
                ));
            }
            crate::data::items::ItemEffect::QiBurst(amt) => {
                let effective_amt = ((*amt as f32) * efficiency) as u32;
                disciple.exp += effective_amt;
                logs.push(format!(
                    "{} gained {} Cultivation XP from Qi Burst ({}).",
                    disciple.name, effective_amt, item.name
                ));
            }
            crate::data::items::ItemEffect::ElementalStrike(element, amt) => {
                logs.push(format!(
                    "{} unleashed a {:?} elemental strike ({} damage) from {}!",
                    disciple.name, element, amt, item.name
                ));
            }
            crate::data::items::ItemEffect::MissionSpeedBoost(amt) => {
                logs.push(format!(
                    "{} gained mission speed boost ({} ticks) from {}.",
                    disciple.name, amt, item.name
                ));
            }
        }

        logs
    }

    fn apply_attribute_boost(
        disciple: &mut Disciple,
        label: &str,
        amount: u32,
        efficiency: f32,
    ) -> String {
        let effective_amt = ((amount as f32) * efficiency).max(1.0) as u32;
        match label {
            "Body" => disciple.attributes.body += effective_amt,
            "Mind" => disciple.attributes.mind += effective_amt,
            "Spirit" => disciple.attributes.spirit += effective_amt,
            _ => {}
        }
        format!("{}'s {} increased by {}!", disciple.name, label, effective_amt)
    }

    pub(super) fn apply_equipment_modifiers(
        attributes: &mut crate::data::disciples::Attributes,
        modifiers: &[crate::data::items::StatModifier],
        direction: i32,
    ) {
        fn apply_stat(target: &mut u32, delta: i32) {
            let new_val = (*target as i32 + delta).max(0) as u32;
            *target = new_val;
        }

        for modifier in modifiers {
            let delta = modifier.value * direction;
            match modifier.stat.to_lowercase().as_str() {
                "body" => apply_stat(&mut attributes.body, delta),
                "mind" => apply_stat(&mut attributes.mind, delta),
                "spirit" => apply_stat(&mut attributes.spirit, delta),
                _ => {}
            }
        }
    }
}