use crate::data::buildings::BuildingType;

#[derive(Debug)]
pub enum Action {
    UpgradeBuilding(BuildingType),
    RecruitDisciple,
    StartMission(String, Vec<usize>),
    SaveGame,
}
