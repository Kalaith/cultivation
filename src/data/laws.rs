use serde::{Deserialize, Serialize};
use crate::data::elements::Element;
use crate::data::disciples::Attributes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CultivationLaw {
    pub id: String,
    pub name: String,
    pub description: String,
    pub element: Element,
    pub stat_growth_modifiers: Attributes,
    pub breakthrough_modifier: f32, // Additive to base chance
}
