use super::{
    components::{
        CombatStats, FieldOfView, Item, Monster, Player, Position, WantsToMelee, WantsToPickupItem,
    },
    BashingBytes, GameLog,
};
use crate::{
    gui::inventory::InvMode,
    map_builder::map::{Map, TileStatus, TileType},
    state::Gameplay,
};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::{Entity, Join, World, WorldExt};

pub fn respond_to_input(game: &mut BashingBytes, ctx: &mut Rltk) -> Gameplay {
    let keys = &game.configs.keys;
    if let Some(key) = ctx.key {
        if key == keys.move_up {
            try_move(0, -1, &mut game.world);
        } else if key == keys.move_down {
            try_move(0, 1, &mut game.world);
        } else if key == keys.move_left {
            try_move(-1, 0, &mut game.world);
        } else if key == keys.move_right {
            try_move(1, 0, &mut game.world);
        } else if key == keys.move_up_left {
            try_move(-1, -1, &mut game.world);
        } else if key == keys.move_up_right {
            try_move(1, -1, &mut game.world);
        } else if key == keys.move_down_left {
            try_move(-1, 1, &mut game.world);
        } else if key == keys.move_down_right {
            try_move(1, 1, &mut game.world);
        } else if key == keys.descend {
            return try_descend(&mut game.world);
        } else if key == keys.grab_item {
            try_pickup(&mut game.world);
        } else if key == keys.drop_item {
            return Gameplay::Inventory(InvMode::Drop);
        } else if key == keys.remove_item {
            return Gameplay::Inventory(InvMode::Remove);
        } else if key == keys.open_inventory {
            return Gameplay::Inventory(InvMode::Use);
        } else if key == keys.go_back {
            return Gameplay::SaveGame;
        } else if key == keys.wait_turn {
            return skip_turn(&mut game.world);
        } else if key == VirtualKeyCode::M {
            //Cheat key :D
        } else {
            return Gameplay::AwaitingInput;
        }
    } else {
        return Gameplay::AwaitingInput;
    }

    Gameplay::PlayerTurn
}

fn try_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut viewsheds = ecs.write_storage::<FieldOfView>();
    let mut players = ecs.write_storage::<Player>();
    let mut attacks = ecs.write_storage::<WantsToMelee>();
    let entities = ecs.entities();

    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = &ecs.fetch::<Map>();

    //Allows the player to attack if position is occupied
    for (entity, _, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        //Check bounds
        if pos.x + delta_x < 1
            || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > map.height - 1
        {
            return;
        }

        //Attack if possible
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        for potential_target in &map.tile_content[destination_idx] {
            if combat_stats.get(*potential_target).is_some() {
                attacks
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

        //If not blocked, moves the player there
        if !map.is_tile_status_set(destination_idx, TileStatus::Blocked) {
            pos.x = std::cmp::min(map.width - 1, std::cmp::max(0, pos.x + delta_x));
            pos.y = std::cmp::min(map.height - 1, std::cmp::max(0, pos.y + delta_y));
            let mut player_pos = ecs.write_resource::<Point>();
            player_pos.x = pos.x;
            player_pos.y = pos.y;
            viewshed.is_dirty = true;
        }
    }
}

fn try_pickup(ecs: &mut World) {
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let player_ent = ecs.fetch::<Entity>();
    let player_pos = ecs.fetch::<Point>();
    let positions = ecs.read_storage::<Position>();
    let mut logs = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_ent, _, pos) in (&entities, &items, &positions).join() {
        if pos.x == player_pos.x && pos.y == player_pos.y {
            target_item = Some(item_ent);
        }
    }

    match target_item {
        None => logs.push(&"There is nothing to pick up"),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_ent,
                    WantsToPickupItem {
                        collected_by: *player_ent,
                        item,
                    },
                )
                .expect("Could not insert the item into wants to pickup");
        }
    }
}

fn try_descend(ecs: &mut World) -> Gameplay {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::StairsDown {
        Gameplay::NextLevel
    } else {
        let mut logs = ecs.fetch_mut::<GameLog>();
        logs.push(&"There is no way down from here.");
        Gameplay::AwaitingInput
    }
}

fn skip_turn(ecs: &mut World) -> Gameplay {
    let viewshed_comps = ecs.read_storage::<FieldOfView>();
    let player_ent = ecs.fetch::<Entity>();
    let player_vs = viewshed_comps.get(*player_ent).unwrap();
    let mobs = ecs.read_storage::<Monster>();
    let map = ecs.fetch::<Map>();

    //Checks if the point contains a mob given the map
    let contains_mob = |tile: Point| {
        let idx = map.xy_idx(tile.x, tile.y);
        map.tile_content[idx]
            .iter()
            .any(|ent| mobs.get(*ent).is_some())
    };

    //If the players viewshed does not contain mobs they may heal a point by waiting
    if !player_vs
        .visible_tiles
        .iter()
        .any(|&tile| contains_mob(tile))
    {
        let mut all_stats = ecs.write_storage::<CombatStats>();
        let player_stats = all_stats.get_mut(*player_ent).unwrap();
        player_stats.hp = i32::min(player_stats.hp + 1, player_stats.max_hp);
    }

    Gameplay::PlayerTurn
}
