use crate::data::buildings::Building;
use crate::data::elements::{Element, InteractionResult};
use crate::data::grid::Grid;

pub fn update_feng_shui_map(grid: &mut Grid, buildings: &mut Vec<Building>) {
    // 1. Reset grid elements (keep terrain base if implemented, but for now reset to just terrain)
    // For MVP, assuming terrain is just generic Wood/Earth baseline.
    // We should iterate tiles and reset `element_strength` to baseline.

    let width = grid.width;
    let height = grid.height;

    for y in 0..height {
        for x in 0..width {
            if let Some(tile) = grid.get_tile_mut(x, y) {
                tile.element_strength.clear();
                // Baseline environment (e.g. Grass = Wood)
                // In future, map_nodes.json could define this per tile?
                // For now, hardcode "Grass" -> Wood 1.0
                tile.element_strength.insert(Element::Wood, 1.0);
                tile.dominant_element = Element::Wood;
                tile.feng_shui_score = 0.0;
            }
        }
    }

    // 2. Propagate Qi from buildings
    for building in buildings.iter() {
        if building.element == Element::None {
            continue;
        }

        // Simple radius propagation
        let radius = 2; // Range of influence
        let center_x = building.x;
        let center_y = building.y;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let tx = center_x + dx;
                let ty = center_y + dy;

                if let Some(tile) = grid.get_tile_mut(tx, ty) {
                    let dist = (dx.abs() + dy.abs()) as f32; // Manhattan distance
                    let decay = 1.0 - (dist / (radius as f32 + 1.0));

                    if decay > 0.0 {
                        let strength =
                            *tile.element_strength.get(&building.element).unwrap_or(&0.0);
                        // Basic Propagation
                        tile.element_strength
                            .insert(building.element.clone(), strength + (1.0 * decay));

                        // Extended Logic: Elemental Suppression (Destruction Cycle)
                        // This building element suppresses another element.
                        // Reduce the strength of that suppressed element on this tile.
                        let suppressed_element = building.element.suppresses();
                        if suppressed_element != Element::None {
                            let target_strength = *tile
                                .element_strength
                                .get(&suppressed_element)
                                .unwrap_or(&0.0);
                            if target_strength > 0.0 {
                                let reduction = 0.5 * decay; // Example dampening factor
                                let new_strength = (target_strength - reduction).max(0.0);
                                tile.element_strength
                                    .insert(suppressed_element, new_strength);
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Recalculate Dominant Element & Feng Shui Score
    for y in 0..height {
        for x in 0..width {
            if let Some(tile) = grid.get_tile_mut(x, y) {
                // Find dominant
                let mut max_str = 0.0;
                let mut dom = Element::None;
                for (elem, str_val) in &tile.element_strength {
                    if *str_val > max_str {
                        max_str = *str_val;
                        dom = elem.clone();
                    }
                }
                tile.dominant_element = dom.clone();
            }
        }
    }

    // Second pass for score on valid buildings
    for building in buildings.iter_mut() {
        if let Some(tile) = grid.get_tile_mut(building.x, building.y) {
            let env_element = &tile.dominant_element;
            let b_element = &building.element;

            let interaction = env_element.get_interaction(b_element);
            let score = match interaction {
                InteractionResult::Feeds => 10.0, // Exceptional (Wood feeds Fire Bed)
                InteractionResult::Suppresses => -10.0, // Terrible (Water suppresses Fire Bed)
                InteractionResult::Neutral => {
                    if env_element == b_element {
                        5.0 // Good (Fire in Fire)
                    } else {
                        0.0
                    }
                }
            };

            tile.feng_shui_score = score;
            building.feng_shui_score = score; // Cache score on building
        }
    }
}
