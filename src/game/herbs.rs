use super::Game;
use crate::data::buildings::BuildingType;
use crate::data::disciples::DiscipleRank;
use crate::data::herbs::{GrowingHerb, RAW_HERB_DECAY_RATE};

impl Game {
    /// Process herbs in the drying pavilion
    pub(super) fn process_drying(&mut self, building_id: u64, herb_id: &str) {
        // Find building
        let building = match self.data.buildings.iter().find(|b| b.id == building_id) {
            Some(b) => b.clone(),
            None => {
                self.event_log
                    .push("No matching hall was found on the mountain.".to_string());
                return;
            }
        };

        if building.building_type != BuildingType::DryingPavilion {
            self.event_log
                .push("Only the Drying Pavilion can preserve fresh herbs.".to_string());
            return;
        }

        // Check if we have the herb (and it's not already dried)
        if herb_id.starts_with("dried_") {
            self.event_log
                .push("That herb is already preserved.".to_string());
            return;
        }

        let current_count = *self.inventory.get(herb_id).unwrap_or(&0);
        if current_count < 5 {
            self.event_log.push(format!(
                "The drying racks need 5 fresh {} (sect stores have {}).",
                herb_id, current_count
            ));
            return;
        }

        // Process: 5 raw -> 4 dried (with loss reduction from level)
        let loss_rate = building.get_drying_loss_rate();
        let output_amount = ((5.0 * (1.0 - loss_rate)).ceil() as u32).max(1);

        // Deduct raw herbs
        if let Some(count) = self.inventory.get_mut(herb_id) {
            *count -= 5;
        }

        // Add dried herbs
        let dried_id = format!("dried_{}", herb_id);
        *self.inventory.entry(dried_id.clone()).or_insert(0) += output_amount;

        self.event_log.push(format!(
            "Preserved 5 fresh {} into {} {}.",
            herb_id, output_amount, dried_id
        ));
    }

