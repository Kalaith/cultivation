use super::{BreakthroughResult, Game};
use crate::data::buildings::BuildingType;
use crate::data::disciples::DiscipleRank;
use crate::data::history::DeceasedDisciple;
use crate::data::missions::{MissionOutcome, OngoingMission};
use crate::data::spirit_beasts::{BeastEquipmentSlot, EquippedBeastItem, SpiritBeast};
use crate::engine::actions::Action;
use crate::engine::proc_gen::generate_disciple;
use crate::engine::tribulation::TribulationState;
use crate::engine::world_sim::WorldSim;
use crate::state::StateTransition;

impl Game {
    pub(super) fn execute_action(&mut self, action: Action) {
        match action {
            Action::UpgradeBuilding(building_type) => self.handle_upgrade_building(building_type),
            Action::RecruitDisciple => self.handle_recruit_disciple(),
            Action::PromoteDisciple(idx) => self.handle_promote_disciple(idx),
            Action::AttemptBreakthrough(idx) => self.handle_attempt_breakthrough_action(idx),
            Action::StartMission(mission_desc, disciple_indices) => {
                self.handle_start_mission(mission_desc, disciple_indices)
            }
            Action::ClaimRewards(outcome) => self.handle_claim_rewards(outcome),
            Action::SaveGame => self.handle_save_game(),
            Action::LoadGame => self.handle_load_game(),
            Action::AssignLaw(disciple_idx, law_id) => self.handle_assign_law(disciple_idx, law_id),
            Action::CraftItem(recipe_id) => self.handle_craft_item(recipe_id),
            Action::UseItem(item_id, disciple_idx) => self.handle_use_item(item_id, disciple_idx),
            Action::EquipItem(item_id, disciple_idx) => self.handle_equip_item(item_id, disciple_idx),
            Action::UnequipItem(slot, disciple_idx) => self.handle_unequip_item(slot, disciple_idx),
            Action::RepairItem(item_id, disciple_idx) => self.handle_repair_item(item_id, disciple_idx),
            Action::ResearchTech(tech_id) => self.handle_research_tech(tech_id),
            Action::RepairBuilding(id) => self.handle_repair_building(id),
            Action::ConstructBuilding(b_type, x, y) => self.handle_construct_building(b_type, x, y),
            Action::StartNewGame(name) => self.handle_start_new_game(name),
            Action::PlantHerb(building_id, plot_index, herb_id) => {
                self.plant_herb(building_id, plot_index, &herb_id);
            }
            Action::AssignDiscipleToBuilding(building_id, disciple_id) => {
                self.assign_disciple_to_building(building_id, disciple_id);
            }
            Action::ProcessDryingPavilion(building_id, herb_id) => {
                self.process_drying(building_id, &herb_id);
            }
            Action::SetGreenhouseInfusion(building_id, element) => {
                self.set_greenhouse_infusion(building_id, element);
            }
            Action::SendDiplomat { faction_id, action } => {
                self.handle_send_diplomat(faction_id, action)
            }
            Action::RespondToEvent { event_id, choice_idx } => {
                self.handle_respond_to_event(event_id, choice_idx)
            }
            Action::RecruitSpiritBeast(def_id) => self.handle_recruit_spirit_beast(def_id),
            Action::EquipBeastItem(beast_id, item_id) => self.handle_equip_beast_item(beast_id, item_id),
        }
    }

    fn handle_recruit_spirit_beast(&mut self, def_id: String) {
        if let Some(def) = self.data.spirit_beast_definitions.get(&def_id) {
            let mut beast = SpiritBeast::new_from_definition(def);
            beast.id = rand::random();
            self.spirit_beasts.push(beast);
            self.event_log
                .push(format!("Recruited Spirit Beast: {}", def.name));
        } else {
            self.event_log
                .push("Failed to recruit Spirit Beast: Unknown definition.".to_string());
        }
    }

