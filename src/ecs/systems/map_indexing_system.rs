use crate::{
    components::{BlocksTile, Position},
    map_builder::map::{Map, TileStatus},
};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, position, blockers, mut map) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (position, entity) in (&position, &entities).join() {
            let idx = map.xy_idx(position.x, position.y);
            if blockers.get(entity).is_some() {
                map.set_tile_status(idx, TileStatus::Blocked);
            }
            map.tile_content[idx].push(entity);
        }
    }
}
