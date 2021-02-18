use crate::{CombatStats, GameLog, Name, Player, RunState, SufferDamage};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut all_stats, mut damages) = data;

        for (mut stats, damage) in (&mut all_stats, &damages).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damages.clear();
    }
}

pub fn cull_dead_characters(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    //This needs to be enclosed, or entities is seen as being borrowed immutably and mutably
    {
        let mut all_stats = ecs.write_storage::<CombatStats>();
        let mut log = ecs.write_resource::<GameLog>();
        let players = ecs.read_storage::<Player>();
        let names = ecs.read_storage::<Name>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &mut all_stats).join() {
            if stats.hp < 1 {
                match players.get(entity) {
                    None => {
                        dead.push(entity);
                        if let Some(name) = names.get(entity) {
                            log.push(format!("{} is dead", &name.name));
                        }
                    }
                    Some(_) => {
                        let mut run_state = ecs.write_resource::<RunState>();
                        *run_state = RunState::GameOver;
                    }
                }
            }
        }
    }
    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete victim");
    }
}
