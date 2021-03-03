use crate::{
    components::{
        AreaOfEffect, CombatStats, Consumable, Equipment, Equipped, InBackpack, InflictsDamage,
        Name, Position, ProvidesHealing, SufferDamage, WantsToDropItem, WantsToPickupItem,
        WantsToRemoveItem, WantsToUseItem,
    },
    game_log::GameLog,
    map_builder::map::Map,
};
use rltk::{Algorithm2D, Point};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

const INVENTORY_LIMIT: usize = 9;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToPickupItem>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_ent, names, mut logs, mut backpack, mut positions, mut attempts) = data;

        let player_inventory_size = (&backpack)
            .join()
            .filter(|&x| x.owner == *player_ent)
            .count();

        for pickup in attempts.join() {
            if player_inventory_size >= INVENTORY_LIMIT {
                logs.push(&format!(
                    "You are unable to pick up the {}.",
                    names.get(pickup.item).unwrap().name
                ));
                logs.push(&"You are carrying too many items!");
                attempts.clear();
                return;
            }
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
                logs.push(&format!(
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
        let (
            entities,
            player_ent,
            names,
            mut logs,
            mut backpack,
            mut positions,
            mut intents_to_drop,
        ) = data;
        for (dropper, intent_to_drop) in (&entities, &intents_to_drop).join() {
            let dropper_pos = positions.get(dropper).unwrap().clone();
            positions
                .insert(intent_to_drop.item, dropper_pos)
                .expect("Unable to add position to dropped item");
            backpack.remove(intent_to_drop.item);
            if dropper == *player_ent {
                logs.push(&format!(
                    "You drop the {}",
                    names.get(intent_to_drop.item).unwrap().name
                ));
            }
        }
        intents_to_drop.clear();
    }
}

pub struct ItemRemoveSystem {}

impl<'a> System<'a> for ItemRemoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToRemoveItem>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_ent, names, mut logs, mut intents, mut equipped_items, mut backpacks) =
            data;
        for (entity, intent) in (&entities, &intents).join() {
            equipped_items.remove(intent.item);
            backpacks
                .insert(intent.item, InBackpack { owner: entity })
                .expect("Unable to insert item into backpack");
            if entity == *player_ent {
                logs.push(&format!(
                    "You unequip the {}",
                    names.get(intent.item).unwrap().name
                ))
            }
        }

        intents.clear();
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
        ReadStorage<'a, Equipment>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, WantsToUseItem>,
    );

    #[allow(clippy::too_many_lines)]
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

            //Get all targets!
            let mut targets: Vec<Entity> = Vec::new();
            match intent.target {
                None => targets.push(user),
                Some(target) => match aoe.get(intent.item) {
                    None => {
                        let idx = map.xy_idx(target.x, target.y);
                        for mob in &map.tile_content[idx] {
                            targets.push(*mob);
                        }
                    }
                    Some(area) => {
                        let mut affected_tiles = rltk::field_of_view(target, area.radius, &*map);
                        affected_tiles.retain(|t| (*map).in_bounds(Point::new(t.x, t.y)));
                        for tile in &affected_tiles {
                            let idx = map.xy_idx(tile.x, tile.y);
                            for mob in &map.tile_content[idx] {
                                targets.push(*mob);
                            }
                        }
                    }
                },
            }

            //if the item heals...
            if let Some(heal) = healing_items.get(intent.item) {
                for target in &targets {
                    if let Some(stats) = all_stats.get_mut(*target) {
                        stats.hp = i32::min(stats.max_hp, stats.hp + heal.heal_amount);
                        if user == *player_ent {
                            logs.push(&format!(
                                "You use the {}, healing {} hp.",
                                names.get(intent.item).unwrap().name,
                                heal.heal_amount
                            ));
                        }
                        used_item = true;
                    }
                }
            }

            //if the item deals damage on use...
            if let Some(damage) = damaging_items.get(intent.item) {
                for mob in &targets {
                    SufferDamage::new_damage(&mut suffering, *mob, damage.damage);
                    if user == *player_ent && all_stats.get(*mob).is_some() {
                        let mob_name = &names.get(*mob).unwrap().name;
                        let item_name = &names.get(intent.item).unwrap().name;
                        logs.push(&format!(
                            "You use {} on {} inflicting {} damage.",
                            item_name, mob_name, damage.damage
                        ));
                    }
                    used_item = true;
                }
            }

            //If the item can be equipped...
            if let Some(equipment) = equipables.get(intent.item) {
                //De-equip all items that share a slot
                let mut to_unequip = Vec::new();
                for (item, already_equipped, name) in (&entities, &equipped_items, &names).join() {
                    if already_equipped.owner == targets[0]
                        && equipment.slot == already_equipped.slot
                    {
                        to_unequip.push(item);
                        if targets[0] == *player_ent {
                            logs.push(&format!("You unequip {}.", name.name));
                        }
                    }
                }

                for item in &to_unequip {
                    equipped_items.remove(*item);
                    backpack
                        .insert(*item, InBackpack { owner: targets[0] })
                        .expect("Unable to put unequipped item into backpack");
                }

                //Equip item
                equipped_items
                    .insert(
                        intent.item,
                        Equipped {
                            owner: targets[0],
                            slot: equipment.slot,
                        },
                    )
                    .expect("Unable to equip desired item");
                backpack.remove(intent.item);

                //Inform if player is equipping
                if targets[0] == *player_ent {
                    logs.push(&format!(
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
