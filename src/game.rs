use crate::data::disciples::{CultivationRealm, Disciple, Talent};
use crate::data::loader::GameData;
use crate::data::missions::OngoingMission;
use crate::engine::actions::Action;
use crate::engine::proc_gen::generate_disciple;
use crate::state::{
    library::LibraryState, main_menu::MainMenuState,
    mission_assignment::MissionAssignmentState, mission_resolution::MissionResolutionState,
    roster::DiscipleRosterState, sect_base::SectBaseState, world_map::WorldMapState, GameState,
    StateTransition,
};
use rand::Rng;

pub struct Game {
    pub state: GameState,
    pub data: GameData,
    pub spirit_stones: u32,
    pub disciples: Vec<Disciple>,
    pub ongoing_missions: Vec<OngoingMission>,
    tick: u64,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().expect("Failed to load game data");
        let disciples = vec![generate_disciple(&data), generate_disciple(&data)];
        Self {
            state: GameState::MainMenu(MainMenuState::new()),
            data,
            spirit_stones: 100,
            disciples,
            ongoing_missions: Vec::new(),
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
            for i in disciples_to_breakthrough {
                let mut disciple = self.disciples[i].clone();
                self.attempt_breakthrough(&mut disciple);
                self.disciples[i] = disciple;
            }
            for disciple in &mut self.disciples {
                disciple.exp += 1 + (disciple.attributes.spirit / 5);
            }

            // Mission Tick
            for mission in &mut self.ongoing_missions {
                mission.ticks_remaining = mission.ticks_remaining.saturating_sub(1);
            }
            self.ongoing_missions.retain(|mission| {
                if mission.ticks_remaining == 0 {
                    println!("Mission '{}' has completed!", mission.mission.description);
                    // In the future, this would trigger a resolution event
                    false // Remove from list
                } else {
                    true // Keep in list
                }
            });
        }

        let update_result = match &mut self.state {
            GameState::MainMenu(s) => s.update(),
            GameState::SectBase(s) => s.update(&self.data),
            GameState::DiscipleRoster(s) => s.update(),
            GameState::WorldMap(s) => s.update(),
            GameState::MissionResolution(s) => s.update(),
            GameState::Library(s) => s.update(),
            GameState::MissionAssignment(s) => s.update(&self.disciples),
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
            GameState::Library(s) => s.draw(&self.data, self.spirit_stones),
            GameState::MissionAssignment(s) => s.draw(&self.data, &self.disciples, self.spirit_stones),
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
        };
    }

    fn attempt_breakthrough(&mut self, disciple: &mut Disciple) {
        let mut rng = rand::thread_rng();
        let success_chance = match disciple.talent {
            Talent::Low => 0.3,
            Talent::Medium => 0.5,
            Talent::High => 0.7,
            Talent::Genius => 0.9,
            Talent::HeavenSent => 1.0,
        };
        if rng.gen::<f32>() < success_chance {
            let next_realm = match disciple.realm {
                CultivationRealm::Mortal => Some(CultivationRealm::QiRefinement),
                CultivationRealm::QiRefinement => Some(CultivationRealm::FoundationEstablishment),
                CultivationRealm::FoundationEstablishment => Some(CultivationRealm::CoreFormation),
                CultivationRealm::CoreFormation => None,
            };
            if let Some(realm) = next_realm {
                println!("{} has broken through to {:?}!", disciple.name, realm);
                disciple.realm = realm;
                disciple.exp = 0;
                disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 2.5) as u32;
            } else {
                println!("{} is already at the highest realm.", disciple.name);
                disciple.exp = disciple.exp_to_next_level;
            }
        } else {
            println!("{}'s breakthrough has failed!", disciple.name);
            disciple.exp = (disciple.exp as f32 * 0.75) as u32;
        }
    }

