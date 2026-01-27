use crate::data::disciples::{Disciple, DiscipleRank, Talent};
use crate::data::grid::Grid;
use crate::data::buildings::{BuildingStatus, BuildingType};
use crate::data::herbs::{Season, GrowingHerb, DIRECT_CONSUMPTION_EFFICIENCY, RAW_HERB_DECAY_RATE};
use crate::data::history::DeceasedDisciple;
use crate::data::loader::GameData;
use crate::data::missions::{MissionOutcome, MissionRewards, OngoingMission, RelevantStat};
use crate::engine::actions::Action;
use crate::engine::proc_gen::generate_disciple;
use crate::engine::tribulation::{TribulationState, TribulationType};
use crate::state::{
    library::LibraryState, main_menu::MainMenuState,
    mission_assignment::MissionAssignmentState, mission_resolution::MissionResolutionState,
    roster::DiscipleRosterState, sect_base::SectBaseState, sect_creation::SectCreationState,
    world_map::WorldMapState, tribulation::TribulationEncounterState, GameState,
    StateTransition,
};
use rand::Rng;
use crate::save::SaveData;

/// Result of a breakthrough attempt
pub enum BreakthroughResult {
    Success,
    Failure,           // Died
    Injured,           // Survived but didn't advance
    Tribulation(TribulationType), // Needs tribulation
}
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use quad_storage::LocalStorage;

pub struct Game {
    pub state: GameState,
    pub data: GameData,
    pub grid: Grid,
    pub sect_name: String,
    pub spirit_stones: u32,
    pub herbs: u32,
    pub influence: u32,
    pub relics: u32,
    pub inventory: std::collections::HashMap<String, u32>, // ItemID -> Count
    pub unlocked_techs: Vec<String>,
    pub disciples: Vec<Disciple>,
    pub deceased_disciples: Vec<DeceasedDisciple>,
    pub ongoing_missions: Vec<OngoingMission>,
    pub completed_missions: Vec<MissionOutcome>,
    pub completed_history: Vec<String>, // List of descriptions of successfully completed missions
    pub event_log: Vec<String>,
    pub tutorial: crate::state::TutorialState,
    tick: u64,
    /// Current season for herb growth
    pub current_season: Season,
    /// Ticks until next season change (3600 ticks = 1 minute at 60 fps)
    pub season_ticks: u32,
}

impl Game {
    pub async fn new() -> Self {
        let mut data = GameData::load().expect("Failed to load game data");
        data.buildings.clear(); // User requested blank map

        // Scenario: Survivors of the Fallen Sect
        // 1. The Patriarch (Sect Leader)
        let mut leader = generate_disciple(&data);
        leader.name = "Patriarch".to_string();
        leader.rank = DiscipleRank::SectLeader;

        // Give Patriarch the Survivor trait - they cannot die, only suffer setbacks
        if let Some(survivor_trait) = data.fate_traits.iter().find(|t| t.name == "Survivor") {
            // Remove any conflicting traits and add Survivor
            leader.fate_traits.retain(|t| t.name != "Survivor");
            leader.fate_traits.push(survivor_trait.clone());
        }

        // 2. No starting workers (User requested "do not start with disciples")
        let disciples = vec![leader];
        
        let grid = Grid::new(20, 20);

        Self {
            state: GameState::MainMenu(MainMenuState::new()),
            data,
            grid,
            sect_name: "Unnamed Sect".to_string(), // Initial placeholder
            spirit_stones: 50, // Reduced to 50 as Sect Hall is free
            herbs: 10,         // Some supplies
            influence: 0,
            relics: 0,
            inventory: std::collections::HashMap::new(),
            unlocked_techs: Vec::new(),
            disciples,
            deceased_disciples: Vec::new(),
            ongoing_missions: Vec::new(),
            completed_missions: Vec::new(),
            completed_history: Vec::new(),
            event_log: vec!["The sect has fallen... We must rebuild.".to_string()],
            tutorial: crate::state::TutorialState::new(),
            tick: 0,
            current_season: Season::Spring,
            season_ticks: 3600, // 1 minute per season at 60 fps
        }
    }

    fn get_population_capacity(&self) -> u32 {
        self.data
            .buildings
            .iter()
            .filter(|b| b.status == BuildingStatus::Active)
            .map(|b| match b.building_type {
                BuildingType::SectHall => b.get_max_disciples(),
                BuildingType::Dormitory => b.get_dorm_capacity(),
                _ => 0,
            })
            .sum()
    }

