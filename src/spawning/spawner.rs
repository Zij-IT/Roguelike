use super::random_table::RandomTable;
use crate::{
    constants::colors,
    ecs::components::{CombatStats, Name, Player, Position, Render, SerializeMe, Viewshed},
    map_builder::{
        map::{Map, TileType},
        rect::Rect,
    },
    raws::spawning::{SpawnType, SPAWN_RAWS},
};
use rltk::{ColorPair, RandomNumberGenerator, RGB};
use specs::{
    prelude::*,
    saveload::{MarkedBuilder, SimpleMarker},
};
use std::collections::HashMap;

const MAX_MONSTERS: i32 = 4;

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
            0_usize
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
    for spawn in &spawn_points {
        spawn_named_entity(ecs, &spawn);
    }
}

pub fn spawn_player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Player {})
        .with(Render {
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

fn create_room_table(map_depth: i32) -> RandomTable {
    SPAWN_RAWS.lock().unwrap().spawn_table(map_depth)
}

fn spawn_named_entity(ecs: &mut World, ((x, y), name): &(&(i32, i32), &String)) {
    if SPAWN_RAWS
        .lock()
        .unwrap()
        .spawn_named_entity(ecs.create_entity(), name, SpawnType::AtPosition(*x, *y))
        .is_some()
    {
        return;
    }

    println!("There exists no entity with the name \"{}\" to spawn", name);
}
