use crate::data::disciples::{CultivationRealm, Disciple, DiscipleRank, Talent};
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
    pub sect_name: String,
    pub spirit_stones: u32,
    pub herbs: u32,
    pub disciples: Vec<Disciple>,
    pub deceased_disciples: Vec<DeceasedDisciple>,
    pub ongoing_missions: Vec<OngoingMission>,
    pub completed_missions: Vec<MissionOutcome>,
    pub event_log: Vec<String>,
    tick: u64,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().expect("Failed to load game data");
        // Scenario: Survivors of the Fallen Sect
        // 1. The Patriarch (Sect Leader)
        let mut leader = generate_disciple(&data);
        leader.name = "Patriarch".to_string(); // User will name sect/player later? Or just rank title?
        leader.rank = DiscipleRank::SectLeader;
        leader.realm = CultivationRealm::FoundationEstablishment;
        leader.attributes.spirit += 10;
        leader.attributes.mind += 5;
        leader.max_qi = 500;
        leader.qi = 500;

        // 2. Loyal Workers (Outer Disciples)
        let mut d1 = generate_disciple(&data);
        d1.rank = DiscipleRank::Outer;
        d1.talent = Talent::Low; // Hard workers, not geniuses
        
        let mut d2 = generate_disciple(&data);
        d2.rank = DiscipleRank::Outer;
        d2.talent = Talent::Low;

        let disciples = vec![leader, d1, d2];
        
        Self {
            state: GameState::MainMenu(MainMenuState::new()),
            data,
            sect_name: "Unnamed Sect".to_string(), // Initial placeholder
            spirit_stones: 50, // Reduced from 100 - hard times
            herbs: 10,         // Some supplies
            disciples,
            deceased_disciples: Vec::new(),
            ongoing_missions: Vec::new(),
            completed_missions: Vec::new(),
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
            let cultivation_multiplier = self.data.buildings
                .get(&BuildingType::TrainingYard)
                .map(|b| b.get_cultivation_multiplier())
                .unwrap_or(1.0);
            
            for disciple in &mut self.disciples {
                let base_exp = 1 + (disciple.attributes.spirit / 5);
                let bonus_exp = (base_exp as f32 * cultivation_multiplier) as u32;
                disciple.exp += bonus_exp;
            }

            // Apply Spirit Garden passive income
            // Apply Spirit Garden passive income (Feature 9.1.3: Requires Outer Disciples)
            if let Some(garden) = self.data.buildings.get(&BuildingType::SpiritGarden) {
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
        }

        let update_result = match &mut self.state {
            GameState::MainMenu(s) => s.update(),
            GameState::SectBase(s) => s.update(&self.data, self.spirit_stones, self.herbs, &self.event_log),
            GameState::DiscipleRoster(s) => s.update(&self.disciples),
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
            GameState::SectBase(s) => s.draw(&self.data, self.spirit_stones),
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

    /// Attempts a breakthrough. Returns `true` if the disciple died.
    fn attempt_breakthrough(&mut self, disciple: &mut Disciple) -> bool {
        let mut rng = rand::thread_rng();
        let base_chance = match disciple.talent {
            Talent::Low => 0.3,
            Talent::Medium => 0.5,
            Talent::High => 0.7,
            Talent::Genius => 0.9,
            Talent::HeavenSent => 1.0,
        };

        // Apply trait modifiers
        let breakthrough_modifier: f32 = disciple.fate_traits.iter().map(|t| t.breakthrough_modifier).sum();
        let success_chance = (base_chance + breakthrough_modifier).clamp(0.05, 0.99);
        
        // println!("[Breakthrough] {} attempts...", disciple.name); // Optional debugging

        if rng.gen::<f32>() < success_chance {
            let next_realm = match disciple.realm {
                CultivationRealm::Mortal => Some(CultivationRealm::QiRefinement),
                CultivationRealm::QiRefinement => Some(CultivationRealm::FoundationEstablishment),
                CultivationRealm::FoundationEstablishment => Some(CultivationRealm::CoreFormation),
                CultivationRealm::CoreFormation => None,
            };
            if let Some(realm) = next_realm {
                self.event_log.push(format!("{} reached {:?}!", disciple.name, realm));
                disciple.realm = realm;
                disciple.exp = 0;
                disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 2.5) as u32;
            } else {
                self.event_log.push(format!("{} is already a pinnacle existence.", disciple.name));
                disciple.exp = disciple.exp_to_next_level;
            }
            false // Did not die
        } else {
            // Failed breakthrough - check for death
            let injury_modifier: f32 = disciple.fate_traits.iter().map(|t| t.injury_modifier).sum();
            let death_chance = (0.1 + injury_modifier).clamp(0.0, 0.5); // Base 10% death on failure
            
            if rng.gen::<f32>() < death_chance {
                self.event_log.push(format!("{} perished attempting to break through!", disciple.name));
                true // Died
            } else {
                self.event_log.push(format!("{} failed breakthrough and suffered internal injuries.", disciple.name));
                disciple.exp = (disciple.exp as f32 * 0.5) as u32; // Lose more EXP
                // Could add injury trait here in future
                false // Did not die
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
                // Realm power
                let realm_power = match disciple.realm {
                    CultivationRealm::Mortal => 1,
                    CultivationRealm::QiRefinement => 2,
                    CultivationRealm::FoundationEstablishment => 4,
                    CultivationRealm::CoreFormation => 7,
                };
                
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
                
                logs.push(format!("{}: Realm {} + Attr {} = {}", disciple.name, realm_power, attr_power, total_power));
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
             MissionRewards {
                 spirit_stones: ongoing.mission.danger_level * 50,
                 disciple_exp: ongoing.mission.danger_level * 100,
                 herbs: if matches!(ongoing.mission.mission_type, crate::data::missions::MissionType::ResourceGathering | crate::data::missions::MissionType::Exploration) {
                     ongoing.mission.danger_level * 5
                 } else { 0 },
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
                        if let Some(building) = self.data.buildings.get_mut(&building_type) {
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
                    let exp_gain = outcome.rewards.disciple_exp;
                    for &idx in &outcome.disciple_indices {
                        if let Some(disciple) = self.disciples.get_mut(idx) {
                             disciple.exp += exp_gain;
                        }
                    }
                    self.event_log.push(format!("Rewards: {} SS, {} Herbs, {} EXP", 
                        outcome.rewards.spirit_stones, outcome.rewards.herbs, outcome.rewards.disciple_exp));
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
            Action::StartNewGame(name) => {
                // We're essentially resetting, but keeping the name.
                // However, self.new() is async and creates everything fresh.
                // We should probably just trigger a state change here OR manually reset fields.
                // For MVP, knowing that new() creates the 'Survivor' preset, we can just apply the name.
                // BUT, new() is static. 
                
                // Let's manually reset to "New Game" state with our preset logic:
                let data = self.data.clone();
                
                // 1. Leader
                let mut leader = generate_disciple(&data);
                leader.name = "Patriarch".to_string(); 
                leader.rank = DiscipleRank::SectLeader;
                leader.realm = CultivationRealm::FoundationEstablishment;
                leader.attributes.spirit += 10;
                leader.attributes.mind += 5;
                leader.max_qi = 500;
                leader.qi = 500;

                // 2. Workers
                let mut d1 = generate_disciple(&data);
                d1.rank = DiscipleRank::Outer;
                d1.talent = Talent::Low;
                
                let mut d2 = generate_disciple(&data);
                d2.rank = DiscipleRank::Outer;
                d2.talent = Talent::Low;

                self.disciples = vec![leader, d1, d2];
                self.sect_name = name;
                self.spirit_stones = 50;
                self.herbs = 10;
                self.ongoing_missions.clear();
                self.completed_missions.clear();
                self.deceased_disciples.clear();
                self.event_log = vec!["The sect has fallen... We must rebuild.".to_string()];
                self.tick = 0;
                
                // Reset Buildings (manually for now as they are part of GameData which is shared... wait. 
                // GameData is loaded once. Building STATE (levels) is inside GameData.buildings.
                // We need to reset the levels.
                for building in self.data.buildings.values_mut() {
                    building.level = 1;
                }

                self.transition(StateTransition::ToSectBase);
            }
        }
    }

    fn save(&self) {
        let buildings_to_save: Vec<_> = self.data.buildings.values().cloned().collect();

        let save_data = SaveData {
            sect_name: self.sect_name.clone(),
            spirit_stones: self.spirit_stones,
            herbs: self.herbs,
            disciples: self.disciples.clone(),
            deceased_disciples: self.deceased_disciples.clone(),
            buildings: buildings_to_save,
            ongoing_missions: self.ongoing_missions.clone(),
            completed_missions: self.completed_missions.clone(),
            tick: self.tick,
        };

        match serde_json::to_string_pretty(&save_data) {
            Ok(json) => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if let Err(e) = fs::write("save.json", json) {
                        println!("Failed to write save file: {}", e);
                    } else {
                        println!("Game saved!");
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    LocalStorage::set("save_data", &json);
                    println!("Game saved to LocalStorage!");
                }
            }
            Err(e) => {
                println!("Failed to serialize save data: {}", e);
            }
        }
    }

    fn load(&self) -> Option<Self> {
        let json_content = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                 fs::read_to_string("save.json").ok()
            }
            #[cfg(target_arch = "wasm32")]
            {
                 LocalStorage::get("save_data").ok()
            }
        };

        if let Some(json) = json_content {
             match serde_json::from_str::<SaveData>(&json) {
                 Ok(save_data) => {
                      // We need to reconstruct the full Game struct. 
                      // Note: We are creating a NEW game instance here mostly, but overwriting fields.
                      // Since 'new' is async, we can't easily call it here synchronously. 
                      // Ideally, LoadGame should probably be async or we construct manually.
                      // Let's construct manually since we have the data.
                      
                      // We need 'data' loaded. We can clone it from self since 'load' is method on &self (which is the current game).
                      let mut game_data = self.data.clone();
                      
                      // Update building levels from save
                      for saved_building in &save_data.buildings {
                          if let Some(existing) = game_data.buildings.get_mut(&saved_building.building_type) {
                               existing.level = saved_building.level;
                          }
                      }

                       Some(Self {
                          state: GameState::MainMenu(MainMenuState::new()), // Reset to main menu or keep current? Let's reset.
                          data: game_data,
                          sect_name: save_data.sect_name,
                          spirit_stones: save_data.spirit_stones,
                          herbs: save_data.herbs,
                          disciples: save_data.disciples,
                          deceased_disciples: save_data.deceased_disciples,
                          ongoing_missions: save_data.ongoing_missions,
                          completed_missions: save_data.completed_missions,
                          event_log: vec!["Game loaded successfully.".to_string()],
                          tick: save_data.tick,
                      })
                 }
                 Err(e) => {
                     println!("Failed to deserialize save data: {}", e);
                     None
                 }
             }
        } else {
             println!("No save file found.");
             None
        }
    }
}
