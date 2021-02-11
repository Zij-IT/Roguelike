use crate::{Map, Player, Position, Viewshed, TILE_REVEALED, TILE_VISIBLE};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, positions, mut map, mut viewsheds) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewsheds, &positions).join() {
            if viewshed.is_dirty {
                viewshed.is_dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|t| t.x >= 0 && t.x < map.width && t.y >= 0 && t.y < map.height);
                if players.get(ent).is_some() {
                    for idx in 0..map.tile_status.len() {
                        map.remove_tile_status(idx, TILE_VISIBLE);
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.set_tile_status(idx, TILE_REVEALED);
                        map.set_tile_status(idx, TILE_VISIBLE);
                    }
                }
            }
        }
    }
}
