use super::{
    components::{
        CombatStats, Item, Monster, Player, Position, Viewshed, WantsToMelee, WantsToPickupItem,
    },
    GameLog, RunState, State,
};
use crate::{map::*, TileStatus, TileType};
use rltk::{Point, Rltk, VirtualKeyCode as VKC};
use specs::prelude::*;

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => {
            return RunState::AwaitingInput;
        }
        Some(key) => match key {
            //Movement keys
            VKC::H | VKC::Left => try_move_player(-1, 0, &mut gs.ecs),
            VKC::L | VKC::Right => try_move_player(1, 0, &mut gs.ecs),
            VKC::K | VKC::Up => try_move_player(0, -1, &mut gs.ecs),
            VKC::J | VKC::Down => try_move_player(0, 1, &mut gs.ecs),
            //Item keys
            VKC::G => get_item(&mut gs.ecs),
            VKC::I => return RunState::ShowInventory,
            VKC::D => return RunState::ShowDropItem,
            VKC::R => return RunState::ShowRemoveItem,
            //Save key
            VKC::Escape => return RunState::SaveGame,
            //Skip key
            VKC::Space => return skip_turn(&mut gs.ecs),
            //Descend Key
            VKC::Period => {
                if try_next_level(&mut gs.ecs) {
                    return RunState::NextLevel;
                }
            }
            //Ignore others
            _ => {
                return RunState::AwaitingInput;
            }
        },
    }
    RunState::PlayerTurn
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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
        if pos.x + delta_x < 1
            || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > map.height - 1
        {
            return;
        }

        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        for potential_target in map.tile_content[destination_idx].iter() {
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
            pos.x = std::cmp::min(79, std::cmp::max(0, pos.x + delta_x));
            pos.y = std::cmp::min(49, std::cmp::max(0, pos.y + delta_y));
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
            viewshed.is_dirty = true;
        }
    }
}

fn get_item(ecs: &mut World) {
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
        None => logs.entries.push("There is nothing to pick up".to_string()),
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

fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::StairsDown {
        true
    } else {
        let mut logs = ecs.fetch_mut::<GameLog>();
        logs.entries
            .push("There is no way down from here.".to_string());
        false
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