use crate::save::SaveData;
use std::fs;

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().expect("Failed to load game data");
        let disciples = vec![generate_disciple(&data), generate_disciple(&data)];
        Self {
            state: GameState::MainMenu(MainMenuState::new()),
            data,
            spirit_stones: 100,
            disciples,
            ongoing_missions: Vec::new(),
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
            for i in disciples_to_breakthrough {
                let mut disciple = self.disciples[i].clone();
                self.attempt_breakthrough(&mut disciple);
                self.disciples[i] = disciple;
            }
            for disciple in &mut self.disciples {
                disciple.exp += 1 + (disciple.attributes.spirit / 5);
            }

            // Mission Tick
            for mission in &mut self.ongoing_missions {
                mission.ticks_remaining = mission.ticks_remaining.saturating_sub(1);
            }
            self.ongoing_missions.retain(|mission| {
                if mission.ticks_remaining == 0 {
                    println!("Mission '{}' has completed!", mission.mission.description);
                    // In the future, this would trigger a resolution event
                    false // Remove from list
                } else {
                    true // Keep in list
                }
            });
        }

        let update_result = match &mut self.state {
            GameState::MainMenu(s) => s.update(),
            GameState::SectBase(s) => s.update(&self.data),
            GameState::DiscipleRoster(s) => s.update(),
            GameState::WorldMap(s) => s.update(),
            GameState::MissionResolution(s) => s.update(),
            GameState::Library(s) => s.update(),
            GameState::MissionAssignment(s) => s.update(&self.disciples),
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
            GameState::Library(s) => s.draw(&self.data, self.spirit_stones),
            GameState::MissionAssignment(s) => s.draw(&self.data, &self.disciples, self.spirit_stones),
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
        };
    }

    fn attempt_breakthrough(&mut self, disciple: &mut Disciple) {
        let mut rng = rand::thread_rng();
        let success_chance = match disciple.talent {
            Talent::Low => 0.3,
            Talent::Medium => 0.5,
            Talent::High => 0.7,
            Talent::Genius => 0.9,
            Talent::HeavenSent => 1.0,
        };
        if rng.gen::<f32>() < success_chance {
            let next_realm = match disciple.realm {
                CultivationRealm::Mortal => Some(CultivationRealm::QiRefinement),
                CultivationRealm::QiRefinement => Some(CultivationRealm::FoundationEstablishment),
                CultivationRealm::FoundationEstablishment => Some(CultivationRealm::CoreFormation),
                CultivationRealm::CoreFormation => None,
            };
            if let Some(realm) = next_realm {
                println!("{} has broken through to {:?}!", disciple.name, realm);
                disciple.realm = realm;
                disciple.exp = 0;
                disciple.exp_to_next_level = (disciple.exp_to_next_level as f32 * 2.5) as u32;
            } else {
                println!("{} is already at the highest realm.", disciple.name);
                disciple.exp = disciple.exp_to_next_level;
            }
        } else {
            println!("{}'s breakthrough has failed!", disciple.name);
            disciple.exp = (disciple.exp as f32 * 0.75) as u32;
        }
    }

    fn execute_action(&mut self, action: Action) {
        match action {
            Action::UpgradeBuilding(building_type) => {
                let cost = 50;
                if self.spirit_stones >= cost {
                    if let Some(building) = self.data.buildings.get_mut(&building_type) {
                        self.spirit_stones -= cost;
                        building.level += 1;
                        println!("Upgraded {:?} to level {}", building.building_type, building.level);
                    }
                }
            }
            Action::RecruitDisciple => {
                let new_disciple = generate_disciple(&self.data);
                println!("Recruited new disciple: {}", new_disciple.name);
                self.disciples.push(new_disciple);
            }
            Action::StartMission(mission_desc, disciple_indices) => {
                if let Some(mission) = self.data.missions.iter().find(|m| m.description == mission_desc) {
                    println!("Starting mission '{}' with {} disciples.", mission.description, disciple_indices.len());
                    self.ongoing_missions.push(OngoingMission {
                        mission: mission.clone(),
                        disciple_indices,
                        ticks_remaining: mission.duration,
                    });
                }
            }
            Action::SaveGame => {
                self.save();
            }
        }
    }

    fn save(&self) {
        let buildings_to_save: Vec<_> = self.data.buildings.values().cloned().collect();

        let save_data = SaveData {
            spirit_stones: self.spirit_stones,
            disciples: self.disciples.clone(),
            buildings: buildings_to_save,
            ongoing_missions: self.ongoing_missions.clone(),
            tick: self.tick,
        };

        match serde_json::to_string_pretty(&save_data) {
            Ok(json) => {
                if let Err(e) = fs::write("save.json", json) {
                    println!("Failed to write save file: {}", e);
                } else {
                    println!("Game saved!");
                }
            }
            Err(e) => {
                println!("Failed to serialize save data: {}", e);
            }
        }
    }
}
