use super::{
    AreaOfEffect, BlocksTile, CombatStats, Consumable, DefenseBonus, Equipable, EquipmentSlot,
    InflictsDamage, Item, MeleeDamageBonus, Monster, Name, Player, Position, ProvidesHealing,
    RandomTable, Ranged, Renderable, SerializeMe, Viewshed,
};
use crate::{constants::colors, rect::Rect, Map, TileType};
use rltk::{ColorPair, RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;

const MAX_MONSTERS: i32 = 4;

//ROOM POPULATION-----
pub fn populate_room(ecs: &mut World, room: &Rect) {
    let mut possible_spawns = Vec::new();
    let map = ecs.fetch::<Map>();
    let map_depth = map.depth;
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == TileType::Floor {
                possible_spawns.push((x, y));
            }
        }
    }
    std::mem::drop(map);
    spawn_region(ecs, &possible_spawns, map_depth);
}

pub fn spawn_region(ecs: &mut World, area: &[(i32, i32)], map_depth: i32) {
    let spawn_table = create_room_table(map_depth);
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let mut spawn_points = HashMap::new();
    let mut areas = Vec::from(area);

    let num_spawns = i32::min(
        areas.len() as i32,
        rng.roll_dice(1, MAX_MONSTERS + 3) + map_depth - 1 - 3,
    );

    for _ in 0..num_spawns {
        let array_index = if areas.len() == 1 {
            0usize
        } else {
            (rng.roll_dice(1, areas.len() as i32) - 1) as usize
        };
        let map_point = areas[array_index];
        if let Some(spawn) = spawn_table.roll(&mut rng) {
            spawn_points.insert(map_point, spawn);
        }
        areas.remove(array_index);
    }

    std::mem::drop(rng);
    for spawn in spawn_points.iter() {
        spawn_named_entity(ecs, &spawn);
    }
}

fn spawn_named_entity(ecs: &mut World, ((x, y), name): &(&(i32, i32), &String)) {
    match name.as_ref() {
        "Goblin" => {
            spawn_goblin(ecs, *x, *y);
        }
        "Kobold" => {
            spawn_kobold(ecs, *x, *y);
        }
        "HealthPotion" => {
            spawn_health_pot(ecs, *x, *y);
        }
        "FireballScroll" => {
            spawn_fireball_scroll(ecs, *x, *y);
        }
        "MagicMissileScroll" => {
            spawn_magic_missile_scroll(ecs, *x, *y);
        }
        "SimpleDagger" => {
            spawn_simple_dagger(ecs, *x, *y);
        }
        "SimpleShield" => {
            spawn_simple_shield(ecs, *x, *y);
        }
        _ => {}
    };
}

fn create_room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .insert("Goblin", 9 + map_depth)
        .insert("Kobold", 3)
        .insert("HealthPotion", 7)
        .insert("FireballScroll", 2 + map_depth)
        .insert("MagicMissileScroll", 4 + map_depth)
        .insert("SimpleDagger", 3)
        .insert("SimpleShield", 3)
}

//ENTITIES-----------
pub fn spawn_player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            colors: ColorPair::new(RGB::named(rltk::YELLOW), RGB::from(colors::BACKGROUND)),
            render_order: 1,
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
            colors: ColorPair::new(RGB::named(rltk::RED), RGB::from(colors::BACKGROUND)),
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
            colors: ColorPair::new(RGB::named(rltk::MAGENTA), RGB::from(colors::BACKGROUND)),
            render_order: 2,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn spawn_magic_missile_scroll(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437(')'),
            colors: ColorPair::new(RGB::named(rltk::CYAN), RGB::from(colors::BACKGROUND)),
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
            colors: ColorPair::new(RGB::named(rltk::ORANGE), RGB::from(colors::BACKGROUND)),
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
            colors: ColorPair::new(RGB::named(rltk::CYAN), RGB::from(colors::BACKGROUND)),
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
            colors: ColorPair::new(RGB::named(rltk::CYAN), RGB::from(colors::BACKGROUND)),
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
