use serde::{Deserialize, Serialize};

/// Records a disciple who has died.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeceasedDisciple {
    pub name: String,
    pub realm_at_death: String,
    pub cause_of_death: String,
    /// Game tick when the disciple died.
    pub tick_of_death: u64,
}

impl DeceasedDisciple {
    pub fn new(name: String, realm: String, cause: String, tick: u64) -> Self {
        Self {
            name,
            realm_at_death: realm,
            cause_of_death: cause,
            tick_of_death: tick,
        }
    }
}
