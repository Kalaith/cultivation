use serde::{Deserialize, Serialize};

/// All resources the player can accumulate.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Resources {
    pub spirit_stones: u32,
    pub herbs: u32,
    pub relics: u32,
    pub influence: u32,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            spirit_stones: 100,
            herbs: 0,
            relics: 0,
            influence: 0,
        }
    }
}
