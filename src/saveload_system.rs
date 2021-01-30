use super::components::*;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SerializeComponents, SimpleMarker};
use std::path::Path;

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs: &mut World) {
    let mapcopy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let save_helper = ecs
        .create_entity()
        .with(SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    let data = (
        ecs.entities(),
        ecs.read_storage::<SimpleMarker<SerializeMe>>(),
    );
    let writer = std::fs::File::create("./savegame.json").unwrap();
    let mut serializer = serde_json::Serializer::new(writer);
    serialize_individually!(
        ecs,
        serializer,
        data,
        AreaOfEffect,
        BlocksTile,
        CombatStats,
        Consumable,
        InBackpack,
        InflictsDamage,
        Item,
        Monster,
        Name,
        Player,
        Position,
        ProvidesHealing,
        Ranged,
        Renderable,
        SerializationHelper,
        SufferDamage,
        Viewshed,
        WantsToDropItem,
        WantsToMelee,
        WantsToPickupItem,
        WantsToUseItem
    );

    std::mem::drop(data);
    ecs.delete_entity(save_helper)
        .expect("Unable to delete save helper");
}

pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}
