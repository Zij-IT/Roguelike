use crate::{
    components::{FieldOfView, Monster, Position, WantsToMelee},
    map_builder::map::Map,
    state::{Gameplay, State, State::Game},
};
use rltk::Point;
use specs::prelude::*;

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, State>,
        ReadStorage<'a, Monster>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_pos,
            player_ent,
            state,
            monsters,
            map,
            mut positions,
            mut fields_of_view,
            mut attacks,
        ) = data;

        if *state != Game(Gameplay::MonsterTurn) {
            return;
        }

        for (mut fov, mut pos, ent, _) in
            (&mut fields_of_view, &mut positions, &entities, &monsters).join()
        {
            //If monster can see player attack if within range or approach
            if fov.visible_tiles.contains(&*player_pos) {
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 2.0 {
                    attacks
                        .insert(
                            ent,
                            WantsToMelee {
                                target: *player_ent,
                            },
                        )
                        .expect("Unable to insert attack");
                } else {
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &*map,
                    );

                    if path.success && path.steps.len() > 1 {
                        //Do note, that this does NOT check if the player is there
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        fov.is_dirty = true;
                    }
                }
            }
        }
    }
}
