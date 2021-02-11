use crate::{Map, Monster, Position, RunState, Viewshed, WantsToMelee};
use rltk::Point;
use specs::prelude::*;

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        ReadStorage<'a, Monster>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_pos,
            player_ent,
            runstate,
            monsters,
            map,
            mut positions,
            mut viewsheds,
            mut attacks,
        ) = data;
        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (mut vs, mut pos, ent, _) in
            (&mut viewsheds, &mut positions, &entities, &monsters).join()
        {
            //If monster can see player attack if within range or approach
            if vs.visible_tiles.contains(&*player_pos) {
                let distance =
                    rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance <= 1.0 {
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
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        vs.is_dirty = true;
                    }
                }
            }
        }
    }
}
