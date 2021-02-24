#![warn(clippy::perf, clippy::style, clippy::nursery, rust_2018_idioms)]
//I would also use clippy::pedantic, but I convert between usize and i32 so much that 90+ warnings
//were enough to make me not. I cleaned the large majority of the non-conversion errors though

//External includes
use rltk::prelude::*;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

//Internal mods and includes
mod camera;
mod constants;
mod ecs;
mod game_log;
mod gui;
mod map_builder;
mod player;
mod raws;
mod rex_assets;
mod save_load_util;
mod spawning;

use crate::gui::{InventoryMode, ItemMenuResult};
use constants::consoles;
use ecs::*;
use game_log::GameLog;
use gui::{MainMenuResult, MainMenuSelection, SettingsMenuResult, SettingsSelection};
use map_builder::map::Map;
use player::respond_to_input;

//Macros
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

//Constants
const MAP_HEIGHT: i32 = 64;
const MAP_WIDTH: i32 = 64;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    GameOver,
    MainMenu(gui::MainMenuSelection),
    SettingsMenu(gui::SettingsSelection),
    MonsterTurn,
    NextLevel,
    PlayerTurn,
    PreRun,
    SaveGame,
    Inventory(gui::InventoryMode),
    ShowTargeting(i32, Entity),
}

//Main construct
pub struct EcsWorld {
    pub world: World,
}

impl EcsWorld {
    ///Gathers all entities that are not related to the player
    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.world.entities();
        let player_ent = self.world.fetch::<Entity>();
        let backpack = self.world.read_storage::<InBackpack>();
        let equipped_items = self.world.read_storage::<Equipped>();

        let mut to_delete = entities.join().collect::<Vec<_>>();
        to_delete.retain(|ent| {
            let is_player = *ent == *player_ent;
            let is_in_player_bag = backpack
                .get(*ent)
                .map_or(false, |pack| pack.owner == *player_ent);
            let is_equipped_by_player = equipped_items
                .get(*ent)
                .map_or(false, |eq| eq.owner == *player_ent);
            !is_player && !is_in_player_bag && !is_equipped_by_player
        });

        to_delete
    }

    ///Generates next level for the player to explore
    fn goto_next_level(&mut self) {
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.world
                .delete_entity(target)
                .expect("Unable to delete entity during level transition");
        }
        self.world.maintain();

        //Build new map and place player
        let current_depth = self.world.fetch::<Map>().depth;
        self.generate_world_map(current_depth + 1);

        //Notify player and heal player
        self.world
            .fetch_mut::<GameLog>()
            .push(&"You descend to the next level.");
        let player_ent = self.world.fetch::<Entity>();
        let mut all_stats = self.world.write_storage::<CombatStats>();
        if let Some(player_stats) = all_stats.get_mut(*player_ent) {
            player_stats.hp = i32::max(player_stats.hp, player_stats.max_hp / 2);
        }
    }

    ///Deletes all entities, and sets up for next game
    fn game_over_cleanup(&mut self) {
        self.world.delete_all();
        self.world.maintain();

        //Add starting message
        let mut logs = self.world.write_resource::<GameLog>();
        logs.clear();
        logs.push(&"Welcome to my Roguelike!");
        std::mem::drop(logs);

        //Create new player resource
        let player_ent = spawning::spawn_player(&mut self.world, 0, 0);
        self.world.insert(player_ent);
        self.world.insert(Point::new(0, 0));

        //Build a new map and place player
        self.generate_world_map(1);
    }

    ///Generates a new level using `random_builder` with the specified depth
    fn generate_world_map(&mut self, new_depth: i32) {
        let mut builder = map_builder::random_builder(MAP_WIDTH, MAP_HEIGHT, new_depth);
        builder.build_map();
        self.world.insert(builder.get_map());
        builder.spawn_entities(&mut self.world);

        //Updates the players position based on the new map generated
        //Also must update the player component, and the player pos resource
        let Position {
            x: player_x,
            y: player_y,
        } = builder.get_starting_position();
        self.world.insert(Point::new(player_x, player_y));

        let mut position_components = self.world.write_storage::<Position>();
        let player_ent = self.world.fetch::<Entity>();
        if let Some(player_pos_comp) = position_components.get_mut(*player_ent) {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        let mut viewsheds = self.world.write_storage::<Viewshed>();
        if let Some(vs) = viewsheds.get_mut(*player_ent) {
            vs.is_dirty = true;
        }
    }
}

