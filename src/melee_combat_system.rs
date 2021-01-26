use specs::prelude::*;
use super::{
    CombatStats,
    WantsToMelee,
    Name,
    SufferDamage,
    GameLog,
};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, WantsToMelee>,
                        WriteStorage<'a, SufferDamage>,
                        ReadStorage<'a, CombatStats>,
                        ReadStorage<'a, Name>,
                        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, mut attacks, mut damages, all_stats, names, entities) = data;

        for (_, attack, name, stats) in (&entities, &attacks, &names, &all_stats).join() {
            if stats.hp > 0 {
                let target_stats = all_stats.get(attack.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = &(names.get(attack.target).unwrap().name);
                    let damage = i32::max(0, stats.power - target_stats.defense);
                    let message;
                    if damage == 0 {
                        message = format!("{} is unable to hurt {}.", 
                            &name.name, 
                            target_name
                        );
                    }
                    else {
                        message = format!("{} hits {} for {} damage.", 
                            &name.name, 
                            target_name, 
                            damage
                        );
                        SufferDamage::new_damage(&mut damages, attack.target, damage);
                    }
                    gamelog.entries.push(message.to_string());
                }
            }
        }
        attacks.clear();
    }
}
