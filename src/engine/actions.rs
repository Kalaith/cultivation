use crate::data::buildings::BuildingType;
use crate::data::missions::MissionOutcome;

#[derive(Debug)]
pub enum Action {
    UpgradeBuilding(BuildingType),
    RecruitDisciple,
    StartMission(String, Vec<usize>),
    ClaimRewards(MissionOutcome),
    SaveGame,
    LoadGame,
}