    fn handle_upgrade_building(&mut self, building_type: BuildingType) {
        // Feature 9.1.3: Restrict Building to Outer Disciples
        let has_outer_workers = self.disciples.iter().any(|d| d.rank == DiscipleRank::Outer);
        if !has_outer_workers {
            self.event_log
                .push("Cannot Build: No Outer Disciples available to work!".to_string());
        } else {
            let cost = 50;
            if self.spirit_stones >= cost {
                if let Some(building) = self
                    .data
                    .buildings
                    .iter_mut()
                    .find(|b| b.building_type == building_type)
                {
                    self.spirit_stones -= cost;
                    building.level += 1;
                    self.event_log
                        .push(format!("Upgraded {:?} to Lv {}", building.building_type, building.level));
                }
            } else {
                self.event_log.push("Not enough Spirit Stones.".to_string());
            }
        }
    }

    fn handle_recruit_disciple(&mut self) {
        let capacity = self.get_population_capacity();
        if capacity == 0 || self.disciples.len() as u32 >= capacity {
            self.event_log
                .push("Population cap reached. Build Dormitories or upgrade the Sect Hall.".to_string());
        } else {
            let new_disciple = generate_disciple(&self.data);
            self.event_log.push(format!("Recruited: {}", new_disciple.name));
            self.disciples.push(new_disciple);
        }
    }

    fn handle_promote_disciple(&mut self, idx: usize) {
        if let Some(disciple) = self.disciples.get_mut(idx) {
            if disciple.rank == DiscipleRank::Outer {
                let cost = 100;
                if self.spirit_stones >= cost {
                    if disciple.realm != "Mortal" {
                        self.spirit_stones -= cost;
                        disciple.rank = DiscipleRank::Inner;
                        // Initialize Qi if 0 (should already be set by breakthrough but ensure here)
                        if disciple.max_qi == 0 {
                            disciple.max_qi = 100;
                            disciple.qi = 100;
                        }

                        self.event_log
                            .push(format!("Promoted {} to Inner Disciple!", disciple.name));
                    } else {
                        self.event_log.push("Cannot Promote: Must reach Qi Refinement first.".to_string());
                    }
                } else {
                    self.event_log
                        .push("Cannot Promote: Not enough Spirit Stones (100).".to_string());
                }
            }
        }
    }

    fn handle_attempt_breakthrough_action(&mut self, idx: usize) {
        if let Some(disciple) = self.disciples.get(idx).cloned() {
            if !disciple.can_attempt_breakthrough() {
                if disciple.is_injured() {
                    self.event_log.push(format!(
                        "{} cannot attempt breakthrough while injured!",
                        disciple.name
                    ));
                } else {
                    self.event_log
                        .push(format!("{} is not ready for breakthrough.", disciple.name));
                }
            } else {
                let mut disciple = disciple;
                let result = self.attempt_breakthrough(&mut disciple, idx);

                match result {
                    BreakthroughResult::Failure => {
                        // Record in hall of fallen
                        self.deceased_disciples.push(DeceasedDisciple::new(
                            disciple.name.clone(),
                            disciple.realm.clone(),
                            "Failed Breakthrough".to_string(),
                            self.tick,
                        ));
                        self.disciples.remove(idx);
                    }
                    BreakthroughResult::Tribulation(t_type) => {
                        // Update disciple and trigger tribulation
                        self.disciples[idx] = disciple.clone();
                        let trib_state = TribulationState::new(t_type, &disciple);
                        self.transition(StateTransition::ToTribulation(trib_state, idx));
                    }
                    BreakthroughResult::Blocked => {
                        // Blocked by progression requirements (no state changes)
                    }
                    _ => {
                        // Success or Injured - update disciple and reset readiness
                        disciple.breakthrough_readiness = 0.0;
                        self.disciples[idx] = disciple;
                    }
                }
            }
        }
    }

