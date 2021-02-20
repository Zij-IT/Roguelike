use crate::map_builder::map::Map;
use rltk::ColorPair;
use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    prelude::*,
    saveload::{ConvertSaveload, Marker},
};
use specs_derive::*;

//Components are organized by who they are **TYPICALLY** assigned to.
//(N)PC
//Items
//Particles

//Shared components
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Render {
    pub glyph: rltk::FontCharType,
    pub colors: ColorPair,
    pub render_order: i32,
}

pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: Map,
}

//(N)PC Components
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<'_, Self>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = Self {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub is_dirty: bool,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

//Item components
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Consumable {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Neck,
    Torso,
    Body,
    PrimaryHand,
    OffHand,
    Feet,
    Finger,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Equipable {
    pub slot: EquipmentSlot,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleeDamageBonus {
    pub bonus: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenseBonus {
    pub bonus: i32,
}

//Particles
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
}