impl GameState for EcsWorld {
    fn tick(&mut self, ctx: &mut Rltk) {
        for i in 0..consoles::NUM_OF_CONSOLES {
            ctx.set_active_console(i);
            ctx.cls();
        }

        ecs::cull_dead_particles(&mut self.world, ctx.frame_time_ms);
        let mut next_state = *self.world.fetch::<RunState>();

        //Draw map & characters
        if !matches!(next_state, RunState::MainMenu(_))
            && !matches!(next_state, RunState::SettingsMenu(_))
        {
            gui::show_hud(&self.world, ctx);
            camera::render(&self.world, ctx);
        }

        //Calculates next state based on current state
        match next_state {
            RunState::PreRun => {
                ecs::pre_run_systems::execute(&mut self.world);
                next_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                next_state = respond_to_input(self, ctx);
            }
            RunState::PlayerTurn => {
                ecs::all_systems::execute(&mut self.world);
                next_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                ecs::all_systems::execute(&mut self.world);
                next_state = RunState::AwaitingInput;
            }
            RunState::SaveGame => {
                save_load_util::save_game(&mut self.world);
                next_state = RunState::MainMenu(gui::MainMenuSelection::LoadGame);
            }
            RunState::NextLevel => {
                self.goto_next_level();
                next_state = RunState::PreRun;
            }
            RunState::Inventory(mode) => match gui::show_inventory(&mut self.world, ctx) {
                ItemMenuResult::Cancel => next_state = RunState::AwaitingInput,
                ItemMenuResult::NoResponse => {}
                ItemMenuResult::Selected(item) => match mode {
                    InventoryMode::Use => {
                        if let Some(range) = self.world.read_storage::<Range>().get(item) {
                            next_state = RunState::ShowTargeting(range.range, item);
                        } else {
                            let mut intent = self.world.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.world.fetch::<Entity>(),
                                    WantsToUseItem { item, target: None },
                                )
                                .expect("Unable to insert intent");
                            next_state = RunState::PlayerTurn;
                        }
                    }
                    InventoryMode::Drop => {
                        let mut intent = self.world.write_storage::<WantsToDropItem>();
                        intent
                            .insert(*self.world.fetch::<Entity>(), WantsToDropItem { item })
                            .expect("Unable to insert intent to drop item");
                        next_state = RunState::PlayerTurn;
                    }
                    InventoryMode::Remove => {
                        let mut intent = self.world.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(*self.world.fetch::<Entity>(), WantsToRemoveItem { item })
                            .expect("Unable to insert intent to remove item");
                        next_state = RunState::PlayerTurn;
                    }
                },
            },
            RunState::ShowTargeting(range, item) => match gui::show_targeting(self, ctx, range) {
                gui::TargetResult::Selected(target) => {
                    let mut intent = self.world.write_storage::<WantsToUseItem>();
                    intent
                        .insert(
                            *self.world.fetch::<Entity>(),
                            WantsToUseItem {
                                item,
                                target: Some(target),
                            },
                        )
                        .expect("Unable to insert intent");
                    next_state = RunState::PlayerTurn;
                }
                gui::TargetResult::Cancel => next_state = RunState::AwaitingInput,
                gui::TargetResult::NoResponse => {}
            },
            RunState::MainMenu(_) => match gui::show_main_menu(&mut self.world, ctx) {
                MainMenuResult::NoSelection(option) => next_state = RunState::MainMenu(option),
                MainMenuResult::Selection(option) => match option {
                    MainMenuSelection::NewGame => {
                        self.game_over_cleanup();
                        next_state = RunState::PreRun;
                    }
                    MainMenuSelection::LoadGame => {
                        if save_load_util::does_save_exist() {
                            save_load_util::load_game(&mut self.world);
                            next_state = RunState::AwaitingInput;
                            save_load_util::delete_save();
                        } else {
                            next_state = RunState::MainMenu(option);
                        }
                    }
                    MainMenuSelection::Settings => {
                        next_state = RunState::SettingsMenu(SettingsSelection::Audio)
                    }
                    MainMenuSelection::Quit => std::process::exit(0),
                },
            },
            RunState::SettingsMenu(_) => match gui::show_settings_menu(&mut self.world, ctx) {
                SettingsMenuResult::NoSelection(option) => {
                    next_state = RunState::SettingsMenu(option)
                }
                SettingsMenuResult::Selection(option) => match option {
                    SettingsSelection::Audio => {
                        todo!()
                    }
                    SettingsSelection::Visual => {
                        todo!()
                    }
                    SettingsSelection::Keybindings => {
                        todo!()
                    }
                    SettingsSelection::Back => {
                        next_state = RunState::MainMenu(MainMenuSelection::Settings)
                    }
                },
            },
            RunState::GameOver => {
                if gui::show_game_over(ctx) == gui::GameOverResult::QuitToMenu {
                    self.game_over_cleanup();
                    next_state = RunState::MainMenu(gui::MainMenuSelection::NewGame);
                }
            }
        }

