use crate::{constants::colors, Map, Position, Renderable, TileStatus, TileType};
use rltk::{Point, Rltk, RGB};
use specs::prelude::*;

const EDGE_BUFFER: usize = 2;

pub fn render_camera(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let map_width = map.width - 1;
    let map_height = map.height - 1;
    let (min_x, max_x, min_y, max_y) = get_screen_bounds(ecs);

    for (y, ty) in (min_y..max_y).enumerate().skip(EDGE_BUFFER) {
        for (x, tx) in (min_x..max_x).enumerate().skip(EDGE_BUFFER) {
            if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                let idx = map.xy_idx(tx, ty);
                if map.is_tile_status_set(idx, TileStatus::Revealed) {
                    let (glyph, fg, bg) = get_tile_glyph(idx, &*map);
                    ctx.set(x, y, fg, bg, glyph);
                }
            }
        }
    }

    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let map = ecs.fetch::<Map>();

    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

    for (pos, render) in data.iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.is_tile_status_set(idx, TileStatus::Visible) {
            let offset_x = pos.x - min_x;
            let offset_y = pos.y - min_y;
            if offset_x >= EDGE_BUFFER as i32 && offset_y >= EDGE_BUFFER as i32 {
                ctx.set(offset_x, offset_y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn get_tile_glyph(idx: usize, map: &Map) -> (rltk::FontCharType, RGB, RGB) {
    let bg = colors::BACKGROUND;
    let (glyph, fg) = match map.tiles[idx] {
        TileType::Wall => (
            35,
            if map.is_tile_status_set(idx, TileStatus::Visible) {
                colors::WALL_VISIBLE
            } else {
                colors::WALL_REVEALED
            },
        ),
        TileType::Floor => (46, colors::FLOOR),
        TileType::StairsDown => (174, colors::STAIRS),
    };

    (glyph, RGB::from(fg), RGB::from(bg))
}

pub fn get_screen_bounds(ecs: &World) -> (i32, i32, i32, i32) {
    let player_pos = ecs.fetch::<Point>();
    let (x_chars, y_chars) = (57, 43); //Set by UI Image

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;

    let min_x = player_pos.x - center_x;
    let min_y = player_pos.y - center_y;
    let max_x = player_pos.x + center_x;
    let max_y = player_pos.y + center_y;

    (min_x, max_x, min_y, max_y)
}
