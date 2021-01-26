use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub is_dirty: bool,
}

#[derive(Component)]
pub struct Monster {}

#[derive(Component)]
pub struct Name {
    pub name: String
}

#[derive(Component)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: usize,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component)]
pub struct WantsToMelee {
    pub target: Entity
}

#[derive(Component)]
pub struct SufferDamage {
    pub amount: Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim : Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        }
        else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component)]
pub struct Item {}
