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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FateTrait {
    pub name: String,
    pub description: String,
    // We can add effect modifiers here later
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
