use super::{
    rect::Rect, AreaOfEffect, BlocksTile, CombatStats, Consumable, InflictsDamage, Item, Monster,
    Name, Player, Position, ProvidesHealing, Ranged, Renderable, SerializeMe, Viewshed,
};
use rltk::{RandomNumberGenerator, RGB};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

const MAX_MONSTERS: i32 = 4; //Per room
const MAX_ITEMS: i32 = 10; //Per room

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

//ROOM POPULATION-----
pub fn populate_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawns: Vec<(i32, i32)> = Vec::new();
    let mut item_spawns: Vec<(i32, i32)> = Vec::new();
    let mut monsters = Vec::new();
    let mut items = Vec::new();
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
    let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

    for _ in 0..num_monsters {
        loop {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
            let n = rng.roll_dice(1, 2);
            if !monster_spawns.contains(&(x, y)) {
                monster_spawns.push((x, y));
                monsters.push(n);
                break;
            }
        }
    }

    for _ in 0..num_items {
        loop {
            let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
            let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));
            let n = rng.roll_dice(1, 3);
            if !item_spawns.contains(&(x, y)) {
                item_spawns.push((x, y));
                items.push(n);
                break;
            }
        }
    }

    //Gotta please the borrow checker
    std::mem::drop(rng);
    for (n, (x, y)) in monster_spawns.iter().enumerate() {
        match monsters[n] {
            1 => spawn_kobold(ecs, *x, *y),
            _ => spawn_goblin(ecs, *x, *y),
        };
    }

    for (n, (x, y)) in item_spawns.iter().enumerate() {
        match items[n] {
            1 => spawn_health_pot(ecs, *x, *y),
            2 => spawn_fireball_scroll(ecs, *x, *y),
            _ => spawn_magic_missile_scroll(ecs, *x, *y),
        };
    }
}
