use serde::{Deserialize, Serialize};
use crate::data::disciples::Disciple;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TribulationType {
    GoldenCore,
    NascentSoul,
    SpiritSevering,
    Ascension,
    DaoAncestor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TribulationConfig {
    pub name: String,
    pub description: String,
    pub total_waves: u32,
    pub base_damage: u32,
    pub damage_scale_per_wave: f32,
    pub lightning_type: String, // "Pure", "Soul", etc.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TribulationState {
    pub tribulation_type: TribulationType,
    pub config: TribulationConfig,
    pub current_wave: u32,
    pub disciple_hp: i32,
    pub disciple_max_hp: i32,
    pub is_finished: bool,
    pub survived: bool,
    pub log: Vec<String>,
}

impl TribulationState {
    pub fn new(t_type: TribulationType, disciple: &Disciple) -> Self {
        let config = match t_type {
            TribulationType::GoldenCore => TribulationConfig {
                name: "Golden Core Tribulation".to_string(),
                description: "The separation of mortal and cultivator. Azoth Thunder strikes!".to_string(),
                total_waves: 3,
                base_damage: 20,
                damage_scale_per_wave: 1.5,
                lightning_type: "Pure".to_string(),
            },
            TribulationType::NascentSoul => TribulationConfig {
                name: "Nascent Soul Tribulation".to_string(),
                description: "Yin-Yang Fire burns the spirit.".to_string(),
                total_waves: 9,
                base_damage: 50,
                damage_scale_per_wave: 1.2,
                lightning_type: "Soul".to_string(),
            },
            _ => TribulationConfig {
                name: "Unknown Tribulation".to_string(),
                description: "A trial beyond understanding.".to_string(),
                total_waves: 1,
                base_damage: 9999,
                damage_scale_per_wave: 1.0,
                lightning_type: "Void".to_string(),
            },
        };

        // Initialize HP based on Body/Spirit
        let max_hp = ((disciple.attributes.body + disciple.attributes.spirit) * 10) as i32;

        Self {
            tribulation_type: t_type,
            config,
            current_wave: 0,
            disciple_hp: max_hp,
            disciple_max_hp: max_hp,
            is_finished: false,
            survived: false,
            log: Vec::new(),
        }
    }

    pub fn process_wave(&mut self, disciple: &Disciple) {
        if self.is_finished { return; }

        self.current_wave += 1;
        self.log.push(format!("Wave {}/{} begins!", self.current_wave, self.config.total_waves));

        // fast calculation for damage
        let raw_damage = (self.config.base_damage as f32 * self.config.damage_scale_per_wave.powi(self.current_wave as i32 - 1)) as i32;
        
        // Mitigation
        let defense = (disciple.attributes.body * 2) as i32; 
        let mitigated_damage = (raw_damage - defense).max(0);
        
        self.log.push(format!("Lightning strikes for {} damage! (Blocked {})", raw_damage, defense));
        
        if mitigated_damage > 0 {
            self.disciple_hp -= mitigated_damage;
            self.log.push(format!("{} takes {} damage! HP: {}/{}", disciple.name, mitigated_damage, self.disciple_hp, self.disciple_max_hp));
        } else {
             self.log.push(format!("{} completely resists the lightning!", disciple.name));
        }

        if self.disciple_hp <= 0 {
            self.is_finished = true;
            self.survived = false;
            self.log.push(format!("{} has fallen to the tribulation...", disciple.name));
        } else if self.current_wave >= self.config.total_waves {
            self.is_finished = true;
            self.survived = true;
            self.log.push(format!("{} has successfully overcome the tribulation!", disciple.name));
        }
    }
}
