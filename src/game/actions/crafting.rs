use super::super::Game;
use crate::game::moments::MomentKind;
use macroquad_toolkit::rng as game_rng;

impl Game {
    pub(in crate::game) fn handle_craft_item(&mut self, recipe_id: String) {
        use crate::engine::crafting::{self, CraftingContext};

        let Some(recipe) = self
            .data
            .recipes
            .iter()
            .find(|r| r.id == recipe_id)
            .cloned()
        else {
            return;
        };

        if !self.has_ingredients(&recipe.ingredients) {
            self.event_log
                .push("Workshop lacks the offerings for this formula.".to_string());
            return;
        }

        let building = self.data.buildings.iter().find(|b| {
            b.building_type == recipe.required_building
                && b.status == crate::data::buildings::BuildingStatus::Active
        });

        let building_level = building.map(|b| b.level).unwrap_or(1);
        let building_element = building.map(|b| b.element.clone()).unwrap_or_default();

        let disciple_mind = building
            .and_then(|b| b.assigned_disciple)
            .and_then(|did| self.disciples.iter().find(|d| d.id == did))
            .map(|d| d.attributes.mind);

        let ctx = CraftingContext {
            building_level,
            building_element,
            disciple_mind,
        };

        let roll = game_rng::gen_range(0, 100u32);
        let result = crafting::calculate_craft(&recipe, &ctx, roll);

        if result.success {
            self.deduct_ingredients(&recipe.ingredients, 1.0);
            *self
                .inventory
                .entry(recipe.output_item_id.clone())
                .or_insert(0) += result.output_amount;
            self.show_moment(
                MomentKind::Discovery,
                "Refinement Succeeds",
                "Pill flame steadies",
                format!(
                    "{} has produced {}x {}, strengthening the sect stores.",
                    recipe.name, result.output_amount, recipe.output_item_id
                ),
            );

            for disciple in &mut self.disciples {
                if let Some(crate::data::disciples::Bottleneck::CraftItem(ref bn_recipe)) =
                    disciple.breakthrough_bottleneck
                {
                    if *bn_recipe == recipe_id {
                        self.event_log.push(format!(
                            "{} fulfilled a fate trial: refine {}!",
                            disciple.name, recipe.name
                        ));
                        disciple.breakthrough_bottleneck = None;
                    }
                }
            }
        } else {
            self.deduct_ingredients(&recipe.ingredients, result.materials_lost_fraction);
        }

        self.event_log.push(result.message);
    }

    fn has_ingredients(&self, ingredients: &[(String, u32)]) -> bool {
        for (ing_id, amount) in ingredients {
            let has = self.get_ingredient_count(ing_id);
            if has < *amount {
                return false;
            }
        }
        true
    }

    fn get_ingredient_count(&self, ing_id: &str) -> u32 {
        match ing_id {
            "spirit_stones" => self.spirit_stones,
            "herbs" => self.herbs,
            _ => *self.inventory.get(ing_id).unwrap_or(&0),
        }
    }

    fn deduct_ingredients(&mut self, ingredients: &[(String, u32)], fraction: f32) {
        for (ing_id, amount) in ingredients {
            let lost = (*amount as f32 * fraction).ceil() as u32;
            match ing_id.as_str() {
                "spirit_stones" => self.spirit_stones = self.spirit_stones.saturating_sub(lost),
                "herbs" => self.herbs = self.herbs.saturating_sub(lost),
                _ => {
                    if let Some(count) = self.inventory.get_mut(ing_id) {
                        *count = count.saturating_sub(lost);
                    }
                }
            }
        }
    }

    pub(in crate::game) fn handle_research_tech(&mut self, tech_id: String) {
        if self.unlocked_techs.contains(&tech_id) {
            return;
        }

        let Some(tech) = self.data.techs.get(&tech_id) else {
            return;
        };

        let prereqs_met = tech
            .prerequisites
            .iter()
            .all(|p| self.unlocked_techs.contains(p));
        if !prereqs_met {
            self.event_log
                .push("Earlier doctrine slips are still sealed.".to_string());
            return;
        }

        if self.spirit_stones < tech.cost_spirit_stones {
            self.event_log
                .push("The treasury lacks spirit stones for doctrine recovery.".to_string());
            return;
        }

        self.spirit_stones -= tech.cost_spirit_stones;
        self.unlocked_techs.push(tech_id.clone());
        let tech_name = tech.name.clone();
        self.event_log
            .push(format!("Doctrine recovered: {}", tech_name));
        self.show_moment(
            MomentKind::Discovery,
            "A Jade Slip Opens",
            "Sect knowledge deepens",
            format!(
                "{} has been understood and entered into the patriarch's archive.",
                tech_name
            ),
        );
    }
}
