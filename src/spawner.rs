use super::{
    rect::Rect, AreaOfEffect, BlocksTile, CombatStats, Consumable, DefenseBonus, Equipable,
    EquipmentSlot, InflictsDamage, Item, MeleeDamageBonus, Monster, Name, Player, Position,
    ProvidesHealing, RandomTable, Ranged, Renderable, SerializeMe, SpawnType, Viewshed,
};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

const MAX_SPAWNS: i32 = 9; //Per room

//ENTITIES-----------
pub fn spawn_player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            is_dirty: true,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn spawn_kobold(ecs: &mut World, x: i32, y: i32) -> Entity {
    spawn_monster(ecs, x, y, rltk::to_cp437('k'), "Kobold")
}

pub fn spawn_goblin(ecs: &mut World, x: i32, y: i32) -> Entity {
    spawn_monster(ecs, x, y, rltk::to_cp437('g'), "Goblin")
}

pub fn spawn_monster(
    ecs: &mut World,
    x: i32,
    y: i32,
    glyph: rltk::FontCharType,
    name: &str,
) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            is_dirty: true,
        })
        .with(Monster {})
        .with(BlocksTile {})
        .with(Name {
            name: name.to_string(),
        })
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

//ITEMS---------------
pub fn spawn_health_pot(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Item {})
        .with(ProvidesHealing { heal_amount: 8 })
        .with(Consumable {})
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Renderable {
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn spawn_magic_missile_scroll(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Magic Missle Scroll".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn spawn_fireball_scroll(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Fireball Scroll".to_string(),
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn spawn_simple_dagger(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Dagger".to_string(),
        })
        .with(Item {})
        .with(Equipable {
            slot: EquipmentSlot::PrimaryHand,
        })
        .with(MeleeDamageBonus { bonus: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn spawn_simple_shield(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: 248,
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name {
            name: "Shield".to_string(),
        })
        .with(Item {})
        .with(Equipable {
            slot: EquipmentSlot::OffHand,
        })
        .with(DefenseBonus { bonus: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

//ROOM POPULATION-----
pub fn populate_room(ecs: &mut World, room: &Rect, map_depth: i32) {
    let spawn_table = create_room_table(map_depth);
    let mut spawn_points: HashMap<(i32, i32), Option<SpawnType>> = HashMap::new();

    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_spawns = rng.roll_dice(1, MAX_SPAWNS + 3) + (map_depth - 1) - 3;

    for _ in 0..num_spawns {
        let mut tries = 20;
        while tries > 0 {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
            if spawn_points.get(&(x, y)).is_some() {
                tries += 1;
            } else {
                spawn_points.insert((x, y), spawn_table.roll(&mut rng));
                break;
            }
        }
    }

    std::mem::drop(rng);
    for spawn in spawn_points.iter() {
        let x = spawn.0 .0;
        let y = spawn.0 .1;

        match spawn.1 {
            Some(SpawnType::Goblin) => {
                spawn_goblin(ecs, x, y);
            }
            Some(SpawnType::Kobold) => {
                spawn_kobold(ecs, x, y);
            }
            Some(SpawnType::HealthPotion) => {
                spawn_health_pot(ecs, x, y);
            }
            Some(SpawnType::FireballScroll) => {
                spawn_fireball_scroll(ecs, x, y);
            }
            Some(SpawnType::MagicMissileScroll) => {
                spawn_magic_missile_scroll(ecs, x, y);
            }
            Some(SpawnType::SimpleDagger) => {
                spawn_simple_dagger(ecs, x, y);
            }
            Some(SpawnType::SimpleShield) => {
                spawn_simple_shield(ecs, x, y);
            }
            _ => {}
        };
    }
}

fn create_room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .insert(SpawnType::Goblin, 9 + map_depth)
        .insert(SpawnType::Kobold, 3)
        .insert(SpawnType::HealthPotion, 7)
        .insert(SpawnType::FireballScroll, 2 + map_depth)
        .insert(SpawnType::MagicMissileScroll, 4 + map_depth)
        .insert(SpawnType::SimpleDagger, 3)
        .insert(SpawnType::SimpleShield, 3)
}
