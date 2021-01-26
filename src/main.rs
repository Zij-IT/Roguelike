use rltk::prelude::*;
use specs::prelude::*;

//All mods
mod map;
mod gui;
mod rect;
mod player;
mod gamelog;
mod spawner;
mod components;
mod damage_system;
mod visibility_system;
mod monster_ai_system;
mod melee_combat_system;
mod map_indexing_system;

use map::*;
use player::*;
use gamelog::*;
use spawner::*;
use components::*;
use damage_system::*;
use visibility_system::*;
use monster_ai_system::*;
use melee_combat_system::*;
use map_indexing_system::*;

//Enums
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut mons = MonsterAI{};
        let mut damage = DamageSystem{};
        let mut vis = VisibilitySystem{};
        let mut melee = MeleeCombatSystem{};
        let mut mapindex = MapIndexingSystem{};

        vis.run_now(&self.ecs);
        mons.run_now(&self.ecs);
        mapindex.run_now(&self.ecs);
        melee.run_now(&self.ecs);
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    } 
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        let mut next_state = *self.ecs.fetch::<RunState>();

        //FSM
        match next_state {
            RunState::PreRun => {
                self.run_systems();
                next_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                next_state = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                next_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                next_state = RunState::AwaitingInput;
            }
        }
        
        //If a resource of a similar type is added, it is overwritten => overwriting old value
        self.ecs.insert::<RunState>(next_state);

        DamageSystem::delete_the_dead(&mut self.ecs);

        //Draw map & entities
        draw_map(&self.ecs, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }

        //GUI
        gui::draw_ui(&self.ecs, ctx);
    }
}

//main
fn main() -> BError {
    //Creating context
    let context = RltkBuilder::simple80x50()
        .with_title("Bashing Bytes")
        .build()?;

    //Construct world
    let mut gs = State { ecs: World::new() };

    //Registering a components
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();

    //Create map and get player location
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    //Build player
    let player_ent = spawn_player(&mut gs.ecs, player_x, player_y); 

    //Create test monsters
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    for room in map.rooms.iter().skip(1) {
        populate_room(&mut gs.ecs, &room);
    }

    //Insert resources into world
    gs.ecs.insert(map);
    gs.ecs.insert(player_ent);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog{entries: vec!["Welcome to my roguelike".to_string()]});

    //Start game
    main_loop(context, gs)
}
