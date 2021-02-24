use super::{
    components::{
        CombatStats, Item, Monster, Player, Position, Viewshed, WantsToMelee, WantsToPickupItem,
    },
    raws::config::CONFIGS,
    EcsWorld, GameLog, RunState,
};
use crate::map_builder::map::{Map, TileStatus, TileType};
use rltk::{Point, Rltk};
use specs::{Entity, Join, World, WorldExt};

pub fn respond_to_input(gs: &mut EcsWorld, ctx: &mut Rltk) -> RunState {
    let keys = &CONFIGS.lock().unwrap().keys;
    if let Some(key) = ctx.key {
        if key == keys.move_up {
            try_move(0, -1, &mut gs.world);
        } else if key == keys.move_down {
            try_move(0, 1, &mut gs.world);
        } else if key == keys.move_left {
            try_move(-1, 0, &mut gs.world);
        } else if key == keys.move_right {
            try_move(1, 0, &mut gs.world);
        } else if key == keys.move_up_left {
            try_move(-1, -1, &mut gs.world);
        } else if key == keys.move_up_right {
            try_move(1, -1, &mut gs.world);
        } else if key == keys.move_down_left {
            try_move(-1, 1, &mut gs.world);
        } else if key == keys.move_down_right {
            try_move(1, 1, &mut gs.world);
        } else if key == keys.descend {
            return try_descend(&mut gs.world);
        } else if key == keys.grab_item {
            try_pickup(&mut gs.world);
        } else if key == keys.drop_item {
            return RunState::ShowDropItem;
        } else if key == keys.remove_item {
            return RunState::ShowRemoveItem;
        } else if key == keys.open_inventory {
            return RunState::ShowInventory;
        } else if key == keys.go_back {
            return RunState::SaveGame;
        } else if key == keys.wait_turn {
            return skip_turn(&mut gs.world);
        } else {
            return RunState::AwaitingInput;
        }
    } else {
        return RunState::AwaitingInput;
    }

    RunState::PlayerTurn
}

fn try_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
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

fn try_descend(ecs: &mut World) -> RunState {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::StairsDown {
        RunState::NextLevel
    } else {
        let mut logs = ecs.fetch_mut::<GameLog>();
        logs.push(&"There is no way down from here.");
        RunState::AwaitingInput
    }
}

fn skip_turn(ecs: &mut World) -> RunState {
    let viewshed_comps = ecs.read_storage::<Viewshed>();
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

    RunState::PlayerTurn
}
