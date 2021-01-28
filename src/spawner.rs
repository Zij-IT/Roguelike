use super::{
    rect::Rect, BlocksTile, CombatStats, Item, Monster, Name, Player, Position, Potion, Renderable,
    Viewshed,
};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;

const MAX_MONSTERS: i32 = 4; //Per room
const MAX_ITEMS: i32 = 2; //Per room

pub fn spawn_player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
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
        .build()
}

pub fn spawn_health_pot(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Position { x, y })
        .with(Item {})
        .with(Potion { heal_amount: 8 })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Renderable {
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
        })
        .build()
}

pub fn populate_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawns: Vec<(i32, i32, i32)> = Vec::new();
    let mut item_spawns: Vec<(i32, i32)> = Vec::new();
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
    let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

    for _ in 0..num_monsters {
        loop {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
            let n = rng.roll_dice(1, 2);
            if !monster_spawns.contains(&(x, y, n)) {
                monster_spawns.push((x, y, n));
                break;
            }
        }
    }

    for _ in 0..num_items {
        loop {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
            if !item_spawns.contains(&(x, y)) {
                item_spawns.push((x, y));
                break;
            }
        }
    }

    std::mem::drop(rng);
    for (x, y, n) in monster_spawns.iter() {
        match *n {
            1 => spawn_kobold(ecs, *x, *y),
            _ => spawn_goblin(ecs, *x, *y),
        };
    }

    for (x, y) in item_spawns.iter() {
        spawn_health_pot(ecs, *x, *y);
    }
}
