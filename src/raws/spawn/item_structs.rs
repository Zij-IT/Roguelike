use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub render: RawRender,
    pub consumable: Option<RawConsumable>,
    pub weapon: Option<RawWeapon>,
    pub shield: Option<RawShield>,
}

#[derive(Deserialize, Debug)]
pub struct RawRender {
    pub glyph: u16,
    pub color: (u8, u8, u8),
    pub order: i32,
}

#[derive(Deserialize, Debug)]
pub struct RawConsumable {
    pub effects: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct RawShield {
    pub defense_bonus: i32,
}

#[derive(Deserialize, Debug)]
pub struct RawWeapon {
    pub damage_bonus: i32,
}
