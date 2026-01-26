use crate::data::disciples::{Disciple, DiscipleRank, Talent};
use crate::data::grid::Grid;
use crate::data::buildings::BuildingType;
use crate::data::history::DeceasedDisciple;
use crate::data::loader::GameData;
use crate::data::missions::{MissionOutcome, MissionRewards, OngoingMission, RelevantStat};
use crate::engine::actions::Action;
use crate::engine::proc_gen::generate_disciple;
use crate::state::{
    library::LibraryState, main_menu::MainMenuState,
    mission_assignment::MissionAssignmentState, mission_resolution::MissionResolutionState,
    roster::DiscipleRosterState, sect_base::SectBaseState, sect_creation::SectCreationState, world_map::WorldMapState, GameState,
    StateTransition,
};
use rand::Rng;
use crate::save::SaveData;
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
    tick: u64,
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
            tick: 0,
        }
    }

    pub fn update(&mut self) {
        self.tick += 1;

        if self.tick % 60 == 0 {
            // Cultivation Tick
            let mut disciples_to_breakthrough = Vec::new();
            for (i, disciple) in self.disciples.iter().enumerate() {
                if disciple.exp >= disciple.exp_to_next_level {
                    disciples_to_breakthrough.push(i);
                }
            }
            
            // Process breakthroughs and handle deaths
            let mut indices_to_remove = Vec::new();
            for i in disciples_to_breakthrough {
                let mut disciple = self.disciples[i].clone();
                let died = self.attempt_breakthrough(&mut disciple);
                if died {
                    // Record in hall of fallen
                    self.deceased_disciples.push(DeceasedDisciple::new(
                        disciple.name.clone(),
                        disciple.realm.clone(),
                        "Failed Breakthrough".to_string(),
                        self.tick,
                    ));
                    indices_to_remove.push(i);
                } else {
                    self.disciples[i] = disciple;
                }
            }
            // Remove dead disciples (in reverse to preserve indices)
            for i in indices_to_remove.into_iter().rev() {
                self.disciples.remove(i);
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
            
            for disciple in &mut self.disciples {
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
            GameState::SectBase(s) => s.update(&mut self.data, &mut self.grid, self.spirit_stones, self.herbs, self.influence, self.relics, &self.inventory, &self.unlocked_techs, &self.event_log, &self.ongoing_missions, &self.completed_missions, &self.completed_history),
            GameState::DiscipleRoster(s) => s.update(&self.data, &self.disciples, &self.inventory),
            GameState::WorldMap(s) => s.update(&self.data),
            GameState::MissionResolution(s) => s.update(&mut self.completed_missions),
            GameState::Library(s) => s.update(&self.data, self.spirit_stones, &self.deceased_disciples),
            GameState::MissionAssignment(s) => s.update(&self.disciples),
            GameState::SectCreation(s) => s.update(),
        };

        if let Some(action) = update_result.action {
            self.execute_action(action);
        }
        if let Some(transition) = update_result.transition {
            self.transition(transition);
        }
    }

    pub fn draw(&self) {
        match &self.state {
            GameState::MainMenu(s) => s.draw(&self.data, self.spirit_stones),
            GameState::SectBase(s) => s.draw(&self.data, &self.grid, self.spirit_stones),
            GameState::DiscipleRoster(s) => s.draw(&self.data, &self.disciples, self.spirit_stones),
            GameState::WorldMap(s) => s.draw(&self.data, self.spirit_stones),
            GameState::MissionResolution(s) => s.draw(&self.data, self.spirit_stones),
            GameState::Library(s) => s.draw(&self.data, self.spirit_stones, &self.deceased_disciples),
            GameState::MissionAssignment(s) => s.draw(&self.data, &self.disciples, self.spirit_stones),
            GameState::SectCreation(s) => s.draw(&self.data),
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
            // Find current stage definition
             if let Some((stage_idx, stage)) = self.data.stages.iter().enumerate().find(|(_, s)| s.id == disciple.realm) {
                // Check if can advance sub-stage
                if disciple.sub_stage < stage.sub_stages.len().saturating_sub(1) {
                     // Minor breakthrough
                     disciple.sub_stage += 1;
                     self.event_log.push(format!("{} advanced to {} - {}!", disciple.name, stage.name, stage.sub_stages[disciple.sub_stage].name));
                     // Reset XP for next sub-stage (scaling?)
                     disciple.exp = 0;
                     // Increase requirement slightly for sub-stages
                     disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 1.2) as u32;
                } else {
                     // Check for Major breakthrough (Next Stage)
                     if let Some(next_stage) = self.data.stages.get(stage_idx + 1) {
                         // Determine if this is a Major Realm breakthrough (Tribulation Trigger)
                         // For MVP, assume explicit IDs or look for specific transitions
                         // Currently: Mortal -> Qi Refinement (Safe), QiRef -> Foundation (Safe), 
                         // Foundation -> Core Formation (TRIBULATION NEEDED)
                         
                         let needs_tribulation = match disciple.realm.as_str() {
                             "foundation_establishment" => true, // To Golden Core
                             "golden_core" => true, // To Nascent Soul
                             _ => false,
                         };
                         
                         if needs_tribulation {
                             // Trigger Tribulation!
                             self.event_log.push(format!("Tribulation clouds gather above {}...", disciple.name));
                             
                             let t_type = match disciple.realm.as_str() {
                                 "foundation_establishment" => crate::engine::tribulation::TribulationType::GoldenCore,
                                 "golden_core" => crate::engine::tribulation::TribulationType::NascentSoul,
                                 _ => crate::engine::tribulation::TribulationType::GoldenCore,
                             };
                             
                             let trib_state = crate::engine::tribulation::TribulationState::new(t_type, disciple);
                             
                             // Find disciple index for state
                             // This is tricky because attempt_breakthrough is called inside a loop over indices.
                             // We need to pass the index or handle state transition queueing.
                             // Wait, attempt_breakthrough returns bool (died).
                             // We can't immediately transition state here because we are in the middle of an update loop.
                             // Refactor: Queue the transition?
                             // Correction: Game update handles transition at end. We can return a transition?
                             // But attempt_breakthrough is a helper.
                             // Compromise: We will handle this by returning a specialized enum or mutating a "pending_transition" field?
                             // Simpler: Just allow it to happen instantly? No, we need UI.
                             // Hack for MVP: We can't easily break the loop.
                             // We will just mark it as "Pending Tribulation" on the disciple and handle it in the next tick?
                             // Or just trigger it for the FIRST one found and ignore others this tick?
                             
                             // Let's assume only 1 major event per tick for now.
                             // We need the index. `attempt_breakthrough` doesn't have it.
                             // Let's modify call signature or logic.
                             // Actually, let's just do the mutation here.
                             // But we need to transition the GAME STATE.
                             // We can't do `self.transition` because we are borrowing `self` mutably for `disciples`.
                             // Actually `disciples` is being iterated.
                             // We are in `self.update()`, specifically:
                             // `for (i, disciple) in self.disciples.iter().enumerate()`
                             
                             // Since we are refactoring, let's change `attempt_breakthrough` to return a `BreakthroughResult` enum.
                             
                             // For this step, I'll return `Breaking(TribulationState)` and handle it in the loop.
                             // Resuming standard logic...
                             
                             return BreakthroughResult::Tribulation(t_type);
                         } else {
                             // Instant Success
                             disciple.realm = next_stage.id.clone();
                             disciple.sub_stage = 0;
                             self.event_log.push(format!("{} broke through to {} realm!", disciple.name, next_stage.name));
                             
                             disciple.exp = 0;
                             disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 2.5) as u32;
                         }
                     } else {
                         // Pinnacle
                         self.event_log.push(format!("{} has reached the apex of this world.", disciple.name));
                         disciple.exp = disciple.exp_to_next_level; // Cap it
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
                BreakthroughResult::None // Did not die
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
                let realm_power = self.data.stages.iter()
                    .find(|s| s.id == disciple.realm)
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
                let new_disciple = generate_disciple(&self.data);
                self.event_log.push(format!("Recruited: {}", new_disciple.name));
                self.disciples.push(new_disciple);
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
                        if let Some(item) = self.data.items.get(&item_id) {
                            // Apply effects
                            for effect in &item.effects {
                                match effect {
                                    crate::data::items::ItemEffect::Heal(_amt) => {
                                        // No HP yet, maybe clear injury trait?
                                        self.event_log.push(format!("Used {} on {}. (Heal not fully impl)", item.name, disciple.name));
                                    },
                                    crate::data::items::ItemEffect::BoostQi(amt) => {
                                        disciple.exp += amt; // Treat Qi Boost as XP for now
                                        self.event_log.push(format!("{} gained {} Cultivation XP from {}.", disciple.name, amt, item.name));
                                    },
                                    crate::data::items::ItemEffect::BoostBody(amt) => {
                                        disciple.attributes.body += amt;
                                        self.event_log.push(format!("{}'s Body increased by {}!", disciple.name, amt));
                                    },
                                    crate::data::items::ItemEffect::BoostMind(amt) => {
                                        disciple.attributes.mind += amt;
                                        self.event_log.push(format!("{}'s Mind increased by {}!", disciple.name, amt));
                                    },
                                    crate::data::items::ItemEffect::BoostSpirit(amt) => {
                                        disciple.attributes.spirit += amt;
                                        self.event_log.push(format!("{}'s Spirit increased by {}!", disciple.name, amt));
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
                self.tick = 0;
                self.grid = Grid::new(20, 20); // Reset Grid
                
                // Reset Buildings (manually for now as they are part of GameData which is shared... wait. 
                // GameData is loaded once. Building STATE (levels) is inside GameData.buildings.
                // We need to reset the levels.
                // Resetting to empty is better for blank map start.
                self.data.buildings.clear();

                self.transition(StateTransition::ToSectBase);
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
                    tick: save_data.tick, 
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
}
