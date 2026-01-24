use crate::data::buildings::BuildingType;

#[derive(Debug)]
pub enum Action {
    UpgradeBuilding(BuildingType),
    RecruitDisciple,
}
