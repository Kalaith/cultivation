use crate::data::disciples::Attributes;
use crate::data::elements::Element;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CultivationLaw {
    pub id: String,
    pub name: String,
    pub description: String,
    pub element: Element,
    pub stat_growth_modifiers: Attributes,
    pub breakthrough_modifier: f32, // Additive to base chance
}
