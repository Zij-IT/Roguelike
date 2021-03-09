use crate::{components::*, map_builder::map::Map};
use specs::{
    error::NoError,
    prelude::*,
    saveload::{
        DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker,
        SimpleMarkerAllocator,
    },
};
use std::{fs, path::Path};

const SAVE_PATH: &str = "./saves/savegame.ron";

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),* $(,)?) => {
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

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),* $(,)?) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &$data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocator
            &mut $de,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<Map>().unwrap().clone();
    let save_helper = ecs
        .create_entity()
        .with(SerializationHelper { map: map_copy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    {
        let data = (
            ecs.entities(),
            ecs.read_storage::<SimpleMarker<SerializeMe>>(),
        );
        let writer = std::fs::File::create(SAVE_PATH).unwrap();

        let mut serializer = ron::Serializer::new(writer, None, false).unwrap();
        serialize_individually!(
            ecs,
            serializer,
            data,
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
            SufferDamage,
            FieldOfView,
            WantsToDropItem,
            WantsToMelee,
            WantsToPickupItem,
            WantsToRemoveItem,
            WantsToUseItem,
        );
    }

    ecs.delete_entity(save_helper)
        .expect("Unable to delete save helper");
}

pub fn load_game(ecs: &mut World) {
    {
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in &to_delete {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    let data = fs::read_to_string(SAVE_PATH).unwrap();
    let mut de = ron::Deserializer::from_str(&data).unwrap();

    {
        let mut d = (
            &mut ecs.entities(),
            &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(),
            &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>(),
        );
        deserialize_individually!(
            ecs,
            de,
            d,
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
            SufferDamage,
            FieldOfView,
            WantsToDropItem,
            WantsToMelee,
            WantsToPickupItem,
            WantsToRemoveItem,
            WantsToUseItem,
        );
    }

    let mut delete_me = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e, h) in (&entities, &helper).join() {
            let mut world_map = ecs.write_resource::<Map>();
            *world_map = h.map.clone();
            world_map.tile_content =
                vec![Vec::new(); (world_map.width * world_map.height) as usize];
            delete_me = Some(e);
        }
        for (e, _, pos) in (&entities, &player, &position).join() {
            let mut player_pos = ecs.write_resource::<rltk::Point>();
            let mut player_ent = ecs.write_resource::<Entity>();
            *player_pos = rltk::Point::new(pos.x, pos.y);
            *player_ent = e;
        }
    }

    ecs.delete_entity(delete_me.unwrap())
        .expect("Unable to delete helper");
}

pub fn does_save_exist() -> bool {
    Path::new(SAVE_PATH).exists()
}

pub fn delete_save() {
    if does_save_exist() {
        std::fs::remove_file(SAVE_PATH).expect("Unable to delete file");
    }
}
