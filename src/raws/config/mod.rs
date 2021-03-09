mod config_structs;
use config_structs::{AudioConfigs, KeyBinds, VisualConfigs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub keys: KeyBinds,
    pub visual: VisualConfigs,
    pub audio: AudioConfigs,
}

impl Config {
    pub fn load_config(&mut self, desired_config: Self) {
        *self = desired_config;
    }
}

pub fn load() -> Result<Config, Config> {
    let config = include_bytes!("../../../prefabs/config.ron");

    match ron::de::from_bytes(config) {
        Ok(config) => Ok(config),
        Err(_) => Err(Config::default()),
    }
}

pub fn save(current_configs: &Config) -> ron::Result<()> {
    let writer = std::fs::File::create("./prefabs/config.ron").unwrap();

    let pretty = ron::ser::PrettyConfig::new();

    ron::ser::to_writer_pretty(writer, current_configs, pretty)
}
