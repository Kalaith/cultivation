use crate::data::buildings::BuildingType;

#[derive(Debug)]
pub enum Action {
    UpgradeBuilding(BuildingType),
    RecruitDisciple,
    PromoteDisciple(usize), // Disciple Index
    StartMission(String, Vec<usize>), // Mission Description, Disciple Indices
    ClaimRewards(crate::data::missions::MissionOutcome),
    AssignLaw(usize, String), // Disciple Index, Law ID
    CraftItem(String), // Recipe ID
    UseItem(String, usize), // Item ID, Disciple Index
    ResearchTech(String), // Tech ID
    RepairBuilding(u64), // Building ID
    ConstructBuilding(BuildingType, i32, i32), // Type, X, Y
    SaveGame,
    LoadGame,
    StartNewGame(String), // Sect Name
    // Herb system actions
    PlantHerb(u64, usize, String), // Building ID, Plot Index, Herb ID
    AssignDiscipleToBuilding(u64, Option<u64>), // Building ID, Disciple ID (None to unassign)
    ProcessDryingPavilion(u64, String), // Building ID, Herb ID to dry
    SetGreenhouseInfusion(u64, Option<crate::data::elements::Element>), // Building ID, Element
}
