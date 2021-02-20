pub mod components;
mod systems;
pub use components::*;
pub use systems::cull_dead_characters;
pub use systems::cull_dead_particles;
pub use systems::ParticleBuilder;

pub mod pre_run_systems {
    use crate::ecs::systems;
    use specs::{RunNow, WorldExt};

    pub fn execute(world: &mut specs::World) {
        let mut vis = systems::VisibilitySystem {};
        let mut map_index = systems::MapIndexingSystem {};

        vis.run_now(world);
        map_index.run_now(world);

        world.maintain();
    }
}

pub mod all_systems {
    use crate::ecs::systems;
    use specs::{RunNow, WorldExt};

    pub fn execute(world: &mut specs::World) {
        let mut vis = systems::VisibilitySystem {};
        let mut map_index = systems::MapIndexingSystem {};
        let mut mons = systems::MonsterAI {};
        let mut melee = systems::MeleeCombatSystem {};
        let mut damage = systems::DamageSystem {};
        let mut pickup_items = systems::ItemCollectionSystem {};
        let mut use_items = systems::ItemUseSystem {};
        let mut drop_items = systems::ItemDropSystem {};
        let mut rem_items = systems::ItemRemoveSystem {};
        let mut particles = systems::ParticleSpawnSystem {};

        vis.run_now(world);
        mons.run_now(world);
        map_index.run_now(world);
        melee.run_now(world);
        damage.run_now(world);
        pickup_items.run_now(world);
        use_items.run_now(world);
        drop_items.run_now(world);
        rem_items.run_now(world);
        particles.run_now(world);

        world.maintain();
    }
}