    pub fn update(&mut self) {
        self.tick += 1;

        if self.tick % 60 == 0 {
            // Collect indices of disciples currently on missions (they don't cultivate)
            let disciples_on_mission: std::collections::HashSet<usize> = self.ongoing_missions
                .iter()
                .flat_map(|m| m.disciple_indices.iter().copied())
                .collect();

            // Cultivation Tick - check for breakthroughs (only for disciples not on missions)
            let mut disciples_to_breakthrough = Vec::new();
            for (i, disciple) in self.disciples.iter().enumerate() {
                if !disciples_on_mission.contains(&i) && disciple.exp >= disciple.exp_to_next_level {
                    disciples_to_breakthrough.push(i);
                }
            }
            
            // Process breakthroughs and handle results
            let mut indices_to_remove = Vec::new();
            let mut pending_tribulation: Option<(TribulationType, usize)> = None;

            for i in disciples_to_breakthrough {
                let mut disciple = self.disciples[i].clone();
                let result = self.attempt_breakthrough(&mut disciple);

                match result {
                    BreakthroughResult::Failure => {
                        // Record in hall of fallen
                        self.deceased_disciples.push(DeceasedDisciple::new(
                            disciple.name.clone(),
                            disciple.realm.clone(),
                            "Failed Breakthrough".to_string(),
                            self.tick,
                        ));
                        indices_to_remove.push(i);
                    }
                    BreakthroughResult::Tribulation(t_type) => {
                        // Queue tribulation (only one per tick)
                        if pending_tribulation.is_none() {
                            pending_tribulation = Some((t_type, i));
                        }
                        self.disciples[i] = disciple;
                    }
                    _ => {
                        // Success or Injured - update disciple
                        self.disciples[i] = disciple;
                    }
                }
            }

            // Remove dead disciples (in reverse to preserve indices)
            for i in indices_to_remove.into_iter().rev() {
                self.disciples.remove(i);
            }

            // Trigger tribulation if pending
            if let Some((t_type, idx)) = pending_tribulation {
                let trib_state = TribulationState::new(t_type, &self.disciples[idx]);
                self.transition(StateTransition::ToTribulation(trib_state, idx));
            }
            // Apply Training Yard bonus
            // Apply Training Yard bonus
            let (yard_multiplier, yard_score) = self.data.buildings.iter()
                .find(|b| b.building_type == BuildingType::TrainingYard)
                .map(|b| (b.get_cultivation_multiplier(), b.feng_shui_score))
                .unwrap_or((1.0, 0.0));
            
            // Feng Shui Bonus: Score / 100.0 (e.g. 50.0 score -> +0.5 multiplier)
            let feng_shui_mod = yard_score / 100.0;
            let final_yard_multiplier = (yard_multiplier + feng_shui_mod).max(0.1);
            
            for (i, disciple) in self.disciples.iter_mut().enumerate() {
                // Skip disciples on missions - they don't cultivate while away
                if disciples_on_mission.contains(&i) {
                    continue;
                }

                let mut base_exp = 1 + (disciple.attributes.spirit / 5);
                
                // --- Law Logic ---
                let mut law_multiplier = 1.0;
                if let Some(law_id) = &disciple.law_id {
                    if let Some(law) = self.data.laws.get(law_id) {
                         // 1. Environment Check (Bonus/Penalty)
                         // We need disciple position. Currently disciples don't have x/y on grid (abstracted).
                         // For MVP, we assume they cultivate in the Training Yard if focused, or just check global/random?
                         // Actually, we can assume they are at the "Main Hall" or "Training Yard".
                         // Let's use the Training Yard's tile for environment check since that's where they cultivate.
                         
                         if let Some(yard) = self.data.buildings.iter().find(|b| b.building_type == BuildingType::TrainingYard) {
                             if let Some(tile) = self.grid.get_tile(yard.x, yard.y) {
                                  let env_element = &tile.dominant_element;
                                  let _type_match = env_element.get_interaction(&law.element);
                                  
                                  // Requirement: Law Element usually wants to be FED by environment or MATCH environment.
                                  // Traditional: Fire Law needs Fire Environment (Match) or Wood (Feed).
                                  // Let's simplify: Match = +50%, Feeds = +20%, Suppressed = -50%.
                                  
                                  if *env_element == law.element {
                                      law_multiplier += 0.5;
                                  } else if env_element.feeds() == law.element {
                                      law_multiplier += 0.2;
                                  } else if env_element.suppresses() == law.element {
                                      law_multiplier -= 0.5;
                                  }
                             }
                         }
                         
                         // 2. Stat Growth (Modify base stats?)
                         // For MVP, just treating this as XP boost based on primary stat?
                         // "stat_growth_modifiers" from struct.
                         // Let's say if Law focuses on Body, add Body / 10 to base_exp.
                         base_exp += (disciple.attributes.body * law.stat_growth_modifiers.body) / 10;
                         base_exp += (disciple.attributes.mind * law.stat_growth_modifiers.mind) / 10;
                         base_exp += (disciple.attributes.spirit * law.stat_growth_modifiers.spirit) / 10;
                    }
                }
                
                // Sum trait modifiers
                let trait_cultivation_mod: f32 = disciple.fate_traits.iter().map(|t| t.cultivation_speed_modifier).sum();
                
                // Bloodline modifiers
                let mut bloodline_mod = 0.0;
                if let Some(bloodline_id) = &disciple.bloodline.bloodline_id {
                    if let Some(bloodline) = self.data.bloodlines.get(bloodline_id) {
                         bloodline_mod = bloodline.passive_effects.cultivation_speed_modifier * disciple.bloodline.effectiveness();
                    }
                }

                let total_multiplier = (final_yard_multiplier * law_multiplier + trait_cultivation_mod + bloodline_mod).max(0.1); 
                
                let bonus_exp = (base_exp as f32 * total_multiplier) as u32;
                disciple.exp += bonus_exp;
            }

            // Apply Sect Hall passive income (meditation/ambient Qi)
            // The heart of the sect generates a small baseline income
            if self.data.buildings.iter().any(|b| b.building_type == BuildingType::SectHall && b.status == crate::data::buildings::BuildingStatus::Active) {
                self.spirit_stones += 1; // Base income of 1 SS per cultivation tick
            }

            // Apply Spirit Garden passive income (Feature 9.1.3: Requires Outer Disciples)
            if let Some(garden) = self.data.buildings.iter().find(|b| b.building_type == BuildingType::SpiritGarden) {
                let outer_count = self.disciples.iter().filter(|d| d.rank == DiscipleRank::Outer).count();
                if outer_count > 0 {
                    let income = garden.get_passive_income();
                    // Optional: Bonus for more workers? For MVP just require > 0.
                    self.spirit_stones += income;

                    // Small chance to find herbs if workers are present
                    if rand::thread_rng().gen_bool(0.1) {
                        self.herbs += 1;
                    }
                }
            }

            // Herb Garden growth and harvest logic
            self.process_herb_gardens();

            // Mission Tick
            let mut completed_indices = Vec::new();
            for (i, mission) in self.ongoing_missions.iter_mut().enumerate() {
                mission.ticks_remaining = mission.ticks_remaining.saturating_sub(1);
                if mission.ticks_remaining == 0 {
                    completed_indices.push(i);
                }
            }

            for index in completed_indices.into_iter().rev() {
                let mission = self.ongoing_missions.remove(index);
                let outcome = self.calculate_mission_outcome(mission);
                self.completed_missions.push(outcome);
                self.transition(StateTransition::ToMissionResolution);
            }
        }
        
        // Season change logic
        self.season_ticks = self.season_ticks.saturating_sub(1);
        if self.season_ticks == 0 {
            let old_season = self.current_season.clone();
            self.current_season = self.current_season.next();
            self.season_ticks = 3600; // Reset for next season
            self.event_log.push(format!("The season has changed from {} to {}.", old_season, self.current_season));

            // Apply herb decay at season change
            self.apply_herb_decay();
        }

        // World Evolution Tick (every 5 seconds approx)
        if self.tick % 300 == 0 {
            let mut rng = rand::thread_rng();
            for node in self.data.map_nodes.iter_mut() {
                if rng.gen_bool(0.3) { // 30% chance per node
                    node.corruption += 1;
                    if node.corruption % 10 == 0 {
                        self.event_log.push(format!("Nodes are corrupting! {} danger increased.", node.name));
                    }
                }
            }

            // Salary Tick (Every 300 ticks or 600? Let's do 600 ~ 10sec for daily salary)
        }
        
        if self.tick % 600 == 0 {
             let inner_count = self.disciples.iter().filter(|d| d.rank == DiscipleRank::Inner || d.rank == DiscipleRank::SectLeader).count();
             if inner_count > 0 {
                 let salary_cost = inner_count as u32;
                 if self.spirit_stones >= salary_cost {
                     self.spirit_stones -= salary_cost;
                     // Optional: reduced logging spam, or group it
                     // self.event_log.push(format!("Paid {} SS in salaries.", salary_cost));
                 } else {
                     self.spirit_stones = 0;
                     self.event_log.push("Warning: Cannot pay salaries! Morale is falling.".to_string());
                     // Future: Apply morale malus
                 }
             }
        }

        let update_result = match &mut self.state {
            GameState::MainMenu(s) => s.update(),
            GameState::SectBase(s) => s.update(&mut self.data, &mut self.grid, self.spirit_stones, self.herbs, self.influence, self.relics, &self.inventory, &self.unlocked_techs, &self.event_log, &self.ongoing_missions, &self.completed_missions, &self.completed_history, &self.disciples, &self.current_season, self.season_ticks, &mut self.tutorial),
            GameState::DiscipleRoster(s) => s.update(&self.data, &self.disciples, &self.inventory),
            GameState::WorldMap(s) => s.update(&self.data),
            GameState::MissionResolution(s) => s.update(&mut self.completed_missions),
            GameState::Library(s) => s.update(&self.data, self.spirit_stones, &self.deceased_disciples),
            GameState::MissionAssignment(s) => s.update(&self.data, &self.disciples),
            GameState::SectCreation(s) => s.update(),
            GameState::Tribulation(s) => s.update(&self.disciples),
        };

        if let Some(action) = update_result.action {
            self.execute_action(action);
        }
        if let Some(transition) = update_result.transition {
            self.transition(transition);
        }
    }

