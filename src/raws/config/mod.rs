mod config_master;
mod config_structs;

use config_master::ConfigMaster;
use std::sync::Mutex;

#[rustfmt::skip]
macro_rules! config_path {
    () => ("../../../prefabs/config.ron")
}

lazy_static::lazy_static! {
    pub static ref CONFIGS: Mutex<ConfigMaster> = Mutex::new(ConfigMaster::new());
}

rltk::embedded_resource!(CONFIG_RAW, config_path!());

pub fn load() -> Result<(), ()> {
    rltk::link_resource!(CONFIG_RAW, config_path!());
    let config_as_bytes = rltk::embedding::EMBED
        .lock()
        .get_resource(config_path!().to_string())
        .unwrap();

    if let Ok(configs) = ron::de::from_bytes(config_as_bytes) {
        CONFIGS.lock().unwrap().load_config(configs);
        return Ok(());
    }

    Err(())
}
