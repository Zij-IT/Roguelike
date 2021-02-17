use super::ParticleBuilder;
use crate::{
    CombatStats, DefenseBonus, Equipped, GameLog, MeleeDamageBonus, Name, Position, SufferDamage,
    WantsToMelee,
};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, DefenseBonus>,
        ReadStorage<'a, Equipped>,
        ReadStorage<'a, MeleeDamageBonus>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, ParticleBuilder>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            all_stats,
            defense_bonuses,
            equipped_items,
            damage_bonuses,
            names,
            positions,
            mut gamelog,
            mut particle_builder,
            mut damages,
            mut attacks,
        ) = data;

        for (attacker, attack, name, stats) in (&entities, &attacks, &names, &all_stats).join() {
            if stats.hp > 0 {
                let mut attack_bonus_sum = 0;
                for (_, damage_bonus, equipped_item) in
                    (&entities, &damage_bonuses, &equipped_items).join()
                {
                    if equipped_item.owner == attacker {
                        attack_bonus_sum += damage_bonus.bonus;
                    }
                }

                //If the target is alive
                let target_stats = all_stats.get(attack.target).unwrap();
                if target_stats.hp > 0 {
                    let mut defense_bonus_sum = 0;
                    for (_, defense_bonus, equipped_item) in
                        (&entities, &defense_bonuses, &equipped_items).join()
                    {
                        if equipped_item.owner == attack.target {
                            defense_bonus_sum += defense_bonus.bonus;
                        }
                    }

                    //Calculate damage
                    let bonus_diff = attack_bonus_sum - defense_bonus_sum;
                    let damage = i32::max(0, stats.power - target_stats.defense + bonus_diff);
                    let target_name = &(names.get(attack.target).unwrap().name);

                    //Inform player
                    let message;
                    if damage == 0 {
                        message = format!("{} is unable to hurt {}.", &name.name, target_name);
                    } else {
                        message =
                            format!("{} hits {} for {} damage.", &name.name, target_name, damage);
                        SufferDamage::new_damage(&mut damages, attack.target, damage);
                    }
                    gamelog.push(message);

                    //Create damage effect
                    if let Some(pos) = positions.get(attack.target) {
                        particle_builder.create_particle(
                            pos.x,
                            pos.y,
                            rltk::RGB::named(rltk::ORANGE),
                            rltk::RGB::named(rltk::BLACK),
                            19, //‼
                            200.0,
                        );
                    }
                }
            }
        }
        attacks.clear();
    }
}
