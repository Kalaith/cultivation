use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CultivationRealm {
    Mortal,
    QiRefinement,
    FoundationEstablishment,
    CoreFormation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Talent {
    Low,
    Medium,
    High,
    Genius,
    HeavenSent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    pub body: u32,
    pub mind: u32,
    pub spirit: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FateTrait {
    pub name: String,
    pub description: String,
    /// Modifier to breakthrough success chance (e.g., 0.2 = +20%)
    #[serde(default)]
    pub breakthrough_modifier: f32,
    /// Modifier to injury/death chance (e.g., 0.15 = +15% injury chance)
    #[serde(default)]
    pub injury_modifier: f32,
    /// Modifier to combat mission success (MonsterSuppression, RuinDelve)
    #[serde(default)]
    pub combat_modifier: f32,
    /// Modifier to diplomacy mission success
    #[serde(default)]
    pub diplomacy_modifier: f32,
    /// Modifier to exploration/resource mission success
    #[serde(default)]
    pub exploration_modifier: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Disciple {
    pub name: String,
    pub realm: CultivationRealm,
    pub talent: Talent,
    pub attributes: Attributes,
    pub loyalty: u32,
    pub fate_traits: Vec<FateTrait>,
    pub exp: u32,
    pub exp_to_next_level: u32,
}
