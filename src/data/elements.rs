use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Element {
    Metal,
    Wood,
    Water,
    Fire,
    Earth,
    #[default]
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InteractionResult {
    Feeds,
    Suppresses,
    Neutral,
}

impl Element {
    /// Returns the element that this element feeds (Generation Cycle)
    pub fn feeds(&self) -> Element {
        match self {
            Element::Metal => Element::Water,
            Element::Water => Element::Wood,
            Element::Wood => Element::Fire,
            Element::Fire => Element::Earth,
            Element::Earth => Element::Metal,
            Element::None => Element::None,
        }
    }

    /// Returns the element that this element suppresses (Destruction Cycle)
    pub fn suppresses(&self) -> Element {
        match self {
            Element::Metal => Element::Wood,
            Element::Wood => Element::Earth,
            Element::Earth => Element::Water,
            Element::Water => Element::Fire,
            Element::Fire => Element::Metal,
            Element::None => Element::None,
        }
    }

    /// Determines the interaction when 'self' acts upon 'other'
    /// e.g. Water.get_interaction(Fire) -> Suppresses
    /// e.g. Wood.get_interaction(Fire) -> Feeds
    pub fn get_interaction(&self, other: &Element) -> InteractionResult {
        if self.feeds() == *other {
            InteractionResult::Feeds
        } else if self.suppresses() == *other {
            InteractionResult::Suppresses
        } else {
            InteractionResult::Neutral
        }
    }

    /// Returns the color associated with this element for visual rendering
    pub fn color(&self) -> Color {
        match self {
            Element::Metal => Color::new(0.85, 0.85, 0.9, 1.0),   // Silver
            Element::Wood => Color::new(0.2, 0.7, 0.3, 1.0),      // Forest Green
            Element::Water => Color::new(0.2, 0.4, 0.85, 1.0),    // Blue
            Element::Fire => Color::new(0.9, 0.3, 0.2, 1.0),      // Red
            Element::Earth => Color::new(0.75, 0.6, 0.3, 1.0),    // Yellow/Brown
            Element::None => Color::new(0.4, 0.4, 0.4, 1.0),      // Gray
        }
    }
}
