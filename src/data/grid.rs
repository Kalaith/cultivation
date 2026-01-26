use serde::{Deserialize, Serialize};
use crate::data::elements::Element;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub terrain: String, // "Grass", "Dirt", "Water", etc.
    // Base strength of elements on this tile (from terrain + nearby objects)
    pub element_strength: HashMap<Element, f32>,
    pub feng_shui_score: f32,
    pub dominant_element: Element,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Self {
        let mut strengths = HashMap::new();
        strengths.insert(Element::Wood, 1.0); // Default grass
        
        Self {
            x,
            y,
            terrain: "Grass".to_string(),
            element_strength: strengths,
            feng_shui_score: 0.0,
            dominant_element: Element::Wood,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                tiles.push(Tile::new(x, y));
            }
        }
        
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            let index = (y * self.width + x) as usize;
            self.tiles.get(index)
        }
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            let index = (y * self.width + x) as usize;
            self.tiles.get_mut(index)
        }
    }
}
