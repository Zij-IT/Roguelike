use rltk::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

//Components--
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: rltk::RGB,
    bg: rltk::RGB,
}

#[derive(Component)]
struct LeftWalker;
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftWalker>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData){
        for (_left, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
} 

#[derive(Component)]
struct Player;
//--Components

//Gamestate--
struct State {
    ecs: specs::prelude::World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    } 
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        //Scans player input and tries to move player
        player_input(self, ctx);

        //Runs all non-player systems (for now)
        self.run_systems();

        //Displays all entities with positions and renderables
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
//--Gamestate


fn main() -> rltk::BError {
    //Creating context
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    //Construct world
    let mut gs = State { 
        ecs: World::new(),
    };

    //Registering a component as it is not in a system
    gs.ecs.register::<Position>();
    gs.ecs.register::<LeftWalker>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    //Build player
    gs.ecs
        .create_entity()
        .with(Position {x: 40, y: 30 })
        .with(Player{})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build();

    //Build left walkers
    for i in 0..11 {
        gs.ecs
            .create_entity()
            .with(Position {x: i * 7, y: 25 })
            .with(Renderable {
                glyph: rltk::to_cp437('ë'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(LeftWalker{})
            .build();
    }
    rltk::main_loop(context, gs)
}

//Player System--
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_, pos) in (&mut players, &mut positions).join() {
        pos.x = std::cmp::min(79, std::cmp::max(0, pos.x+delta_x));
        pos.y = std::cmp::min(49, std::cmp::max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            _ => {},
        }
    }
}
//--Player System
