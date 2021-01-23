use specs::prelude::*;
use super::{
    Map,
    Position,
    BlocksTile
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>);
    
    fn run(&mut self, data : Self::SystemData) {
        let (mut map, position, blockers) = data;

        map.populate_blocked();
        for (position, _) in (&position, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked_tiles[idx] = true;
        }
    }
}
