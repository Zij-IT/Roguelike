use rltk::prelude::*;
use specs::prelude::*;

//All mods
mod map;
mod rect;
mod player;
mod components;
mod game_state;

use map::*;
use game_state::*;
use components::*;

//main
fn main() -> rltk::BError {
    //Creating context
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    //Construct world
    let mut gs = State { 
        ecs: World::new(),
    };

    //Insert Map into world
    let (map, rooms) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);

    //Registering a component as it is not in a system
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    //Grab player
    let (player_x, player_y) = rooms[0].center();

    //Build player
    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Player{})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build();

    rltk::main_loop(context, gs)
}
