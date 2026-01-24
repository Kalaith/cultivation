use crate::engine::actions::Action;

pub mod library;
pub mod main_menu;
pub mod mission_resolution;
pub mod roster;
pub mod sect_base;
pub mod world_map;

pub mod mission_assignment;
// ...
use self::{
    library::LibraryState, main_menu::MainMenuState,
    mission_assignment::MissionAssignmentState,
    mission_resolution::MissionResolutionState, roster::DiscipleRosterState,
    sect_base::SectBaseState, world_map::WorldMapState,
};

pub enum GameState {
    MainMenu(MainMenuState),
    SectBase(SectBaseState),
    DiscipleRoster(DiscipleRosterState),
    WorldMap(WorldMapState),
    MissionResolution(MissionResolutionState),
    Library(LibraryState),
    MissionAssignment(MissionAssignmentState),
}

pub enum StateTransition {
    ToMainMenu,
    ToSectBase,
    ToDiscipleRoster,
    ToWorldMap,
    ToMissionAssignment(String),
    ToMissionResolution, // This will likely carry data
    ToLibrary,
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