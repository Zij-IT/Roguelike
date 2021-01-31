use super::{
    components::{CombatStats, Item, Player, Position, Viewshed, WantsToMelee, WantsToPickupItem},
    map::Map,
    GameLog, RunState, State, TILE_BLOCKED,
};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => {
            return RunState::AwaitingInput;
        }
        Some(key) => match key {
            VirtualKeyCode::H | VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L | VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::K | VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::J | VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::I => return RunState::ShowInventory,
            VirtualKeyCode::D => return RunState::ShowDropItem,
            VirtualKeyCode::Escape => return RunState::SaveGame,
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
        if !map.is_tile_status_set(destination_idx, TILE_BLOCKED) {
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