        //Replace RunState with the new one
        self.world.insert::<RunState>(next_state);
        ecs::cull_dead_characters(&mut self.world);
    }
}

rltk::embedded_resource!(GAME_FONT, "../resources/cp437_8x8.png");

fn main() -> BError {
    //Load Configurations for the game
    if raws::config::load().is_err() {
        //Let player know that the config file wasn't able to be read, and that the defaults
        //will be used
    }

    let full_screen = raws::config::CONFIGS.lock().unwrap().visuals.full_screen;

    //Create RltkBuilder
    rltk::link_resource!(GAME_FONT, "/resources/cp437_8x8.png");
    let context = RltkBuilder::new()
        .with_title("Bashing Bytes")
        .with_font("cp437_8x8.png", 8, 8)
        .with_fullscreen(full_screen)
        .with_dimensions(80, 60)
        .with_simple_console(80, 60, "cp437_8x8.png") // map
        .with_simple_console_no_bg(80, 60, "cp437_8x8.png") // creatures
        .with_sparse_console(80, 60, "cp437_8x8.png") // hud
        .with_tile_dimensions(8, 8)
        .build()?;

    //Build world
    let mut world = EcsWorld {
        world: World::new(),
    };

    //Register the components
    //gs.ecs must be first, otherwise irrelevant
    register_all!(
        world.world,
        AreaOfEffect,
        BlocksTile,
        CombatStats,
        Consumable,
        DefenseBonus,
        Equipable,
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
        Viewshed,
        WantsToDropItem,
        WantsToMelee,
        WantsToPickupItem,
        WantsToRemoveItem,
        WantsToUseItem,
    );

    //Load all that data driven design goodness
    raws::spawn::load();

    //gs.ecs must be first, otherwise follow the dependencies
    //DEPENDENCIES:
    //player -> SimpleMarkerAllocator
    insert_all!(
        world.world,
        RunState::MainMenu(gui::MainMenuSelection::NewGame),
        SimpleMarkerAllocator::<SerializeMe>::new(),
        rltk::RandomNumberGenerator::new(),
        rex_assets::RexAssets::new(),
        ecs::ParticleBuilder::new(),
        Map::new(1, 1, 1),
        Point::new(0, 0),
        GameLog::new(),
    );

    //Unable to include this statement in the above batch due to the borrow checker
    //Reason: Both world::insert and spawn_player both borrow world.world mutably
    let player_ent = spawning::spawn_player(&mut world.world, 0, 0);
    insert_all!(world.world, player_ent);

    //Generate map
    world.generate_world_map(1);

    //Start game
    main_loop(context, world)
}
