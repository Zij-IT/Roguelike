use super::config_structs::{KeyBinds, VisualEffects};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigMaster {
    pub keys: KeyBinds,
    pub visuals: VisualEffects,
}

impl ConfigMaster {
    pub const fn new() -> Self {
        Self {
            keys: KeyBinds::new(),
            visuals: VisualEffects::new(),
        }
    }
    pub fn load_config(&mut self, desired_config: Self) {
        *self = desired_config;
    }
}
