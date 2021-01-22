use rltk::{
    VirtualKeyCode,
    Rltk,
};
use specs::prelude::*;
use super::{
    components::{
        Position,
        Player,
        Viewshed,
    },
    map::{
        TileType,
        Map,
    },
    State,
};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    let map = &ecs.fetch::<Map>();

    for (_, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = std::cmp::min(79, std::cmp::max(0, pos.x + delta_x));
            pos.y = std::cmp::min(49, std::cmp::max(0, pos.y + delta_y));
            viewshed.is_dirty = true;
        } 
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::H | 
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L | 
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::K | 
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::J | 
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {},
        }
    }
}
