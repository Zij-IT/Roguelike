use rltk::prelude::*;
use specs::prelude::*;

//All mods
mod map;
mod rect;
mod player;
mod components;
mod visibility_system;

use map::*;
use player::*;
use components::*;
use visibility_system::*;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain()
    } 
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        //Scans player input and tries to move player
        player_input(self, ctx);

        //Runs all non-player systems (for now)
        self.run_systems();

        //Draw map
        draw_map(&self.ecs, ctx);

        //Displays all entities with positions and renderables
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

//main
fn main() -> rltk::BError {
    //Creating context
    let context = RltkBuilder::simple80x50()
        .with_title("Bashing Bytes")
        .build()?;

    //Construct world
    let mut gs = State { 
        ecs: World::new(),
    };

    //Registering a components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    //Insert Map into world
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

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
        .with(Viewshed{visible_tiles : Vec::new(), range : 8, is_dirty: true})
        .build();

    rltk::main_loop(context, gs)
}
