use super::{
    gamelog::GameLog, AreaOfEffect, CombatStats, Consumable, Equipable, Equipped, InBackpack,
    InflictsDamage, Map, Name, Position, ProvidesHealing, SufferDamage, WantsToDropItem,
    WantsToPickupItem, WantsToUseItem,
};
use rltk::{Algorithm2D, Point};
use specs::prelude::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToPickupItem>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_ent, mut logs, names, mut backpack, mut positions, mut attempts) = data;

        for pickup in attempts.join() {
            positions.remove(pickup.item);
            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_ent {
                logs.entries.push(format!(
                    "You pick up the {}.",
                    names.get(pickup.item).unwrap().name
                ));
            }
        }
        attempts.clear();
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToDropItem>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_ent, names, mut logs, mut backpack, mut positions, mut wants_to_drop) =
            data;
        for (dropper, intent_to_drop) in (&entities, &wants_to_drop).join() {
            let dropper_pos = positions.get(dropper).unwrap().clone();
            positions
                .insert(intent_to_drop.item, dropper_pos)
                .expect("Unable to add position to dropped item");
            backpack.remove(intent_to_drop.item);
            if dropper == *player_ent {
                logs.entries.push(format!(
                    "You drop the {}",
                    names.get(intent_to_drop.item).unwrap().name
                ));
            }
        }
        wants_to_drop.clear();
    }
}

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, Consumable>,
        ReadStorage<'a, InflictsDamage>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, ProvidesHealing>,
        ReadStorage<'a, Equipable>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, WantsToUseItem>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_ent,
            map,
            aoe,
            consumables,
            damaging_items,
            names,
            healing_items,
            equipables,
            mut logs,
            mut equipped_items,
            mut backpack,
            mut all_stats,
            mut suffering,
            mut intents,
        ) = data;

        for (user, intent) in (&entities, &intents).join() {
            let mut used_item = true;

            //targeting
            let mut targets: Vec<Entity> = Vec::new();
            match intent.target {
                None => targets.push(*player_ent),
                Some(target) => match aoe.get(intent.item) {
                    None => {
                        let idx = map.xy_idx(target.x, target.y);
                        for mob in map.tile_content[idx].iter() {
                            targets.push(*mob);
                        }
                    }
                    Some(area) => {
                        let mut affected_tiles = rltk::field_of_view(target, area.radius, &*map);
                        affected_tiles.retain(|t| (*map).in_bounds(Point::new(t.x, t.y)));
                        for tile in affected_tiles.iter() {
                            let idx = map.xy_idx(tile.x, tile.y);
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                    }
                },
            }

            //apply heals
            if let Some(heal) = healing_items.get(intent.item) {
                for target in targets.iter() {
                    if let Some(stats) = all_stats.get_mut(*target) {
                        stats.hp = i32::min(stats.max_hp, stats.hp + heal.heal_amount);
                        if user == *player_ent {
                            logs.entries.push(format!(
                                "You use the {}, healing {} hp.",
                                names.get(intent.item).unwrap().name,
                                heal.heal_amount
                            ));
                        }
                        used_item = true;
                    }
                }
            }

            //deal damage
            if let Some(damage) = damaging_items.get(intent.item) {
                for mob in targets.iter() {
                    SufferDamage::new_damage(&mut suffering, *mob, damage.damage);
                    if user == *player_ent && all_stats.get(*mob).is_some() {
                        let mob_name = &names.get(*mob).unwrap().name;
                        let item_name = &names.get(intent.item).unwrap().name;
                        logs.entries.push(format!(
                            "You use {} on {} inflicting {} damage.",
                            item_name, mob_name, damage.damage
                        ));
                    }
                    used_item = true;
                }
            }

            //Equippables
            if let Some(equipable) = equipables.get(intent.item) {
                let mut to_unequip = Vec::new();
                for (item, already_equipped, name) in (&entities, &equipped_items, &names).join() {
                    if already_equipped.owner == targets[0]
                        && equipable.slot == already_equipped.slot
                    {
                        to_unequip.push(item);
                        if targets[0] == *player_ent {
                            logs.entries.push(format!("You unequip {}.", name.name));
                        }
                    }
                }

                for item in to_unequip.iter() {
                    equipped_items.remove(*item);
                    backpack
                        .insert(*item, InBackpack { owner: targets[0] })
                        .expect("Unable to put unequipped item into backpack");
                }

                equipped_items
                    .insert(
                        intent.item,
                        Equipped {
                            owner: targets[0],
                            slot: equipable.slot,
                        },
                    )
                    .expect("Unable to equip desired item");
                backpack.remove(intent.item);
                if targets[0] == *player_ent {
                    logs.entries.push(format!(
                        "You equip {}.",
                        names.get(intent.item).unwrap().name
                    ));
                }
            }

            //Consumable
            if consumables.get(intent.item).is_some() && used_item {
                entities
                    .delete(intent.item)
                    .expect("Deletion of consumable failed");
            }
        }

        intents.clear();
    }
}
