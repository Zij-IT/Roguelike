#![warn(
    clippy::perf,
    clippy::style,
    clippy::nursery,
    rust_2018_idioms,
    clippy::pedantic
)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::wildcard_imports,
    clippy::cast_precision_loss
)]

//External includes
use rltk::prelude::*;
use specs::{
    prelude::*,
    saveload::{SimpleMarker, SimpleMarkerAllocator},
};

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
mod state;

use constants::consoles;
use ecs::*;
use game_log::GameLog;
use gui::{
    inventory::{InvMode, InvResult},
    targeting::TargetResult,
};
use map_builder::map::Map;
use player::respond_to_input;
use state::{
    AudioOption, Gameplay,
    Gameplay::{AwaitingInput, PreRun},
    KeyBindingOption, MainOption, Menu, SettingsOption, State,
    State::Game,
    VisualOption,
};

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

//Main construct
pub struct BashingBytes {
    pub world: World,
}

impl BashingBytes {
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

        let mut fields_of_view = self.world.write_storage::<FieldOfView>();
        if let Some(fov) = fields_of_view.get_mut(*player_ent) {
            fov.is_dirty = true;
        }
    }

    fn calc_menu_state(&mut self, ctx: &mut Rltk, current_state: Menu) -> State {
        match current_state {
            Menu::Main(option) => {
                let main_menu_res = {
                    //Assets are fetched here to please the borrow checker!
                    let assets = self.world.fetch::<rex_assets::RexAssets>();
                    gui::main_menu::show(ctx, option, &*assets)
                };

                match main_menu_res {
                    (option, false) => State::Menu(Menu::Main(option)),
                    (option, true) => match option {
                        MainOption::NewGame => {
                            self.game_over_cleanup();
                            State::Game(PreRun)
                        }
                        MainOption::LoadGame => {
                            if save_load_util::does_save_exist() {
                                save_load_util::load_game(&mut self.world);
                                save_load_util::delete_save();
                                State::Game(AwaitingInput)
                            } else {
                                State::Menu(Menu::Main(MainOption::LoadGame))
                            }
                        }
                        MainOption::Settings => State::Menu(Menu::Settings(SettingsOption::Audio)),
                        MainOption::Quit => std::process::exit(0),
                    },
                }
            }
            Menu::Settings(option) => {
                let assets = &*self.world.fetch::<rex_assets::RexAssets>();
                match gui::settings::show_settings_menu(ctx, option, assets) {
                    (new_option, false) => State::Menu(Menu::Settings(new_option)),
                    (new_option, true) => match new_option {
                        SettingsOption::Audio => {
                            State::Menu(Menu::Audio(AudioOption::MasterVolume))
                        }
                        SettingsOption::Visual => {
                            State::Menu(Menu::Visual(VisualOption::FullScreen))
                        }
                        SettingsOption::Keybindings => {
                            State::Menu(Menu::Keybinding(KeyBindingOption::Up))
                        }
                        SettingsOption::Back => State::Menu(Menu::Main(MainOption::Settings)),
                    },
                }
            }
            Menu::Audio(_option) => todo!(),
            Menu::Visual(_option) => todo!(),
            Menu::Keybinding(_option) => todo!(),
        }
    }

    fn calc_game_state(&mut self, ctx: &mut Rltk, current_state: Gameplay) -> State {
        match current_state {
            Gameplay::PreRun => {
                ecs::pre_run_systems::execute(&mut self.world);
                Game(Gameplay::AwaitingInput)
            }
            Gameplay::AwaitingInput => Game(respond_to_input(self, ctx)),
            Gameplay::PlayerTurn => {
                ecs::all_systems::execute(&mut self.world);
                Game(Gameplay::MonsterTurn)
            }
            Gameplay::MonsterTurn => {
                ecs::all_systems::execute(&mut self.world);
                Game(Gameplay::AwaitingInput)
            }
            Gameplay::Inventory(mode) => match gui::inventory::show(&mut self.world, ctx) {
                InvResult::Cancel => Game(Gameplay::AwaitingInput),
                InvResult::NoResponse => Game(current_state),
                InvResult::Selected(item) => match mode {
                    InvMode::Use => self.world.read_storage::<Range>().get(item).map_or_else(
                        || {
                            let mut intent = self.world.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.world.fetch::<Entity>(),
                                    WantsToUseItem { item, target: None },
                                )
                                .expect("Unable to insert intent");
                            Game(Gameplay::PlayerTurn)
                        },
                        |range| Game(Gameplay::ShowTargeting(range.range, item)),
                    ),
                    InvMode::Drop => {
                        let mut intent = self.world.write_storage::<WantsToDropItem>();
                        intent
                            .insert(*self.world.fetch::<Entity>(), WantsToDropItem { item })
                            .expect("Unable to insert intent to drop item");
                        Game(Gameplay::PlayerTurn)
                    }
                    InvMode::Remove => {
                        let mut intent = self.world.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(*self.world.fetch::<Entity>(), WantsToRemoveItem { item })
                            .expect("Unable to insert intent to remove item");
                        Game(Gameplay::PlayerTurn)
                    }
                },
            },
            Gameplay::NextLevel => {
                self.goto_next_level();
                Game(Gameplay::PreRun)
            }
            Gameplay::SaveGame => {
                save_load_util::save_game(&mut self.world);
                State::Menu(Menu::Main(MainOption::LoadGame))
            }
            Gameplay::GameOver => {
                if gui::game_over::show(ctx) {
                    Game(current_state)
                } else {
                    self.game_over_cleanup();
                    State::Menu(Menu::Main(MainOption::NewGame))
                }
            }
            Gameplay::ShowTargeting(range, item) => {
                match gui::targeting::show(&self.world, ctx, range) {
                    TargetResult::NoResponse => Game(current_state),
                    TargetResult::Cancel => Game(Gameplay::AwaitingInput),
                    TargetResult::Selected(target) => {
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
                        Game(Gameplay::PlayerTurn)
                    }
                }
            }
        }
    }
}

