use crate::data::bloodlines::{BloodlineRarity, DiscipleBloodline};
use crate::data::disciples::{Attributes, Disciple, DiscipleNeeds, DiscipleRank, Talent};
use crate::data::loader::GameData;
use rand::prelude::*;

const NAMES: &[&str] = &["Chen", "Wang", "Zhang", "Liu", "Zhao", "Jia", "Shen"];

pub fn generate_disciple(game_data: &GameData) -> Disciple {
    let mut rng = thread_rng();

    let name = NAMES.choose(&mut rng).unwrap_or(&"Unnamed").to_string();

    let talent = match rng.gen_range(0..=99) {
        0..=49 => Talent::Low,      // 50%
        50..=79 => Talent::Medium,   // 30%
        80..=94 => Talent::High,     // 15%
        95..=98 => Talent::Genius,   // 4%
        _ => Talent::HeavenSent, // 1%
    };

    let attributes = Attributes {
        body: rng.gen_range(5..=15),
        mind: rng.gen_range(5..=15),
        spirit: rng.gen_range(5..=15),
    };

    let num_traits = rng.gen_range(1..=2);
    let fate_traits = game_data.fate_traits
        .choose_multiple(&mut rng, num_traits)
        .cloned()
        .collect();

    // Generate bloodline based on rarity chances
    let bloodline = generate_bloodline(game_data, &mut rng);

    Disciple {
        id: rng.gen(), // Generate random unique ID
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
    }
}

/// Generate a random bloodline for a disciple based on rarity chances
fn generate_bloodline(game_data: &GameData, rng: &mut ThreadRng) -> DiscipleBloodline {
    // Roll for whether disciple has a bloodline at all
    let bloodline_roll = rng.gen_range(0..=99);

    // 60% no bloodline, 25% Mortal, 10% Spirit, 4% Ancient, 0.9% Primordial, 0.1% Mythic
    let target_rarity = match bloodline_roll {
        0..=59 => return DiscipleBloodline::none(),  // No bloodline
        60..=84 => BloodlineRarity::Mortal,          // 25%
        85..=94 => BloodlineRarity::Spirit,          // 10%
        95..=98 => BloodlineRarity::Ancient,         // 4%
        99 => {
            // 1% chance, split between Primordial (0.9%) and Mythic (0.1%)
            if rng.gen_range(0..10) == 0 {
                BloodlineRarity::Mythic
            } else {
                BloodlineRarity::Primordial
            }
        }
        _ => return DiscipleBloodline::none(),
    };

    // Filter bloodlines by target rarity
    let matching_bloodlines: Vec<_> = game_data.bloodlines.values()
        .filter(|b| b.rarity == target_rarity)
        .collect();

    if let Some(bloodline) = matching_bloodlines.choose(rng) {
        DiscipleBloodline::new(bloodline.id.clone())
    } else {
        DiscipleBloodline::none()
    }
}