    fn handle_start_mission(&mut self, mission_desc: String, disciple_indices: Vec<usize>) {
        if let Some(mission) = self.data.missions.iter().find(|m| m.description == mission_desc) {
            self.event_log.push(format!("Mission Started: {}", mission.description));
            self.ongoing_missions.push(OngoingMission {
                mission: mission.clone(),
                disciple_indices,
                ticks_remaining: mission.duration,
            });
        }
    }

    fn handle_claim_rewards(&mut self, outcome: MissionOutcome) {
        if outcome.success {
            self.spirit_stones += outcome.rewards.spirit_stones;
            self.herbs += outcome.rewards.herbs;
            self.influence += outcome.rewards.influence;
            self.relics += outcome.rewards.relics;

            // Add Item Rewards
            for (item_id, amount) in outcome.rewards.items {
                *self.inventory.entry(item_id.clone()).or_insert(0) += amount;
                self.event_log.push(format!("Received {}x Item '{}'", amount, item_id));
            }

            // Grant XP to disciples involved
            for &idx in &outcome.disciple_indices {
                if let Some(disciple) = self.disciples.get_mut(idx) {
                    disciple.exp += outcome.rewards.disciple_exp;
                }
            }

            // Discover recipes from mission rewards
            for recipe_id in &outcome.rewards.recipe_discoveries {
                if !self.discovered_recipes.contains(recipe_id) {
                    self.discovered_recipes.push(recipe_id.clone());
                    if let Some(recipe) = self.data.recipes.iter().find(|r| r.id == *recipe_id) {
                        self.event_log.push(format!("Discovered recipe: {}!", recipe.name));
                    }
                }
            }

            self.event_log.push(format!(
                "Mission Rewards Claimed: {} SS, {} XP",
                outcome.rewards.spirit_stones,
                outcome.rewards.disciple_exp
            ));

            // Record history
            self.completed_history.push(outcome.description.clone());

            // Check for CompleteMission bottleneck clearing on mission party disciples
            for &idx in &outcome.disciple_indices {
                if let Some(disciple) = self.disciples.get_mut(idx) {
                    if let Some(crate::data::disciples::Bottleneck::CompleteMission(ref mt)) = disciple.breakthrough_bottleneck {
                        // Check if the completed mission matches the bottleneck type
                        let mission_matches = self.data.missions.iter().any(|m| {
                            m.description == outcome.description && match (&m.mission_type, mt.as_str()) {
                                (crate::data::missions::MissionType::Exploration, "Exploration") => true,
                                (crate::data::missions::MissionType::ResourceGathering, "ResourceGathering") => true,
                                (crate::data::missions::MissionType::MonsterSuppression, "MonsterSuppression") => true,
                                (crate::data::missions::MissionType::Diplomacy, "Diplomacy") => true,
                                (crate::data::missions::MissionType::RuinDelve, "RuinDelve") => true,
                                _ => false,
                            }
                        });
                        if mission_matches {
                            self.event_log.push(format!(
                                "{} overcame their bottleneck: Complete a {} mission!",
                                disciple.name, mt
                            ));
                            disciple.breakthrough_bottleneck = None;
                        }
                    }
                }
            }
        }
    }

    fn handle_save_game(&mut self) {
        self.save();
        self.event_log.push("Game saved successfully.".to_string());
    }

    fn handle_load_game(&mut self) {
        let loaded_game = self.load();
        if let Some(mut game) = loaded_game {
            // Preserve loaded buildings before restoring base data definitions
            let loaded_buildings = game.data.buildings.clone();

            // Restore base data definitions (stages, techs, items, etc.) that aren't saved
            game.data = self.data.clone();

            // Restore the loaded buildings
            game.data.buildings = loaded_buildings;

            *self = game;
            self.event_log.push("Game loaded successfully.".to_string());
        }
    }

    fn handle_assign_law(&mut self, disciple_idx: usize, law_id: String) {
        if let Some(disciple) = self.disciples.get_mut(disciple_idx) {
            if self.data.laws.contains_key(&law_id) {
                disciple.law_id = Some(law_id.clone());
                self.event_log.push(format!("{} is now practicing {}.", disciple.name, law_id));
            }
        }
    }