    /// Set or clear greenhouse elemental infusion
    pub(super) fn set_greenhouse_infusion(
        &mut self,
        building_id: u64,
        element: Option<crate::data::elements::Element>,
    ) {
        if let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == building_id) {
            if building.building_type != BuildingType::Greenhouse {
                self.event_log
                    .push("Only the Greenhouse can hold a season-bending array.".to_string());
                return;
            }

            if let Some(ref elem) = element {
                building.infused_element = Some(elem.clone());
                self.event_log
                    .push(format!("Greenhouse array tuned to the {} aspect.", elem));
            } else {
                building.infused_element = None;
                self.event_log
                    .push("Greenhouse array extinguished.".to_string());
            }
        }
    }

    /// Process herb growth and harvesting in herb gardens
    pub(super) fn process_herb_gardens(&mut self) {
        let mut harvested_herbs: Vec<(String, u32)> = Vec::new();
        let mut log_messages: Vec<String> = Vec::new();

        // Get disciple info for quality calculation
        let disciple_spirits: std::collections::HashMap<u64, u32> = self
            .disciples
            .iter()
            .map(|d| (d.id, d.attributes.spirit))
            .collect();

        for building in self.data.buildings.iter_mut() {
            if building.building_type != BuildingType::HerbGarden
                && building.building_type != BuildingType::Greenhouse
            {
                continue;
            }

            // Sync plots with building level
            building.sync_herb_plots();

            let growth_multiplier = building.get_growth_speed_multiplier();
            let has_worker = building.assigned_disciple.is_some();
            let worker_spirit = building
                .assigned_disciple
                .and_then(|id| disciple_spirits.get(&id).copied())
                .unwrap_or(0);

            for plot in building.herb_plots.iter_mut() {
                if let Some(ref mut growing) = plot.growing {
                    // Apply growth
                    let growth = (1.0 * growth_multiplier) as u32;
                    growing.ticks_remaining = growing.ticks_remaining.saturating_sub(growth.max(1));

                    // Check for harvest
                    if growing.is_mature() {
                        if has_worker {
                            // Harvest with quality bonus from worker Spirit
                            let quality_bonus = 1.0 + (worker_spirit as f32 / 100.0);
                            let final_quality = (growing.quality * quality_bonus).min(2.0);
                            let harvest_amount = (final_quality).ceil() as u32;

                            harvested_herbs.push((growing.herb_id.clone(), harvest_amount));
                            log_messages.push(format!(
                                "Gathered {}x {} from {}.",
                                harvest_amount, growing.herb_id, building.building_type
                            ));

                            // Clear plot for replanting
                            plot.growing = None;
                            plot.decay_ticks = 0;
                        } else {
                            // No worker - herb decays on vine
                            plot.decay_ticks += 1;
                            if plot.decay_ticks > 60 {
                                log_messages.push(format!(
                                    "A {} withered in {} with no attendant to harvest it.",
                                    growing.herb_id, building.building_type
                                ));
                                plot.growing = None;
                                plot.decay_ticks = 0;
                            }
                        }
                    }
                }
            }
        }

        // Apply harvests to inventory
        for (herb_id, amount) in harvested_herbs {
            *self.inventory.entry(herb_id).or_insert(0) += amount;
        }

        // Add log messages
        for msg in log_messages {
            self.event_log.push(msg);
        }
    }

    /// Apply herb decay at season change
    pub(super) fn apply_herb_decay(&mut self) {
        // Calculate total decay reduction from Herb Storage buildings
        let storage_reduction: f32 = self
            .data
            .buildings
            .iter()
            .filter(|b| {
                b.building_type == BuildingType::HerbStorage
                    && b.status == crate::data::buildings::BuildingStatus::Active
            })
            .map(|b| b.get_decay_reduction())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let final_decay_rate = RAW_HERB_DECAY_RATE * (1.0 - storage_reduction);

        if final_decay_rate <= 0.001 {
            return; // Effectively no decay
        }

        // Get list of raw herb IDs from data
        let raw_herb_ids: Vec<String> = self.data.herbs.keys().cloned().collect();

        let mut decay_messages: Vec<String> = Vec::new();

        for herb_id in raw_herb_ids {
            // Only decay raw herbs (not dried)
            if herb_id.starts_with("dried_") {
                continue;
            }

            if let Some(count) = self.inventory.get_mut(&herb_id) {
                if *count > 0 {
                    let decay_amount = ((*count as f32) * final_decay_rate).ceil() as u32;
                    let actual_decay = decay_amount.min(*count);
                    if actual_decay > 0 {
                        *count -= actual_decay;
                        decay_messages.push(format!(
                            "{} {} withered in the apothecary jars.",
                            actual_decay, herb_id
                        ));
                    }
                }
            }
        }

        for msg in decay_messages {
            self.event_log.push(msg);
        }
    }

    /// Plant an herb in a garden plot
    pub fn plant_herb(&mut self, building_id: u64, plot_index: usize, herb_id: &str) -> bool {
        // Validate herb exists
        let herb = match self.data.herbs.get(herb_id) {
            Some(h) => h.clone(),
            None => return false,
        };

        // Find building
        let building = match self.data.buildings.iter_mut().find(|b| b.id == building_id) {
            Some(b) => b,
            None => return false,
        };

        // Validate building type
        if building.building_type != BuildingType::HerbGarden
            && building.building_type != BuildingType::Greenhouse
        {
            return false;
        }

        // Check tier restrictions
        if herb.tier > building.get_max_herb_tier() {
            self.event_log.push(format!(
                "{} is too potent for the current {} terrace grade (tier {}).",
                herb.name, building.building_type, herb.tier
            ));
            return false;
        }

        // Check season (unless greenhouse with infusion)
        let can_grow_this_season = herb.grow_seasons.contains(&self.current_season)
            || (building.building_type == BuildingType::Greenhouse
                && building.infused_element.as_ref() == Some(&herb.element));

        if !can_grow_this_season {
            self.event_log.push(format!(
                "{} rejects the current mountain season ({}).",
                herb.name, self.current_season
            ));
            return false;
        }

        // Check plot availability
        building.sync_herb_plots();
        if plot_index >= building.herb_plots.len() {
            return false;
        }

        if building.herb_plots[plot_index].growing.is_some() {
            self.event_log
                .push("That spirit terrace is already occupied.".to_string());
            return false;
        }

        // Plant the herb
        let growing = GrowingHerb::new(herb_id.to_string(), herb.grow_time_ticks);
        building.herb_plots[plot_index].growing = Some(growing);
        self.event_log.push(format!(
            "Sowed {} in {}.",
            herb.name, building.building_type
        ));
        true
    }

    /// Assign a disciple to work a building
    pub fn assign_disciple_to_building(
        &mut self,
        building_id: u64,
        disciple_id: Option<u64>,
    ) -> bool {
        // If assigning, validate disciple exists and is Outer rank
        if let Some(d_id) = disciple_id {
            let is_valid_worker = self
                .disciples
                .iter()
                .any(|d| d.id == d_id && d.rank == DiscipleRank::Outer);
            if !is_valid_worker {
                self.event_log
                    .push("Only outer disciples can be appointed to hall duty.".to_string());
                return false;
            }

            // Check if disciple is already assigned elsewhere
            let already_assigned = self
                .data
                .buildings
                .iter()
                .any(|b| b.assigned_disciple == Some(d_id));
            if already_assigned {
                self.event_log
                    .push("That disciple already serves another hall.".to_string());
                return false;
            }
        }

        // Find and update building
        if let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == building_id) {
            building.assigned_disciple = disciple_id;
            if disciple_id.is_some() {
                self.event_log.push(format!(
                    "Appointed an attendant to {}.",
                    building.building_type
                ));
            } else {
                self.event_log.push(format!(
                    "Recalled the attendant from {}.",
                    building.building_type
                ));
            }
            return true;
        }
        false
    }
}
