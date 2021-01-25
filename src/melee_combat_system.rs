use specs::prelude::*;
use super::{
    CombatStats,
    WantsToMelee,
    Name,
    SufferDamage,
};
use rltk::console;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToMelee>,
                        WriteStorage<'a, SufferDamage>,
                        ReadStorage<'a, CombatStats>,
                        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut attacks, mut damages, all_stats, names) = data;

        for (_, attack, name, stats) in (&entities, &attacks, &names, &all_stats).join() {
            if stats.hp > 0 {
                let target_stats = all_stats.get(attack.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = &(names.get(attack.target).unwrap().name);
                    let damage = i32::max(0, stats.power - target_stats.defense);
                    if damage == 0 {
                        console::log(&format!("{} is unable to hurt {}.", 
                            &name.name, 
                            target_name
                        ));
                    }
                    else {
                        console::log(&format!("{} hits {} for {} damage.", 
                            &name.name,
                            target_name,
                            damage
                        ));
                        SufferDamage::new_damage(&mut damages, attack.target, damage);
                    }
                }
            }
        }
        attacks.clear();
    }
}