    pub fn draw(&mut self) {
        match &mut self.state {
            GameState::MainMenu(s) => s.draw(&self.data, self.spirit_stones),
            GameState::SectBase(s) => s.draw(&self.data, &self.grid, self.spirit_stones),
            GameState::DiscipleRoster(s) => s.draw(&self.data, &self.disciples, self.spirit_stones),
            GameState::WorldMap(s) => s.draw(&self.data, self.spirit_stones),
            GameState::MissionResolution(s) => s.draw(&self.data, self.spirit_stones),
            GameState::Library(s) => s.draw(&self.data, self.spirit_stones, &self.deceased_disciples),
            GameState::MissionAssignment(s) => s.draw(&self.data, &self.disciples, self.spirit_stones),
            GameState::SectCreation(s) => s.draw(&self.data),
            GameState::Tribulation(s) => s.draw(&self.data, &self.disciples),
        }
    }

    fn transition(&mut self, transition: StateTransition) {
        self.state = match transition {
            StateTransition::ToMainMenu => GameState::MainMenu(MainMenuState::new()),
            StateTransition::ToSectBase => GameState::SectBase(SectBaseState::new()),
            StateTransition::ToDiscipleRoster => GameState::DiscipleRoster(DiscipleRosterState::new()),
            StateTransition::ToWorldMap => GameState::WorldMap(WorldMapState::new()),
            StateTransition::ToMissionAssignment(desc) => {
                GameState::MissionAssignment(MissionAssignmentState::new(desc))
            }
            StateTransition::ToMissionResolution => {
                GameState::MissionResolution(MissionResolutionState::new())
            }
            StateTransition::ToLibrary => GameState::Library(LibraryState::new()),
            StateTransition::ToSectCreation => GameState::SectCreation(SectCreationState::new()),
            StateTransition::ToTribulation(trib_state, disciple_idx) => {
                GameState::Tribulation(TribulationEncounterState::new(trib_state, disciple_idx))
            }
        };
    }

