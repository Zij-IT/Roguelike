use crate::{
    components::{FieldOfView, Position},
    map_builder::map::{Map, TileStatus},
};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, FieldOfView>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_entity, positions, mut map, mut fields_of_view) = data;

        for (ent, fov, pos) in (&entities, &mut fields_of_view, &positions).join() {
            if fov.is_dirty {
                fov.is_dirty = false;
                fov.visible_tiles.clear();
                fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
                fov.visible_tiles
                    .retain(|t| t.x >= 0 && t.x < map.width && t.y >= 0 && t.y < map.height);
                if ent == *player_entity {
                    for idx in 0..map.tile_status.len() {
                        map.remove_tile_status(idx, TileStatus::Visible);
                    }
                    for vis in &fov.visible_tiles {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.set_tile_status(idx, TileStatus::Revealed);
                        map.set_tile_status(idx, TileStatus::Visible);
                    }
                }
            }
        }
    }
}
