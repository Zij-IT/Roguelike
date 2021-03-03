use crate::{
    constants::{colors, consoles},
    ecs::{Equipped, InBackpack, Name},
    raws::config::CONFIGS,
    rex_assets,
    state::{Gameplay, State, State::Game},
};
use rltk::{Rltk, RGB};
use specs::{Entity, Join, World, WorldExt};

#[derive(PartialEq, Copy, Clone)]
pub enum InvResult {
    Cancel,
    NoResponse,
    Selected(Entity),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum InvMode {
    Use,
    Drop,
    Remove,
}

pub fn show(world: &mut World, ctx: &mut Rltk) -> InvResult {
    let player_ent = world.fetch::<Entity>();
    let current_state = world.fetch::<State>();
    let names = world.read_storage::<Name>();
    let entities = world.entities();

    //Get all relevant items
    //Unable to simplify to avoid the duplication of the lines .join() .. .collect() because the
    //if arms are of different types.
    #[allow(clippy::filter_map)]
    let relevant_entities = {
        if *current_state == Game(Gameplay::Inventory(InvMode::Remove)) {
            let equipped_items = world.read_storage::<Equipped>();
            (&equipped_items, &names, &entities)
                .join()
                .filter(|item| item.0.owner == *player_ent)
                .map(|item| (item.1, item.2))
                .collect::<Vec<_>>()
        } else {
            let backpack_items = world.read_storage::<InBackpack>();
            (&backpack_items, &names, &entities)
                .join()
                .filter(|item| item.0.owner == *player_ent)
                .map(|item| (item.1, item.2))
                .collect::<Vec<_>>()
        }
    };

    ctx.set_active_console(consoles::HUD_CONSOLE);
    let assets = world.fetch::<rex_assets::RexAssets>();
    ctx.render_xp_sprite(&assets.blank_inv, 0, 0);

    //Base locations
    let base_x = 3;
    let base_y = 4;

    //Print out relevant items
    for (offset, (name, _)) in relevant_entities.iter().enumerate() {
        let y = base_y + offset as i32;
        ctx.set(
            base_x + 1,
            y,
            RGB::named(rltk::YELLOW),
            RGB::from(colors::BACKGROUND),
            97 + offset as rltk::FontCharType,
        );
        ctx.set(
            base_x + 2,
            y,
            RGB::from(colors::FOREGROUND),
            RGB::from(colors::BACKGROUND),
            rltk::to_cp437(')'),
        );
        ctx.print(base_x + 4, y, &name.name.to_string());
    }

    //Respond to players response
    let keys = &CONFIGS.lock().unwrap().keys;
    if let Some(key) = ctx.key {
        return if key == keys.go_back {
            InvResult::Cancel
        } else {
            let selection = rltk::letter_to_option(key);
            if selection > -1 && selection < relevant_entities.len() as i32 {
                return InvResult::Selected(relevant_entities[selection as usize].1);
            }
            InvResult::NoResponse
        };
    }
    InvResult::NoResponse
}