    /// Attempts a breakthrough. Returns result logic.
    fn attempt_breakthrough(&mut self, disciple: &mut Disciple) -> BreakthroughResult {
        let mut rng = rand::thread_rng();
        let base_chance = match disciple.talent {
             Talent::Low => 0.3,
             Talent::Medium => 0.5,
             Talent::High => 0.7,
             Talent::Genius => 0.9,
             Talent::HeavenSent => 1.0,
         };

        // Apply trait modifiers
        let trait_modifier: f32 = disciple.fate_traits.iter().map(|t| t.breakthrough_modifier).sum();

        // Bloodline modifiers
        let mut bloodline_breakthrough_mod = 0.0;
        let mut bloodline_survivor = false;
        let mut bloodline_injury_mod = 0.0;
        
        if let Some(bloodline_id) = &disciple.bloodline.bloodline_id {
            if let Some(bloodline) = self.data.bloodlines.get(bloodline_id) {
                let effectiveness = disciple.bloodline.effectiveness();
                bloodline_breakthrough_mod = bloodline.passive_effects.breakthrough_modifier * effectiveness;
                bloodline_injury_mod = bloodline.passive_effects.injury_modifier * effectiveness;
                if bloodline.passive_effects.survivor {
                    bloodline_survivor = true;
                }
            }
        }

        // Check if disciple has Survivor trait (cannot die)
        let is_survivor = disciple.fate_traits.iter().any(|t| t.survivor) || bloodline_survivor;

        // Apply Law modifiers
        let mut law_modifier = 0.0;
        if let Some(law_id) = &disciple.law_id {
            if let Some(law) = self.data.laws.get(law_id) {
                law_modifier = law.breakthrough_modifier;
            }
        }

        let success_chance = (base_chance + trait_modifier + law_modifier + bloodline_breakthrough_mod).clamp(0.05, 0.99);

        if rng.gen::<f32>() < success_chance {
            // Find current stage index using stages_order
            let stage_idx = self.data.stages_order.iter().position(|id| id == &disciple.realm);
            let stage = self.data.stages.get(&disciple.realm);

            if let (Some(stage_idx), Some(stage)) = (stage_idx, stage) {
                // Check if can advance sub-stage
                if disciple.sub_stage < stage.sub_stages.len().saturating_sub(1) {
                    // Minor breakthrough (sub-stage advance)
                    disciple.sub_stage += 1;
                    let sub_stage_name = stage.sub_stages.get(disciple.sub_stage)
                        .map(|s| s.name.as_str())
                        .unwrap_or("Unknown");
                    self.event_log.push(format!("{} advanced to {} - {}!", disciple.name, stage.name, sub_stage_name));
                    disciple.exp = 0;
                    disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 1.2) as u32;
                } else {
                    // Major breakthrough (next realm)
                    let next_stage_id = self.data.stages_order.get(stage_idx + 1);

                    if let Some(next_id) = next_stage_id {
                        if let Some(next_stage) = self.data.stages.get(next_id) {
                            // Determine if tribulation is needed (Core Formation and above)
                            let needs_tribulation = matches!(
                                disciple.realm.as_str(),
                                "FoundationEstablishment" | "CoreFormation" | "NascentSoul" |
                                "SoulTransformation" | "Ascension" | "TrueImmortal"
                            );

                            if needs_tribulation {
                                self.event_log.push(format!("Tribulation clouds gather above {}...", disciple.name));

                                let t_type = match disciple.realm.as_str() {
                                    "FoundationEstablishment" => TribulationType::GoldenCore,
                                    "CoreFormation" => TribulationType::NascentSoul,
                                    "NascentSoul" => TribulationType::SpiritSevering,
                                    "SoulTransformation" => TribulationType::Ascension,
                                    _ => TribulationType::GoldenCore,
                                };

                                return BreakthroughResult::Tribulation(t_type);
                            } else {
                                // Instant success for early realms
                                disciple.realm = next_stage.id.clone();
                                disciple.sub_stage = 0;
                                self.event_log.push(format!("{} broke through to {} realm!", disciple.name, next_stage.name));
                                disciple.exp = 0;
                                disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 2.5) as u32;
                            }
                        }
                    } else {
                        // Pinnacle
                        self.event_log.push(format!("{} has reached the apex of this world.", disciple.name));
                        disciple.exp = disciple.exp_to_next_level;
                    }
                }
            } else {
                self.event_log.push(format!("Error: Unknown realm {} for {}", disciple.realm, disciple.name));
            }

