use specs::prelude::*;
use super::{
    Viewshed,
    Position,
    Map,
    Monster,
    Name,
};
use rltk::{
    Point,
    console,
};

pub struct MonsterAI{}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect< 'a, Map>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, Viewshed>,
                        ReadExpect<  'a, Point>,
                        ReadStorage< 'a, Monster>,
                        ReadStorage< 'a, Name>);
    
    fn run(&mut self, data : Self::SystemData) {
        let (map, mut positions, mut viewsheds, player_pos, monsters, names) = data;

        for (mut viewshed, mut pos, name, _) in (&mut viewsheds, &mut positions, &names, &monsters).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance <= 1.0 {
                    console::log(&format!("{} shouts insults", name.name));
                }
                else {
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &*map
                    );

                    if path.success && path.steps.len() > 1 {
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        viewshed.is_dirty = true;
                    }
                }

            }
        }
    }
}
