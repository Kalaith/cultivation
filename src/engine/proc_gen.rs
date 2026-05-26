use crate::data::bloodlines::{BloodlineRarity, DiscipleBloodline};
use crate::data::disciples::{Attributes, Disciple, DiscipleNeeds, DiscipleRank, Talent};
use crate::data::loader::GameData;
use macroquad_toolkit::rng as game_rng;

const NAMES: &[&str] = &["Chen", "Wang", "Zhang", "Liu", "Zhao", "Jia", "Shen"];

pub fn generate_disciple(game_data: &GameData) -> Disciple {
    let name = game_rng::choose(NAMES).unwrap_or(&"Unnamed").to_string();

    let talent = match game_rng::gen_range(0, 100) {
        0..=49 => Talent::Low,     // 50%
        50..=79 => Talent::Medium, // 30%
        80..=94 => Talent::High,   // 15%
        95..=98 => Talent::Genius, // 4%
        _ => Talent::HeavenSent,   // 1%
    };

    let attributes = Attributes {
        body: game_rng::gen_range(5, 16),
        mind: game_rng::gen_range(5, 16),
        spirit: game_rng::gen_range(5, 16),
    };

    let num_traits = game_rng::gen_range(1usize, 3).min(game_data.fate_traits.len());
    let mut fate_trait_indices = (0..game_data.fate_traits.len()).collect::<Vec<_>>();
    game_rng::shuffle(&mut fate_trait_indices);
    let fate_traits = fate_trait_indices
        .into_iter()
        .take(num_traits)
        .map(|index| game_data.fate_traits[index].clone())
        .collect();

    // Generate bloodline based on rarity chances
    let bloodline = generate_bloodline(game_data);

    Disciple {
        id: crate::engine::random::next_u64(),
        name,
        rank: DiscipleRank::Outer,
        realm: "Mortal".to_string(),
        sub_stage: 0,
        talent,
        attributes,
        loyalty: 50,
        fate_traits,
        exp: 0,
        exp_to_next_level: 100,
        qi: 0,
        max_qi: 0,
        law_id: None,
        bloodline,
        needs: DiscipleNeeds::from_tuning(&game_data.ai_scheduler),
        injury: None,
        breakthrough_readiness: 0.0,
        equipment: std::collections::HashMap::new(),
        breakthrough_bottleneck: None,
    }
}

/// Generate a random bloodline for a disciple based on rarity chances
fn generate_bloodline(game_data: &GameData) -> DiscipleBloodline {
    // Roll for whether disciple has a bloodline at all
    let bloodline_roll = game_rng::gen_range(0, 100);

    // 60% no bloodline, 25% Mortal, 10% Spirit, 4% Ancient, 0.9% Primordial, 0.1% Mythic
    let target_rarity = match bloodline_roll {
        0..=59 => return DiscipleBloodline::none(), // No bloodline
        60..=84 => BloodlineRarity::Mortal,         // 25%
        85..=94 => BloodlineRarity::Spirit,         // 10%
        95..=98 => BloodlineRarity::Ancient,        // 4%
        99 => {
            // 1% chance, split between Primordial (0.9%) and Mythic (0.1%)
            if game_rng::gen_range(0, 10) == 0 {
                BloodlineRarity::Mythic
            } else {
                BloodlineRarity::Primordial
            }
        }
        _ => return DiscipleBloodline::none(),
    };

    // Filter bloodlines by target rarity
    let matching_bloodlines: Vec<_> = game_data
        .bloodlines
        .values()
        .filter(|b| b.rarity == target_rarity)
        .collect();

    if let Some(bloodline) = game_rng::choose(&matching_bloodlines) {
        DiscipleBloodline::new(bloodline.id.clone())
    } else {
        DiscipleBloodline::none()
    }
}
