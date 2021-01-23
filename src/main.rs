use rltk::prelude::*;
use specs::prelude::*;

//All mods
mod map;
mod rect;
mod player;
mod components;
mod visibility_system;
mod monster_ai_system;
mod map_indexing_system;

use map::*;
use player::*;
use components::*;
use visibility_system::*;
use monster_ai_system::*;
use map_indexing_system::*;

//Enums
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub run_state : RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        let mut mons = MonsterAI{};
        let mut mapindex = MapIndexingSystem{};

        vis.run_now(&self.ecs);
        mons.run_now(&self.ecs);
        mapindex.run_now(&self.ecs);

        self.ecs.maintain();
    } 
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        //Runs all non-player systems (for now)
        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }

        //Draw map
        draw_map(&self.ecs, ctx);

        //Displays all entities with positions and renderables
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
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
        run_state: RunState::Running,
    };

    //Registering a components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();

    //Create map and get player location
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    //Build player
    gs.ecs
        .create_entity()
        .with(Position {x: player_x, y: player_y})
        .with(Player{})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, is_dirty: true})
        .with(Name{name: "Player".to_string()})
        .build();

    //Create test monsters
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let roll = rng.roll_dice(1, 2);
        let glyph : rltk::FontCharType;
        let name : String;
        match roll {
            1 => { glyph = rltk::to_cp437('k'); name = "Kobold".to_string(); },
            _ => { glyph = rltk::to_cp437('g'); name = "Goblin".to_string(); },
        };
        gs.ecs.create_entity()
            .with(Position{x, y})
            .with(Renderable{
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK), 
            })
            .with(Viewshed{visible_tiles: Vec::new(), range: 8, is_dirty: true})
            .with(Monster{})
            .with(BlocksTile{})
            .with(Name{name:format!("{} #{}", name, i)})
            .build();
    }

    //Insert resources into world
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    //Start game
    rltk::main_loop(context, gs)
}
