mod item_structs;
mod mob_structs;
mod raw_master;

use serde::Deserialize;
use std::sync::Mutex;

pub use raw_master::{RawMaster, SpawnType};

//In rust you are not able to use const string slices inside of macros, and because I don't want to
//type the same thing multiple times and have an error result out of that, I am using a macro as a
//constant. If that is too dirty for you, I suggest you avert your eyes.
#[rustfmt::skip]
macro_rules! raw_spawns_path {
    () => ("../../prefabs/spawns.ron")
}

lazy_static::lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

rltk::embedded_resource!(RAW_SPAWNS, raw_spawns_path!());

#[derive(Deserialize, Debug)]
pub struct Raws {
    pub mobs: Vec<mob_structs::Mob>,
    pub items: Vec<item_structs::Item>,
}

pub fn load_raws() {
    rltk::link_resource!(RAW_SPAWNS, raw_spawns_path!());
    let spawn_raw = rltk::embedding::EMBED
        .lock()
        .get_resource(raw_spawns_path!().to_string())
        .unwrap();
    let decoder: Raws = ron::de::from_bytes(spawn_raw).expect("Unable to parse RON");
    RAWS.lock().unwrap().load(decoder);
}
