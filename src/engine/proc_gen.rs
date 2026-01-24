use crate::data::disciples::{Attributes, CultivationRealm, Disciple, FateTrait, Talent};
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

    Disciple {
        name,
        realm: CultivationRealm::Mortal,
        talent,
        attributes,
        loyalty: 50,
        fate_traits,
        exp: 0,
        exp_to_next_level: 100,
    }
}