    fn handle_craft_item(&mut self, recipe_id: String) {
        use crate::engine::crafting::{self, CraftingContext};

        // Find recipe
        let Some(recipe) = self.data.recipes.iter().find(|r| r.id == recipe_id).cloned() else {
            return;
        };

        // Check ingredients
        let mut can_craft = true;
        for (ing_id, amount) in &recipe.ingredients {
            match ing_id.as_str() {
                "spirit_stones" => {
                    if self.spirit_stones < *amount {
                        can_craft = false;
                        break;
                    }
                }
                "herbs" => {
                    if self.herbs < *amount {
                        can_craft = false;
                        break;
                    }
                }
                _ => {
                    let current_amount = *self.inventory.get(ing_id).unwrap_or(&0);
                    if current_amount < *amount {
                        can_craft = false;
                        break;
                    }
                }
            }
        }

        if !can_craft {
            self.event_log.push("Cannot Craft: Missing ingredients.".to_string());
            return;
        }

        // Find the building for this recipe and get its level/element
        let building = self.data.buildings.iter().find(|b| {
            b.building_type == recipe.required_building
                && b.status == crate::data::buildings::BuildingStatus::Active
        });

        let building_level = building.map(|b| b.level).unwrap_or(1);
        let building_element = building.map(|b| b.element.clone()).unwrap_or_default();

        // Find assigned disciple's mind stat
        let disciple_mind = building
            .and_then(|b| b.assigned_disciple)
            .and_then(|did| self.disciples.iter().find(|d| d.id == did))
            .map(|d| d.attributes.mind);

        let ctx = CraftingContext {
            building_level,
            building_element,
            disciple_mind,
        };

        let roll = (rand::random::<u32>()) % 100;
        let result = crafting::calculate_craft(&recipe, &ctx, roll);

        if result.success {
            // Deduct all ingredients on success
            for (ing_id, amount) in &recipe.ingredients {
                match ing_id.as_str() {
                    "spirit_stones" => self.spirit_stones -= amount,
                    "herbs" => self.herbs -= amount,
                    _ => {
                        if let Some(count) = self.inventory.get_mut(ing_id) {
                            *count -= amount;
                        }
                    }
                }
            }

            // Add output
            *self.inventory.entry(recipe.output_item_id.clone()).or_insert(0) += result.output_amount;

            // Check for CraftItem bottleneck clearing on all disciples
            for disciple in &mut self.disciples {
                if let Some(crate::data::disciples::Bottleneck::CraftItem(ref bn_recipe)) = disciple.breakthrough_bottleneck {
                    if *bn_recipe == recipe_id {
                        self.event_log.push(format!(
                            "{} overcame their bottleneck: Craft {}!",
                            disciple.name, recipe.name
                        ));
                        disciple.breakthrough_bottleneck = None;
                    }
                }
            }
        } else {
            // Failure: consume 50% of ingredients (rounded up)
            for (ing_id, amount) in &recipe.ingredients {
                let lost = (*amount as f32 * result.materials_lost_fraction).ceil() as u32;
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

        self.event_log.push(result.message);
    }

    fn handle_repair_item(&mut self, item_id: String, disciple_idx: usize) {
        let Some(disciple) = self.disciples.get(disciple_idx) else {
            return;
        };

        let Some(item) = self.data.items.get(&item_id) else {
            self.event_log.push("Unknown item.".to_string());
            return;
        };

        let Some(equipment) = &item.equipment else {
            self.event_log.push("Item has no durability to repair.".to_string());
            return;
        };

        // Check disciple has this item equipped
        let is_equipped = disciple.equipment.values().any(|id| id == &item_id);
        if !is_equipped {
            self.event_log.push("Item is not equipped.".to_string());
            return;
        }

        // Repair cost based on quality tier
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

    fn handle_use_item(&mut self, item_id: String, disciple_idx: usize) {
        // Check if we have the item
        let count = *self.inventory.get(&item_id).unwrap_or(&0);
        if count > 0 {
            if let Some(disciple) = self.disciples.get_mut(disciple_idx) {
                if let Some(item) = self.data.items.get(&item_id).cloned() {
                    if item.equipment.is_some() {
                        self.event_log.push(format!(
                            "{} is equipment. Equip it from the roster.",
                            item.name
                        ));
                        return;
                    }
                    // Determine efficiency: herbs (Resources) have 50% efficiency when consumed directly
                    let is_herb = item.item_type == crate::data::items::ItemType::Resource
                        && self.data.herbs.contains_key(&item_id);
                    let efficiency = if is_herb { crate::data::herbs::DIRECT_CONSUMPTION_EFFICIENCY } else { 1.0 };

                    let mut logs: Vec<String> = Vec::new();
                    for effect in &item.effects {
                        logs.extend(Game::apply_item_effect(disciple, effect, &item, efficiency, is_herb));
                    }

                    // Deduct item
                    if let Some(c) = self.inventory.get_mut(&item_id) {
                        *c -= 1;
                    }

                    // Check for UseItem bottleneck clearing
                    if let Some(crate::data::disciples::Bottleneck::UseItem(ref bn_id)) = disciple.breakthrough_bottleneck {
                        if *bn_id == item_id {
                            logs.push(format!("{} overcame their bottleneck: Use {}!", disciple.name, item.name));
                            disciple.breakthrough_bottleneck = None;
                        }
                    }

                    for log in logs {
                        self.event_log.push(log);
                    }
                }
            }
        } else {
            self.event_log.push("Item not found in inventory.".to_string());
        }
    }

    fn handle_equip_item(&mut self, item_id: String, disciple_idx: usize) {
        if let Some(beast_item) = self.data.beast_equipment_definitions.get(&item_id) {
            if beast_item.beast_only {
                self.event_log.push("Cannot equip beast-only items on disciples.".to_string());
                return;
            }
        }
        let count = *self.inventory.get(&item_id).unwrap_or(&0);
        if count == 0 {
            self.event_log.push("Cannot equip: item not in inventory.".to_string());
            return;
        }

        let Some(item) = self.data.items.get(&item_id).cloned() else {
            self.event_log.push("Cannot equip: unknown item.".to_string());
            return;
        };

        let Some(equipment) = item.equipment.clone() else {
            self.event_log.push(format!("{} cannot be equipped.", item.name));
            return;
        };

        if let Some(disciple) = self.disciples.get_mut(disciple_idx) {
            // Unequip existing item in slot, if any
            if let Some(existing_id) = disciple.equipment.get(&equipment.slot).cloned() {
                if let Some(existing_item) = self.data.items.get(&existing_id) {
                    if let Some(existing_eq) = existing_item.equipment.as_ref() {
                        Self::apply_equipment_modifiers(&mut disciple.attributes, &existing_eq.modifiers, -1);
                    }
                }
                *self.inventory.entry(existing_id.clone()).or_insert(0) += 1;
                disciple.equipment.remove(&equipment.slot);
            }

            // Equip new item
            Self::apply_equipment_modifiers(&mut disciple.attributes, &equipment.modifiers, 1);
            disciple.equipment.insert(equipment.slot.clone(), item_id.clone());

            if let Some(c) = self.inventory.get_mut(&item_id) {
                *c -= 1;
            }

            self.event_log.push(format!("{} equipped {}.", disciple.name, item.name));
        }
    }

    fn handle_equip_beast_item(&mut self, beast_id: u64, item_id: String) {
        let count = *self.inventory.get(&item_id).unwrap_or(&0);
        if count == 0 {
            self.event_log.push("Beast equipment not in inventory.".to_string());
            return;
        }

        let Some(item) = self.data.beast_equipment_definitions.get(&item_id) else {
            self.event_log.push("Unknown beast equipment item.".to_string());
            return;
        };

        if !item.beast_only {
            self.event_log.push("Item is not beast-only equipment.".to_string());
            return;
        }

        let Some(beast) = self.spirit_beasts.iter_mut().find(|b| b.id == beast_id) else {
            self.event_log.push("Spirit beast not found.".to_string());
            return;
        };

        if !beast.can_equip(item) {
            self.event_log.push("Beast cannot equip this item yet.".to_string());
            return;
        }

        // Replace existing slot and return old item to inventory
        if let Some(existing) = beast.equipment.iter().find(|eq| eq.slot == item.slot).cloned() {
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

    fn handle_unequip_item(&mut self, slot: crate::data::items::EquipmentSlot, disciple_idx: usize) {
        if let Some(disciple) = self.disciples.get_mut(disciple_idx) {
            if let Some(item_id) = disciple.equipment.get(&slot).cloned() {
                if let Some(item) = self.data.items.get(&item_id) {
                    if let Some(eq) = item.equipment.as_ref() {
                        Self::apply_equipment_modifiers(&mut disciple.attributes, &eq.modifiers, -1);
                    }
                    *self.inventory.entry(item_id.clone()).or_insert(0) += 1;
                    disciple.equipment.remove(&slot);
                    self.event_log.push(format!("{} unequipped {}.", disciple.name, item.name));
                }
            }
        }
    }

    fn handle_research_tech(&mut self, tech_id: String) {
        if !self.unlocked_techs.contains(&tech_id) {
            if let Some(tech) = self.data.techs.get(&tech_id) {
                // Check prerequisites
                let prereqs_met = tech.prerequisites.iter().all(|p| self.unlocked_techs.contains(p));
                if prereqs_met {
                    if self.spirit_stones >= tech.cost_spirit_stones {
                        self.spirit_stones -= tech.cost_spirit_stones;
                        self.unlocked_techs.push(tech_id.clone());
                        self.event_log.push(format!("Researched: {}", tech.name));
                    } else {
                        self.event_log.push("Not enough Spirit Stones to research.".to_string());
                    }
                } else {
                    self.event_log.push("Prerequisites not met.".to_string());
                }
            }
        }
    }

    fn handle_repair_building(&mut self, id: u64) {
        if let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == id) {
            if building.status == crate::data::buildings::BuildingStatus::Ruined {
                let cost = building.repair_cost;
                if self.spirit_stones >= cost {
                    self.spirit_stones -= cost;
                    building.status = crate::data::buildings::BuildingStatus::Active;
                    self.event_log.push(format!("Repaired {}!", building.building_type));
                } else {
                    self.event_log
                        .push(format!("Not enough Spirit Stones to repair ({} required).", cost));
                }
            }
        }
    }

    fn handle_construct_building(&mut self, b_type: BuildingType, x: i32, y: i32) {
        // Get building definition
        let def = self.data.building_definitions.get(&b_type);
        let cost = def.map(|d| d.cost).unwrap_or(100);
        let is_unique = def.map(|d| d.unique).unwrap_or(false);
        let element = def.map(|d| d.element.clone()).unwrap_or_default();

        // Check if unique building already exists
        if is_unique && self.data.buildings.iter().any(|b| b.building_type == b_type) {
            self.event_log.push(format!("Cannot build: {} already exists.", b_type));
            return;
        }

        if self.spirit_stones >= cost {
            self.spirit_stones -= cost;
            let mut new_b = crate::data::buildings::Building::new(b_type.clone());
            new_b.id = rand::random(); // Simple ID
            new_b.x = x;
            new_b.y = y;
            new_b.element = element; // Set element from definition
            if let Some(def) = def {
                new_b.repair_cost = def.repair_cost;
            }
            new_b.status = crate::data::buildings::BuildingStatus::Active; // Instant build for MVP
            self.data.buildings.push(new_b);
            self.event_log.push(format!("Constructed {} at {},{}", b_type, x, y));

            // Auto-discover basic recipes for this building type
            let new_recipes: Vec<String> = self.data.recipes.iter()
                .filter(|r| r.required_building == b_type && !self.discovered_recipes.contains(&r.id))
                .map(|r| r.id.clone())
                .collect();
            for recipe_id in &new_recipes {
                if let Some(recipe) = self.data.recipes.iter().find(|r| r.id == *recipe_id) {
                    self.event_log.push(format!("Discovered recipe: {}!", recipe.name));
                }
                self.discovered_recipes.push(recipe_id.clone());
            }
        } else {
            self.event_log.push(format!("Not enough Spirit Stones ({} required).", cost));
        }
    }

    fn handle_start_new_game(&mut self, name: String) {
        // Manually reset to "New Game" state with our preset logic:
        let data = self.data.clone();

        // 1. Leader with Survivor trait
        let mut leader = generate_disciple(&data);
        leader.name = "Patriarch".to_string();
        leader.rank = DiscipleRank::SectLeader;

        // Give Patriarch the Survivor trait
        if let Some(survivor_trait) = data.fate_traits.iter().find(|t| t.name == "Survivor") {
            leader.fate_traits.retain(|t| t.name != "Survivor");
            leader.fate_traits.push(survivor_trait.clone());
        }

        // 2. No starting workers
        self.disciples = vec![leader];
        self.sect_name = name;
        self.spirit_stones = 50; // Reduced to 50 check
        self.herbs = 10;
        self.influence = 0;
        self.relics = 0;
        self.inventory.clear();
        self.unlocked_techs.clear();
        self.discovered_recipes = vec![
            "recipe_healing_pill".to_string(),
            "recipe_iron_sword".to_string(),
        ];
        self.ongoing_missions.clear();
        self.completed_missions.clear();
        self.completed_history.clear();
        self.deceased_disciples.clear();
        self.event_log = vec!["The sect has fallen... We must rebuild.".to_string()];
        self.tutorial = crate::state::TutorialState::new();
        self.tick = 0;
        self.current_season = crate::data::herbs::Season::Spring;
        self.season_ticks = 3600;
        self.grid = crate::data::grid::Grid::new(20, 20); // Reset Grid

        // Reset world simulation
        self.world_sim = WorldSim::new(
            self.data.factions.clone(),
            self.data.economy_nodes.clone(),
            self.data.trade_routes.clone(),
            self.data.world_events.clone(),
            self.data.balance.clone(),
        );

        // Reset Buildings
        self.data.buildings.clear();

        // Start with a ruined Sect Hall to restore
        let mut sect_hall = crate::data::buildings::Building::new(BuildingType::SectHall);
        sect_hall.id = rand::random();
        sect_hall.x = 2048;
        sect_hall.y = 2048;
        sect_hall.element = self
            .data
            .building_definitions
            .get(&BuildingType::SectHall)
            .map(|d| d.element.clone())
            .unwrap_or_default();
        sect_hall.status = crate::data::buildings::BuildingStatus::Ruined;
        if let Some(def) = self.data.building_definitions.get(&BuildingType::SectHall) {
            sect_hall.repair_cost = def.repair_cost;
        }
        self.data.buildings.push(sect_hall);

        self.transition(StateTransition::ToSectBase);
    }

    fn handle_send_diplomat(
        &mut self,
        faction_id: String,
        action: crate::data::relations::DiplomaticAction,
    ) {
        let results = self.world_sim.process_diplomatic_action(&faction_id, action);
        for result in results {
            self.handle_world_sim_result(result);
        }
    }

    fn handle_respond_to_event(&mut self, event_id: String, choice_idx: usize) {
        let effects = self.world_sim.respond_to_event(&event_id, choice_idx);
        for effect in effects {
            self.apply_event_effect(&effect);
        }
    }
}