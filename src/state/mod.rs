use crate::engine::actions::Action;

pub mod library;
pub mod main_menu;
pub mod mission_assignment;
pub mod mission_resolution;
pub mod roster;
pub mod sect_base;
pub mod sect_creation;
pub mod world_map;

use self::{
    library::LibraryState, main_menu::MainMenuState,
    mission_assignment::MissionAssignmentState,
    mission_resolution::MissionResolutionState, roster::DiscipleRosterState,
    sect_base::SectBaseState, sect_creation::SectCreationState, world_map::WorldMapState,
};

pub enum GameState {
    MainMenu(MainMenuState),
    SectBase(SectBaseState),
    DiscipleRoster(DiscipleRosterState),
    WorldMap(WorldMapState),
    MissionResolution(MissionResolutionState),
    Library(LibraryState),
    MissionAssignment(MissionAssignmentState),
    SectCreation(SectCreationState),
}

pub enum StateTransition {
    ToMainMenu,
    ToSectBase,
    ToDiscipleRoster,
    ToWorldMap,
    ToMissionAssignment(String),
    ToMissionResolution,
    ToLibrary,
    ToSectCreation,
}
//...

pub struct UpdateResult {
    pub transition: Option<StateTransition>,
    pub action: Option<Action>,
}

impl UpdateResult {
    pub fn new() -> Self {
        Self {
            transition: None,
            action: None,
        }
    }

    pub fn with_transition(mut self, t: StateTransition) -> Self {
        self.transition = Some(t);
        self
    }

    pub fn with_action(mut self, a: Action) -> Self {
        self.action = Some(a);
        self
    }
}