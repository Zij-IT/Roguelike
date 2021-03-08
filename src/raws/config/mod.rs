mod config;
mod config_structs;

pub use config::Config;

pub fn load() -> Result<Config, Config> {
    let config = include_bytes!("../../../prefabs/config.ron");

    return match ron::de::from_bytes(config) {
        Ok(config) => Ok(config),
        Err(_) => Err(Config::new()),
    };
}

pub fn save(current_configs: &Config) -> ron::Result<()> {
    let writer = std::fs::File::create("./prefabs/config.ron").unwrap();

    let pretty = ron::ser::PrettyConfig::new();

    ron::ser::to_writer_pretty(writer, current_configs, pretty)
}
