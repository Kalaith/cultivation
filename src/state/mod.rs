use crate::engine::actions::Action;

pub mod faction_screen;
pub mod intro;
pub mod library;
pub mod main_menu;
pub mod mission_assignment;
pub mod mission_resolution;
pub mod roster;
pub mod sect_base;
pub mod sect_creation;
pub mod trade_screen;
pub mod tribulation;
pub mod world_map;

use self::{
    faction_screen::FactionScreenState, intro::IntroState, library::LibraryState,
    main_menu::MainMenuState, mission_assignment::MissionAssignmentState,
    mission_resolution::MissionResolutionState, roster::DiscipleRosterState,
    sect_base::SectBaseState, sect_creation::SectCreationState, trade_screen::TradeScreenState,
    tribulation::TribulationEncounterState, world_map::WorldMapState,
};

pub enum GameState {
    MainMenu(MainMenuState),
    Intro(IntroState),
    SectBase(SectBaseState),
    DiscipleRoster(DiscipleRosterState),
    WorldMap(WorldMapState),
    MissionResolution(MissionResolutionState),
    Library(LibraryState),
    MissionAssignment(MissionAssignmentState),
    SectCreation(SectCreationState),
    Tribulation(TribulationEncounterState),
    FactionScreen(FactionScreenState),
    TradeScreen(TradeScreenState),
}

pub enum StateTransition {
    ToMainMenu,
    ToIntro,
    ToSectBase,
    ToDiscipleRoster,
    ToWorldMap,
    ToMissionAssignment(String),
    ToMissionResolution,
    ToLibrary,
    ToSectCreation,
    ToTribulation(crate::engine::tribulation::TribulationState, usize),
    ToFactionScreen,
    ToTradeScreen,
}
//...

pub struct TutorialState {
    pub active: bool,
    pub step: usize,
    pub hidden: bool,
}

impl TutorialState {
    pub fn new() -> Self {
        Self {
            active: true,
            step: 0,
            hidden: false,
        }
    }
}

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
