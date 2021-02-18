use crate::{Map, Position, TileStatus, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_entity, positions, mut map, mut viewsheds) = data;

        for (ent, view, pos) in (&entities, &mut viewsheds, &positions).join() {
            if view.is_dirty {
                view.is_dirty = false;
                view.visible_tiles.clear();
                view.visible_tiles = field_of_view(Point::new(pos.x, pos.y), view.range, &*map);
                view.visible_tiles
                    .retain(|t| t.x >= 0 && t.x < map.width && t.y >= 0 && t.y < map.height);
                if ent == *player_entity {
                    for idx in 0..map.tile_status.len() {
                        map.remove_tile_status(idx, TileStatus::Visible);
                    }
                    for vis in view.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.set_tile_status(idx, TileStatus::Revealed);
                        map.set_tile_status(idx, TileStatus::Visible);
                    }
                }
            }
        }
    }
}
