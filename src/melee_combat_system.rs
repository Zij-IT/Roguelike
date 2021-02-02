use super::{
    CombatStats, DefenseBonus, Equipped, GameLog, MeleeDamageBonus, Name, SufferDamage,
    WantsToMelee,
};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Equipped>,
        ReadStorage<'a, MeleeDamageBonus>,
        ReadStorage<'a, DefenseBonus>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            all_stats,
            names,
            equipped_items,
            damage_bonuses,
            defense_bonuses,
            mut gamelog,
            mut attacks,
            mut damages,
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

                //Get targets stats
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
                    let bonus_diff = attack_bonus_sum - defense_bonus_sum;
                    let damage = i32::max(0, stats.power - target_stats.defense + bonus_diff);
                    let target_name = &(names.get(attack.target).unwrap().name);
                    let message;
                    if damage == 0 {
                        message = format!("{} is unable to hurt {}.", &name.name, target_name);
                    } else {
                        message =
                            format!("{} hits {} for {} damage.", &name.name, target_name, damage);
                        SufferDamage::new_damage(&mut damages, attack.target, damage);
                    }
                    gamelog.entries.push(message.to_string());
                }
            }
        }
        attacks.clear();
    }
}