            BreakthroughResult::Success
        } else {
            // Failed breakthrough - check for death
            let injury_modifier: f32 = disciple.fate_traits.iter().map(|t| t.injury_modifier).sum();
            let death_chance = (0.1 + injury_modifier + bloodline_injury_mod).clamp(0.0, 0.5); // Base 10% death on failure

            if !is_survivor && rng.gen::<f32>() < death_chance {
                self.event_log.push(format!("{} perished attempting to break through!", disciple.name));
                BreakthroughResult::Failure // Died
            } else {
                // Survivor trait or lucky - just injured
                if is_survivor {
                    self.event_log.push(format!("{}'s indomitable will saved them! Suffered severe injuries but survived.", disciple.name));
                    disciple.exp = (disciple.exp as f32 * 0.25) as u32; // Lose 75% EXP - harsher penalty for cheating death
                } else {
                    self.event_log.push(format!("{} failed breakthrough and suffered internal injuries.", disciple.name));
                    disciple.exp = (disciple.exp as f32 * 0.5) as u32; // Lose 50% EXP
                }
                BreakthroughResult::Injured
            }
        }
    }

    fn calculate_mission_outcome(&self, ongoing: OngoingMission) -> MissionOutcome {
        let mut rng = rand::thread_rng();
        let mut team_power: i32 = 0;
        let mut trait_modifier: f32 = 0.0;
        let mut logs = Vec::new();

        let relevant_stat = ongoing.mission.mission_type.get_relevant_stat();
        logs.push(format!("Mission Type: {:?} (Uses {:?})", ongoing.mission.mission_type, relevant_stat));

        for &idx in &ongoing.disciple_indices {
            if let Some(disciple) = self.disciples.get(idx) {
                // Realm power from Data
                let realm_power = self.data.stages.get(&disciple.realm)
                    .map(|s| (s.base_hp + s.base_qi) / 100) // Rough estimate
                    .unwrap_or(1) as i32;
                
                // Attribute power based on mission type
                let attr_power = match relevant_stat {
                    RelevantStat::Body => disciple.attributes.body / 5,
                    RelevantStat::Mind => disciple.attributes.mind / 5,
                    RelevantStat::Spirit => disciple.attributes.spirit / 5,
                };
                
                let total_power = realm_power + attr_power as i32;
                team_power += total_power;
                
                // Sum relevant trait modifiers
                for t in &disciple.fate_traits {
                    trait_modifier += ongoing.mission.mission_type.get_trait_modifier(t);
                }
                
                logs.push(format!("{}: Realm Pwr {} + Attr {} = {}", disciple.name, realm_power, attr_power, total_power));
            }
        }

        let difficulty = (ongoing.mission.danger_level * 2) as i32;
        logs.push(format!("Team Power: {} vs Difficulty: {}", team_power, difficulty));
        
        let base_chance = 0.5;
        let power_modifier = (team_power - difficulty) as f32 * 0.08;
        let chance = (base_chance + power_modifier + trait_modifier).clamp(0.1, 0.95);
        
        logs.push(format!("Success Chance: {:.0}% (Trait Mod: {:+.0}%)", chance * 100.0, trait_modifier * 100.0));
        
        let roll = rng.gen::<f32>();
        let success = roll < chance;

        let rewards = if success {
             logs.push("Mission Successful!".to_string());
             
             match ongoing.mission.mission_type {
                 crate::data::missions::MissionType::Exploration => {
                     // Low Risk, Herbs
                     MissionRewards {
                         spirit_stones: ongoing.mission.danger_level * 10,
                         disciple_exp: ongoing.mission.danger_level * 50,
                         herbs: ongoing.mission.danger_level * 5,
                         influence: 0,
                         relics: 0,
                         items: vec![],
                     }
                 },
                 crate::data::missions::MissionType::ResourceGathering => {
                     // Med Risk, Stones/Herbs/Ore
                     let ore_amount = ongoing.mission.danger_level * 5;
                     logs.push(format!("Gathered {} Spirit Ore.", ore_amount));
                     MissionRewards {
                         spirit_stones: ongoing.mission.danger_level * 30,
                         disciple_exp: ongoing.mission.danger_level * 60,
                         herbs: ongoing.mission.danger_level * 2, // Less herbs than exploration
                         influence: 0,
                         relics: 0,
                         items: vec![("spirit_ore".to_string(), ore_amount)],
                     }
                 },
                 crate::data::missions::MissionType::MonsterSuppression => {
                     // Combat, Stones + Chance for Relic
                     let found_relic = rng.gen_bool(0.2); // 20% chance
                     if found_relic { logs.push("Found a Monster Core (Relic)!".to_string()); }
                     MissionRewards {
                         spirit_stones: ongoing.mission.danger_level * 80,
                         disciple_exp: ongoing.mission.danger_level * 120,
                         herbs: 0,
                         influence: 0,
                         relics: if found_relic { 1 } else { 0 },
                         items: vec![],
                     }
                 },
                 crate::data::missions::MissionType::Diplomacy => {
                     // Mind, Influence + Trade
                     MissionRewards {
                         spirit_stones: ongoing.mission.danger_level * 40,
                         disciple_exp: ongoing.mission.danger_level * 80,
                         herbs: 0,
                         influence: ongoing.mission.danger_level * 5,
                         relics: 0,
                         items: vec![],
                     }
                 },
                 crate::data::missions::MissionType::RuinDelve => {
                     // High Risk, Relics
                     let found_relic = rng.gen_bool(0.7); // 70% chance
                     if found_relic { logs.push("Recovered an Ancient Artifact!".to_string()); }
                     MissionRewards {
                         spirit_stones: ongoing.mission.danger_level * 100,
                         disciple_exp: ongoing.mission.danger_level * 150,
                         herbs: 0,
                         influence: 0,
                         relics: if found_relic { 1 } else { 0 },
                         items: vec![],
                     }
                 },
             }
        } else {
             logs.push("Mission Failed.".to_string());
             MissionRewards::default()
        };

        MissionOutcome {
            success,
            mission_name: ongoing.mission.description.clone(), 
            description: ongoing.mission.description, 
            logs,
            rewards,
            disciple_indices: ongoing.disciple_indices,
        }
    }

    fn execute_action(&mut self, action: Action) {
        match action {
            Action::UpgradeBuilding(building_type) => {
                // Feature 9.1.3: Restrict Building to Outer Disciples
                let has_outer_workers = self.disciples.iter().any(|d| d.rank == DiscipleRank::Outer);
                if !has_outer_workers {
                    self.event_log.push("Cannot Build: No Outer Disciples available to work!".to_string());
                } else {
                    let cost = 50;
                    if self.spirit_stones >= cost {
                        if let Some(building) = self.data.buildings.iter_mut().find(|b| b.building_type == building_type) {
                            self.spirit_stones -= cost;
                            building.level += 1;
                            self.event_log.push(format!("Upgraded {:?} to Lv {}", building.building_type, building.level));
                        }
                    } else {
                        self.event_log.push("Not enough Spirit Stones.".to_string());
                    }
                }
            }
            Action::RecruitDisciple => {
                let capacity = self.get_population_capacity();
                if capacity == 0 || self.disciples.len() as u32 >= capacity {
                    self.event_log.push("Population cap reached. Build Dormitories or upgrade the Sect Hall.".to_string());
                } else {
                    let new_disciple = generate_disciple(&self.data);
                    self.event_log.push(format!("Recruited: {}", new_disciple.name));
                    self.disciples.push(new_disciple);
                }
            }
            Action::PromoteDisciple(idx) => {
                if let Some(disciple) = self.disciples.get_mut(idx) {
                    if disciple.rank == DiscipleRank::Outer {
                        let cost = 100;
                        if self.spirit_stones >= cost {
                             if disciple.realm != "Mortal" {
                                 self.spirit_stones -= cost;
                                 disciple.rank = DiscipleRank::Inner;
                                 // Initialize Qi if 0 (should already be set by breakthrough but ensure here)
                                 if disciple.max_qi == 0 { disciple.max_qi = 100; disciple.qi = 100; } 
                                 
                                 self.event_log.push(format!("Promoted {} to Inner Disciple!", disciple.name));
                             } else {
                                 self.event_log.push("Cannot Promote: Must reach Qi Refinement first.".to_string());
                             }
                        } else {
                             self.event_log.push("Cannot Promote: Not enough Spirit Stones (100).".to_string());
                        }
                    }
                }
            }
            Action::StartMission(mission_desc, disciple_indices) => {
                if let Some(mission) = self.data.missions.iter().find(|m| m.description == mission_desc) {
                    self.event_log.push(format!("Mission Started: {}", mission.description));
                    self.ongoing_missions.push(OngoingMission {
                        mission: mission.clone(),
                        disciple_indices,
                        ticks_remaining: mission.duration,
                    });
                }
            }
            Action::ClaimRewards(outcome) => {
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
                for idx in outcome.disciple_indices {
                    if let Some(disciple) = self.disciples.get_mut(idx) {
                        disciple.exp += outcome.rewards.disciple_exp;
                    }
                }
                
                self.event_log.push(format!("Mission Rewards Claimed: {} SS, {} XP", outcome.rewards.spirit_stones, outcome.rewards.disciple_exp));
            
                    // Record history
                    self.completed_history.push(outcome.description.clone());
                }
            }
            Action::SaveGame => {
                self.save();
            }
            Action::LoadGame => {
                let loaded_game = self.load(); // This needs to be sync for now, or we handle it differently
                if let Some(mut game) = loaded_game {
                     // Preserve data that isn't saved but needed
                     game.data = self.data.clone(); 
                     *self = game;
                     self.event_log.push("Game loaded successfully.".to_string());
                }
            }
            Action::AssignLaw(disciple_idx, law_id) => {
                if let Some(disciple) = self.disciples.get_mut(disciple_idx) {
                    if self.data.laws.contains_key(&law_id) {
                        disciple.law_id = Some(law_id.clone());
                        self.event_log.push(format!("{} is now practicing {}.", disciple.name, law_id));
                    }
                }
            }
            Action::CraftItem(recipe_id) => {
                // Find recipe
                if let Some(recipe) = self.data.recipes.iter().find(|r| r.id == recipe_id).cloned() {
                    // Check ingredients
                    let mut can_craft = true;
                    for (ing_id, amount) in &recipe.ingredients {
                        match ing_id.as_str() {
                            "spirit_stones" => if self.spirit_stones < *amount { can_craft = false; break; },
                            "herbs" => if self.herbs < *amount { can_craft = false; break; },
                            _ => {
                                let current_amount = *self.inventory.get(ing_id).unwrap_or(&0);
                                if current_amount < *amount { can_craft = false; break; }
                            }
                        }
                    }
                    
                    if can_craft {
                        // Deduct ingredients
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
                        
                        // Add Output
                        *self.inventory.entry(recipe.output_item_id.clone()).or_insert(0) += recipe.output_amount;
                        self.event_log.push(format!("Crafted {}!", recipe.name));
                    } else {
                        self.event_log.push("Cannot Craft: Missing ingredients.".to_string());
                    }
                }
            }
            Action::UseItem(item_id, disciple_idx) => {
                // Check if we have the item
                let count = *self.inventory.get(&item_id).unwrap_or(&0);
                if count > 0 {
                    if let Some(disciple) = self.disciples.get_mut(disciple_idx) {
                        if let Some(item) = self.data.items.get(&item_id).cloned() {
                            // Determine efficiency: herbs (Resources) have 50% efficiency when consumed directly
                            let is_herb = item.item_type == crate::data::items::ItemType::Resource &&
                                         self.data.herbs.contains_key(&item_id);
                            let efficiency = if is_herb { DIRECT_CONSUMPTION_EFFICIENCY } else { 1.0 };

                            // Apply effects with efficiency modifier
                            for effect in &item.effects {
                                match effect {
                                    crate::data::items::ItemEffect::Heal(amt) => {
                                        let effective_amt = ((*amt as f32) * efficiency) as u32;
                                        // No HP yet, maybe clear injury trait?
                                        self.event_log.push(format!("Used {} on {}. Healed {} (Heal not fully impl)", item.name, disciple.name, effective_amt));
                                    },
                                    crate::data::items::ItemEffect::BoostQi(amt) => {
                                        let effective_amt = ((*amt as f32) * efficiency) as u32;
                                        disciple.exp += effective_amt;
                                        if is_herb {
                                            self.event_log.push(format!("{} gained {} Cultivation XP from {} (50% herb efficiency).", disciple.name, effective_amt, item.name));
                                        } else {
                                            self.event_log.push(format!("{} gained {} Cultivation XP from {}.", disciple.name, effective_amt, item.name));
                                        }
                                    },
                                    crate::data::items::ItemEffect::BoostBody(amt) => {
                                        let effective_amt = ((*amt as f32) * efficiency).max(1.0) as u32;
                                        disciple.attributes.body += effective_amt;
                                        self.event_log.push(format!("{}'s Body increased by {}!", disciple.name, effective_amt));
                                    },
                                    crate::data::items::ItemEffect::BoostMind(amt) => {
                                        let effective_amt = ((*amt as f32) * efficiency).max(1.0) as u32;
                                        disciple.attributes.mind += effective_amt;
                                        self.event_log.push(format!("{}'s Mind increased by {}!", disciple.name, effective_amt));
                                    },
                                    crate::data::items::ItemEffect::BoostSpirit(amt) => {
                                        let effective_amt = ((*amt as f32) * efficiency).max(1.0) as u32;
                                        disciple.attributes.spirit += effective_amt;
                                        self.event_log.push(format!("{}'s Spirit increased by {}!", disciple.name, effective_amt));
                                    },
                                }
                            }

                            // Deduct item
                            if let Some(c) = self.inventory.get_mut(&item_id) {
                                *c -= 1;
                            }
                        }
                    }
                } else {
                     self.event_log.push("Item not found in inventory.".to_string());
                }
            }
            Action::ResearchTech(tech_id) => {
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
            Action::RepairBuilding(id) => {
                if let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == id) {
                    if building.status == crate::data::buildings::BuildingStatus::Ruined {
                        let cost = 50; // Flat repair cost for MVP
                        if self.spirit_stones >= cost {
                            self.spirit_stones -= cost;
                            building.status = crate::data::buildings::BuildingStatus::Active;
                            self.event_log.push(format!("Repaired {}!", building.building_type));
                        } else {
                            self.event_log.push("Not enough Spirit Stones to repair (50 required).".to_string());
                        }
                    }
                }
            }
            Action::ConstructBuilding(b_type, x, y) => {
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
                    // Check overlap
                    if !self.data.buildings.iter().any(|b| b.x == x && b.y == y) {
                         self.spirit_stones -= cost;
                         let mut new_b = crate::data::buildings::Building::new(b_type.clone());
                         new_b.id = rand::random(); // Simple ID
                         new_b.x = x;
                         new_b.y = y;
                         new_b.element = element; // Set element from definition
                         new_b.status = crate::data::buildings::BuildingStatus::Active; // Instant build for MVP
                         self.data.buildings.push(new_b);
                         self.event_log.push(format!("Constructed {} at {},{}", b_type, x, y));
                    } else {
                        self.event_log.push("Cannot build: Tile occupied.".to_string());
                    }
                } else {
                    self.event_log.push(format!("Not enough Spirit Stones ({} required).", cost));
                }
            }
            Action::StartNewGame(name) => {
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
                self.ongoing_missions.clear();
                self.completed_missions.clear();
                self.completed_history.clear();
                self.deceased_disciples.clear();
                self.event_log = vec!["The sect has fallen... We must rebuild.".to_string()];
                self.tutorial = crate::state::TutorialState::new();
                self.tick = 0;
                self.current_season = Season::Spring;
                self.season_ticks = 3600;
                self.grid = Grid::new(20, 20); // Reset Grid

                // Reset Buildings (manually for now as they are part of GameData which is shared... wait.
                // GameData is loaded once. Building STATE (levels) is inside GameData.buildings.
                // We need to reset the levels.
                // Resetting to empty is better for blank map start.
                self.data.buildings.clear();

                // Start with a ruined Sect Hall to restore
                let mut sect_hall = crate::data::buildings::Building::new(BuildingType::SectHall);
                sect_hall.id = rand::random();
                sect_hall.x = 10;
                sect_hall.y = 10;
                sect_hall.element = self
                    .data
                    .building_definitions
                    .get(&BuildingType::SectHall)
                    .map(|d| d.element.clone())
                    .unwrap_or_default();
                sect_hall.status = crate::data::buildings::BuildingStatus::Ruined;
                self.data.buildings.push(sect_hall);

                self.transition(StateTransition::ToSectBase);
            }
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
        }
    }

    /// Process herbs in the drying pavilion
    fn process_drying(&mut self, building_id: u64, herb_id: &str) {
        // Find building
        let building = match self.data.buildings.iter().find(|b| b.id == building_id) {
            Some(b) => b.clone(),
            None => {
                self.event_log.push("Building not found.".to_string());
                return;
            }
        };

        if building.building_type != BuildingType::DryingPavilion {
            self.event_log.push("This is not a Drying Pavilion.".to_string());
            return;
        }

        // Check if we have the herb (and it's not already dried)
        if herb_id.starts_with("dried_") {
            self.event_log.push("Already dried.".to_string());
            return;
        }

        let current_count = *self.inventory.get(herb_id).unwrap_or(&0);
        if current_count < 5 {
            self.event_log.push(format!("Need at least 5 {} to dry (have {}).", herb_id, current_count));
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
            "Dried 5 {} -> {} {}.",
            herb_id, output_amount, dried_id
        ));
    }

    /// Set or clear greenhouse elemental infusion
    fn set_greenhouse_infusion(&mut self, building_id: u64, element: Option<crate::data::elements::Element>) {
        if let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == building_id) {
            if building.building_type != BuildingType::Greenhouse {
                self.event_log.push("This is not a Greenhouse.".to_string());
                return;
            }

            if let Some(ref elem) = element {
                // Check for required materials (simplified: 1 herb of matching element per season)
                // For MVP, just set it - material cost could be checked per season tick
                building.infused_element = Some(elem.clone());
                self.event_log.push(format!("Greenhouse infused with {} element.", elem));
            } else {
                building.infused_element = None;
                self.event_log.push("Greenhouse infusion cleared.".to_string());
            }
        }
    }

    fn save(&self) {
        let buildings_to_save = self.data.buildings.clone();

        let save_data = SaveData {
            grid: Some(self.grid.clone()),
            sect_name: self.sect_name.clone(),
            spirit_stones: self.spirit_stones,
            herbs: self.herbs,
            influence: self.influence,
            relics: self.relics,
            inventory: self.inventory.clone(),
            unlocked_techs: self.unlocked_techs.clone(),
            disciples: self.disciples.clone(),
            deceased_disciples: self.deceased_disciples.clone(),
            buildings: buildings_to_save,
            ongoing_missions: self.ongoing_missions.clone(),
            completed_missions: self.completed_missions.clone(),
            completed_history: self.completed_history.clone(),
            tick: self.tick,
        };

        // Platform specific save
        #[cfg(not(target_arch = "wasm32"))]
        {
            let json = serde_json::to_string_pretty(&save_data).expect("Failed to serialize save");
            if let Err(e) = fs::write("savegame.json", json) {
                // Cant log to self.event_log here easily as self is immutable ref
                println!("Failed to save: {}", e);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let json = serde_json::to_string(&save_data).expect("Failed to serialize save");
            LocalStorage::set("cultivation_save", &json);
        }
    }

    fn load(&self) -> Option<Self> {
        // Platform specific load
        let json_content = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                fs::read_to_string("savegame.json").ok()
            }

            #[cfg(target_arch = "wasm32")]
            {
                LocalStorage::get("cultivation_save")
            }
        };

        if let Some(json) = json_content {
            if let Ok(save_data) = serde_json::from_str::<SaveData>(&json) {
                // Reconstruct Game
                let data = self.data.clone(); // In a real engine, we might reload data, but here we reuse
                // Important: We need to overwrite building states in data with saved ones
                // But data is shared immutable usually? In this struct it's owned `data: GameData`.

                let mut new_game = Self {
                    state: GameState::SectBase(SectBaseState::new()), // Default to base on load
                    data: data.clone(),
                    grid: save_data.grid.unwrap_or(Grid::new(20, 20)),
                    sect_name: save_data.sect_name,
                    spirit_stones: save_data.spirit_stones,
                    herbs: save_data.herbs,
                    influence: save_data.influence,
                    relics: save_data.relics,
                    inventory: save_data.inventory,
                    unlocked_techs: save_data.unlocked_techs,
                    disciples: save_data.disciples,
                    deceased_disciples: save_data.deceased_disciples,
                    ongoing_missions: save_data.ongoing_missions,
                    completed_missions: save_data.completed_missions,
                    completed_history: save_data.completed_history,
                    event_log: Vec::new(),
                    tutorial: crate::state::TutorialState::new(),
                    tick: save_data.tick,
                    current_season: Season::Spring, // TODO: save/load season
                    season_ticks: 3600,
                };

                // Restore building states
                // save_data.buildings has the modified buildings.
                // data.buildings (from json) has defaults.
                new_game.data.buildings = save_data.buildings;

                return Some(new_game);
            }
        }
        None
    }

    /// Process herb growth and harvesting in herb gardens
    fn process_herb_gardens(&mut self) {
        let mut harvested_herbs: Vec<(String, u32)> = Vec::new();
        let mut log_messages: Vec<String> = Vec::new();

        // Get disciple info for quality calculation
        let disciple_spirits: std::collections::HashMap<u64, u32> = self.disciples.iter()
            .map(|d| (d.id, d.attributes.spirit))
            .collect();

        for building in self.data.buildings.iter_mut() {
            if building.building_type != BuildingType::HerbGarden &&
               building.building_type != BuildingType::Greenhouse {
                continue;
            }

            // Sync plots with building level
            building.sync_herb_plots();

            let growth_multiplier = building.get_growth_speed_multiplier();
            let has_worker = building.assigned_disciple.is_some();
            let worker_spirit = building.assigned_disciple
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
                                "Harvested {}x {} from {}.",
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
                                    "A {} withered in {} - no worker to harvest!",
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
    fn apply_herb_decay(&mut self) {
        // Calculate total decay reduction from Herb Storage buildings
        let storage_reduction: f32 = self.data.buildings.iter()
            .filter(|b| b.building_type == BuildingType::HerbStorage &&
                       b.status == crate::data::buildings::BuildingStatus::Active)
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
                        decay_messages.push(format!("{} {} decayed.", actual_decay, herb_id));
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
        if building.building_type != BuildingType::HerbGarden &&
           building.building_type != BuildingType::Greenhouse {
            return false;
        }

        // Check tier restrictions
        if herb.tier > building.get_max_herb_tier() {
            self.event_log.push(format!(
                "Cannot plant {} - tier {} exceeds {} capacity.",
                herb.name, herb.tier, building.building_type
            ));
            return false;
        }

        // Check season (unless greenhouse with infusion)
        let can_grow_this_season = herb.grow_seasons.contains(&self.current_season) ||
            (building.building_type == BuildingType::Greenhouse &&
             building.infused_element.as_ref() == Some(&herb.element));

        if !can_grow_this_season {
            self.event_log.push(format!(
                "Cannot plant {} - wrong season ({}).",
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
            self.event_log.push("Plot is already occupied.".to_string());
            return false;
        }

        // Plant the herb
        let growing = GrowingHerb::new(herb_id.to_string(), herb.grow_time_ticks);
        building.herb_plots[plot_index].growing = Some(growing);
        self.event_log.push(format!("Planted {} in {}.", herb.name, building.building_type));
        true
    }

    /// Assign a disciple to work a building
    pub fn assign_disciple_to_building(&mut self, building_id: u64, disciple_id: Option<u64>) -> bool {
        // If assigning, validate disciple exists and is Outer rank
        if let Some(d_id) = disciple_id {
            let is_valid_worker = self.disciples.iter()
                .any(|d| d.id == d_id && d.rank == DiscipleRank::Outer);
            if !is_valid_worker {
                self.event_log.push("Only Outer Disciples can be assigned to work buildings.".to_string());
                return false;
            }

            // Check if disciple is already assigned elsewhere
            let already_assigned = self.data.buildings.iter()
                .any(|b| b.assigned_disciple == Some(d_id));
            if already_assigned {
                self.event_log.push("This disciple is already assigned to another building.".to_string());
                return false;
            }
        }

        // Find and update building
        if let Some(building) = self.data.buildings.iter_mut().find(|b| b.id == building_id) {
            building.assigned_disciple = disciple_id;
            if disciple_id.is_some() {
                self.event_log.push(format!("Assigned disciple to {}.", building.building_type));
            } else {
                self.event_log.push(format!("Removed assignment from {}.", building.building_type));
            }
            return true;
        }
        false
    }
}
