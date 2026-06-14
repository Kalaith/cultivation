use super::super::Game;
use crate::data::buildings::BuildingType;
use crate::data::disciples::DiscipleRank;
use crate::engine::random;
use crate::game::moments::MomentKind;

impl Game {
    pub(in crate::game) fn handle_upgrade_building(&mut self, building_type: BuildingType) {
        if !self.disciples.iter().any(|d| d.rank == DiscipleRank::Outer) {
            self.event_log
                .push("Cannot Build: No Outer Disciples available to work!".to_string());
            return;
        }

        let cost = 50;
        if self.spirit_stones < cost {
            self.event_log.push("Not enough Spirit Stones.".to_string());
            return;
        }

        let Some(building) = self
            .data
            .buildings
            .iter_mut()
            .find(|b| b.building_type == building_type)
        else {
            return;
        };

        self.spirit_stones -= cost;
        building.level += 1;
        self.event_log.push(format!(
            "Upgraded {:?} to Lv {}",
            building.building_type, building.level
        ));
    }

    pub(in crate::game) fn handle_repair_building(&mut self, id: u64) {
        let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == id) else {
            return;
        };

        if building.status != crate::data::buildings::BuildingStatus::Ruined {
            return;
        }

        let cost = building.repair_cost;
        if self.spirit_stones < cost {
            self.event_log.push(format!(
                "Not enough Spirit Stones to repair ({} required).",
                cost
            ));
            return;
        }

        self.spirit_stones -= cost;
        let building_type = building.building_type.clone();
        building.status = crate::data::buildings::BuildingStatus::Active;
        self.event_log
            .push(format!("Repaired {}!", building.building_type));
        self.show_moment(
            MomentKind::Founding,
            "A Hall Rises From Ruin",
            "Sect grounds restored",
            format!(
                "{} has been restored. Incense smoke climbs again from the fallen mountain.",
                building_type
            ),
        );
    }

    pub(in crate::game) fn handle_construct_building(
        &mut self,
        b_type: BuildingType,
        x: i32,
        y: i32,
    ) {
        let def = self.data.building_definitions.get(&b_type);
        let cost = def.map(|d| d.cost).unwrap_or(100);
        let is_unique = def.map(|d| d.unique).unwrap_or(false);
        let element = def.map(|d| d.element.clone()).unwrap_or_default();

        if is_unique
            && self
                .data
                .buildings
                .iter()
                .any(|b| b.building_type == b_type)
        {
            self.event_log
                .push(format!("Cannot build: {} already exists.", b_type));
            return;
        }

        if self.spirit_stones < cost {
            self.event_log
                .push(format!("Not enough Spirit Stones ({} required).", cost));
            return;
        }

        self.spirit_stones -= cost;
        let mut new_b = crate::data::buildings::Building::new(b_type.clone());
        new_b.id = random::next_u64();
        new_b.x = x;
        new_b.y = y;
        new_b.element = element;
        if let Some(def) = def {
            new_b.repair_cost = def.repair_cost;
        }
        new_b.status = crate::data::buildings::BuildingStatus::Active;
        self.data.buildings.push(new_b);
        self.event_log
            .push(format!("Constructed {} at {},{}", b_type, x, y));
        self.show_moment(
            MomentKind::Founding,
            "New Hall Raised",
            "The mountain grows stronger",
            format!(
                "{} now stands among the sect grounds, a promise carved into stone and jade.",
                b_type
            ),
        );

        let new_recipes: Vec<String> = self
            .data
            .recipes
            .iter()
            .filter(|r| r.required_building == b_type && !self.discovered_recipes.contains(&r.id))
            .map(|r| r.id.clone())
            .collect();
        for recipe_id in &new_recipes {
            if let Some(recipe) = self.data.recipes.iter().find(|r| r.id == *recipe_id) {
                let recipe_name = recipe.name.clone();
                self.event_log
                    .push(format!("Discovered recipe: {}!", recipe_name));
                self.show_moment(
                    MomentKind::Discovery,
                    "A Forgotten Method Surfaces",
                    "Recipe discovered",
                    format!(
                        "{} has been copied into the sect archive for future refinement.",
                        recipe_name
                    ),
                );
            }
            self.discovered_recipes.push(recipe_id.clone());
        }
    }
}