impl GameState for BashingBytes {
    fn tick(&mut self, ctx: &mut Rltk) {
        for i in 0..consoles::NUM_OF_CONSOLES {
            ctx.set_active_console(i);
            ctx.cls();
        }

        let current_state = *self.world.fetch::<State>();

        let next_state: State = match current_state {
            State::Menu(menu) => self.calc_menu_state(ctx, menu),
            State::Game(game) => {
                gui::hud::show(&self.world, ctx);
                camera::render(&self.world, ctx);

                ecs::cull_dead_particles(&mut self.world, ctx.frame_time_ms);

                let state = self.calc_game_state(ctx, game);

                ecs::cull_dead_characters(&mut self.world);

                state
            }
        };

        //Replace State with the new one
        self.world.insert::<State>(next_state);
    }
}

rltk::embedded_resource!(GAME_FONT, "../resources/cp437_8x8.png");

fn main() -> BError {
    //Load Configurations for the game
    if raws::config::load().is_err() {
        //Let player know that the config file wasn't able to be read, and that the defaults
        //will be used
        todo!()
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
    let mut bashing_bytes = BashingBytes {
        world: World::new(),
    };

    //Register the components
    //gs.ecs must be first, otherwise irrelevant
    register_all!(
        bashing_bytes.world,
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
        SimpleMarker<SerializeMe>,
        SufferDamage,
        FieldOfView,
        WantsToDropItem,
        WantsToMelee,
        WantsToPickupItem,
        WantsToRemoveItem,
        WantsToUseItem,
    );

    //Load all that data driven design goodness
    raws::spawn::load();

    //DEPENDENCIES:
    //player -> SimpleMarkerAllocator
    insert_all!(
        bashing_bytes.world,
        State::Menu(Menu::Main(MainOption::NewGame)),
        SimpleMarkerAllocator::<SerializeMe>::new(),
        rex_assets::RexAssets::new(),
        ecs::ParticleBuilder::new(),
        GameLog::new(),
    );

    //Unable to include this statement in the above batch due to the borrow checker
    //Reason: Both world::insert and spawn_player both borrow world.world mutably
    let player_ent = spawning::spawn_player(&mut bashing_bytes.world, 0, 0);
    insert_all!(bashing_bytes.world, player_ent);

    //Generate map
    bashing_bytes.generate_world_map(1);

    //Start game
    main_loop(context, bashing_bytes)
}
