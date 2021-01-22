use specs::prelude::*;
use rltk::{field_of_view, Point};
use super::{Map, Viewshed, Position, Player};

pub struct VisibilitySystem {}

impl <'a> System <'a> for VisibilitySystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>, 
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewsheds, poses, players) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewsheds, &poses).join() {
            viewshed.visible_tiles.clear();
            if viewshed.is_dirty {
                viewshed.is_dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(pos.x, pos.y), 
                    viewshed.range,
                    &*map
                );
                viewshed.visible_tiles.retain(|t| t.x >= 0 && t.x < map.width &&
                                              t.y >= 0 && t.y < map.height);
                if let Some(_) = players.get(ent) {
                    for t in map.visible_tiles.iter_mut() { *t = false };
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
