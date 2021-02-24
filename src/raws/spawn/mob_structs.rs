use super::item_structs::RawRender;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Mob {
    pub name: String,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub render: RawRender,
    pub stats: RawStats,
}

#[derive(Deserialize, Debug)]
pub struct RawStats {
    pub max_hp: i32,
    pub defense: i32,
    pub power: i32,
}
