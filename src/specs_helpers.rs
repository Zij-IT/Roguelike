use super::{
    ecs::{components::*, ParticleBuilder},
    game_log::GameLog,
    rex_assets::RexAssets,
    state::{MainOption, Menu, State},
};
use specs::{
    prelude::*,
    saveload::{SimpleMarker, SimpleMarkerAllocator},
};

///Given a specs::World, and a list of components, it registers all components in the world
macro_rules! register_all {
    ($ecs:expr, $($component:ty),* $(,)*) => {
        {
            $($ecs.register::<$component>();)*
        }
    };
}

///Given a specs::World, and a list of resources, it inserts all resources in the world
macro_rules! insert_all {
    ($ecs:expr, $($resource:expr),* $(,)*) => {
        {
            $($ecs.insert($resource);)*
        }
    };
}

pub fn register_all_components(world: &mut specs::World) {
    register_all!(
        world,
        AreaOfEffect,
        BlocksTile,
        CombatStats,
        Consumable,
        DefenseBonus,
        Equipment,
        Equipped,
        InBackpack,
        InflictsDamage,
        Item,
        MeleeDamageBonus,
        Monster,
        Name,
        ParticleLifetime,
        Player,
        Position,
        ProvidesHealing,
        Range,
        Render,
        SerializationHelper,
        SimpleMarker<SerializeMe>,
        SufferDamage,
        FieldOfView,
        WantsToDropItem,
        WantsToMelee,
        WantsToPickupItem,
        WantsToRemoveItem,
        WantsToUseItem,
    );
}

pub fn insert_all_resources(world: &mut specs::World) {
    //DEPENDENCIES:
    //player -> SimpleMarkerAllocator
    insert_all!(
        world,
        State::Menu(Menu::Main(MainOption::NewGame)),
        SimpleMarkerAllocator::<SerializeMe>::new(),
        RexAssets::load(),
        ParticleBuilder::new(),
        GameLog::new(),
    );

    //Unable to include this statement in the above batch due to the borrow checker
    //Reason: Both world::insert and spawn_player both borrow world.world mutably
    let player_ent = super::spawning::spawn_player(world, 0, 0);
    insert_all!(world, player_ent);
}
