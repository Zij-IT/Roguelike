use super::{gamelog::GameLog, InBackpack, Name, Position, WantsToPickupItem};
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
