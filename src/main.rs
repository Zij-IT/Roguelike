use rltk::prelude::*;
use specs::prelude::*;

//All mods
mod map;
mod gui;
mod rect;
mod player;
mod gamelog;
mod components;
mod damage_system;
mod visibility_system;
mod monster_ai_system;
mod melee_combat_system;
mod map_indexing_system;

use map::*;
use player::*;
use gamelog::*;
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
    let mut gs = State { 
        ecs: World::new(),
    };

    //Registering a components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<WantsToMelee>();

    //Create map and get player location
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    //Build player
    let player_ent = gs.ecs
        .create_entity()
        .with(Position {x: player_x, y: player_y})
        .with(Player{})
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Viewshed{visible_tiles: Vec::new(), range: 8, is_dirty: true})
        .with(Name{name: "Player".to_string()})
        .with(CombatStats{max_hp: 30, hp: 30, defense: 2, power: 5})
        .build();

    //Create test monsters
    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let roll = rng.roll_dice(1, 2);
        let glyph : FontCharType;
        let name : String;
        match roll {
            1 => { glyph = to_cp437('k'); name = "Kobold".to_string(); },
            _ => { glyph = to_cp437('g'); name = "Goblin".to_string(); },
        };
        gs.ecs.create_entity()
            .with(Position{x, y})
            .with(Renderable{
                glyph,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK), 
            })
            .with(Viewshed{visible_tiles: Vec::new(), range: 8, is_dirty: true})
            .with(Monster{})
            .with(BlocksTile{})
            .with(Name{name:format!("{} #{}", name, i)})
            .with(CombatStats{max_hp: 16, hp: 16, defense: 1, power: 4})
            .build();
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
