use super::config_structs::{AudioConfigs, KeyBinds, VisualConfigs};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub keys: KeyBinds,
    pub visual: VisualConfigs,
    pub audio: AudioConfigs,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            keys: KeyBinds::new(),
            visual: VisualConfigs::new(),
            audio: AudioConfigs::new(),
        }
    }
    pub fn load_config(&mut self, desired_config: Self) {
        *self = desired_config;
    }
}
