use specs::prelude::*;
use super::{
    CombatStats,
    SufferDamage,
};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = ( WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut all_stats, mut damages) = data;

        for (mut stats, damage) in (&mut all_stats, &damages).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damages.clear();
    }
}

impl DamageSystem {
    pub fn delete_the_dead(ecs: &mut World) {
        let mut dead : Vec<Entity> = Vec::new();
        //This needs to be enclosed, or entities is seen as being borrowed immutably and mutably
        {
            let all_stats = ecs.read_storage::<CombatStats>();
            let entities = ecs.entities();
            for (entity, stats) in (&entities, &all_stats).join() {
                if stats.hp < 1 {
                    dead.push(entity);
                }
            }
        }
        for victim in dead {
            ecs.delete_entity(victim).expect("Unable to delete victim");
        }
    } 
}
