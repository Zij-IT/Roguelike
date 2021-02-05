use rltk::prelude::*;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

//Macros
macro_rules! register_all {
    ($world:expr, $($component:ty),* $(,)*) => {
        {
            $($world.register::<$component>();)*
        }
    };
}

//All mods
mod components;
mod gamelog;
mod gui;
mod map_builder;
mod player;
mod random_table;
mod rex_assets;
mod spawner;
mod systems;

use components::*;
use gamelog::*;
use map_builder::*;
use player::*;
use random_table::*;
use spawner::*;
use systems::*;

//Enums
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    AwaitingInput,
    GameOver,
    MainMenu(gui::MainMenuSelection),
    MonsterTurn,
    NextLevel,
    PlayerTurn,
    PreRun,
    SaveGame,
    ShowDropItem,
    ShowInventory,
    ShowRemoveItem,
    ShowTargeting(i32, Entity),
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        let mut mons = MonsterAI {};
        let mut mapindex = MapIndexingSystem {};
        let mut melee = MeleeCombatSystem {};
        let mut damage = DamageSystem {};
        let mut pickup_items = ItemCollectionSystem {};
        let mut use_items = ItemUseSystem {};
        let mut drop_items = ItemDropSystem {};
        let mut rem_items = ItemRemoveSystem {};
        let mut particles = ParticleSpawnSystem {};

        vis.run_now(&self.ecs);
        mons.run_now(&self.ecs);
        mapindex.run_now(&self.ecs);
        melee.run_now(&self.ecs);
        damage.run_now(&self.ecs);
        pickup_items.run_now(&self.ecs);
        use_items.run_now(&self.ecs);
        drop_items.run_now(&self.ecs);
        rem_items.run_now(&self.ecs);
        particles.run_now(&self.ecs);

        self.ecs.maintain();
    }

    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player_ent = self.ecs.fetch::<Entity>();
        let backpack = self.ecs.read_storage::<InBackpack>();
        let equipped_items = self.ecs.read_storage::<Equipped>();

        let mut to_delete = entities.join().collect::<Vec<_>>();
        to_delete.retain(|ent| {
            let is_player = *ent == *player_ent;
            let is_in_player_bag = {
                if let Some(pack) = backpack.get(*ent) {
                    pack.owner == *player_ent
                } else {
                    false
                }
            };
            let is_equipped_by_player = {
                if let Some(eq) = equipped_items.get(*ent) {
                    eq.owner == *player_ent
                } else {
                    false
                }
            };
            !is_player && !is_in_player_bag && !is_equipped_by_player
        });

        to_delete
    }

    fn goto_next_level(&mut self) {
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.ecs
                .delete_entity(target)
                .expect("Unable to delete entity during level transition");
        }

        //Build new map and place player
        let mut builder;
        let player_pos;
        {
            let mut world_map_res = self.ecs.write_resource::<Map>();
            builder = random_builder(world_map_res.depth + 1);
            builder.build_map();
            *world_map_res = builder.get_map();
            player_pos = builder.get_starting_position();
        }

        //Spawn baddies
        builder.spawn_entities(&mut self.ecs);

        //Change player position
        let mut player_point = self.ecs.write_resource::<Point>();
        *player_point = Point::new(player_pos.x, player_pos.y);

        //Change player position comp
        let mut pos_comps = self.ecs.write_storage::<Position>();
        let player_ent = self.ecs.fetch::<Entity>();
        if let Some(pos_comp) = pos_comps.get_mut(*player_ent) {
            pos_comp.x = player_pos.x;
            pos_comp.y = player_pos.y;
        }

        //Dirty players viewshed
        let mut viewsheds = self.ecs.write_storage::<Viewshed>();
        if let Some(vs) = viewsheds.get_mut(*player_ent) {
            vs.is_dirty = true;
        }

        //Notify player
        let mut logs = self.ecs.fetch_mut::<gamelog::GameLog>();
        logs.entries
            .push("You descend to the next level.".to_string());
        let mut all_stats = self.ecs.write_storage::<CombatStats>();
        if let Some(player_stats) = all_stats.get_mut(*player_ent) {
            player_stats.hp = i32::max(player_stats.hp, player_stats.max_hp / 2);
        }
    }

    fn game_over_cleanup(&mut self) {
        let to_delete = self.ecs.entities().join().collect::<Vec<_>>();
        for ent in to_delete.iter() {
            self.ecs.delete_entity(*ent).expect("Deletion failed");
        }

        let mut builder;
        let player_pos;
        {
            let mut world_map_res = self.ecs.write_resource::<Map>();
            builder = random_builder(world_map_res.depth + 1);
            builder.build_map();
            *world_map_res = builder.get_map();
            player_pos = builder.get_starting_position();
        }

        //Spawn baddies
        builder.spawn_entities(&mut self.ecs);

        //Add starting message
        let mut logs = self.ecs.write_resource::<GameLog>();
        logs.entries.clear();
        logs.entries.push("Welcome to my Roguelike!".to_string());
        std::mem::drop(logs);

        //Restart World
        let new_player_ent = spawner::spawn_player(&mut self.ecs, player_pos.x, player_pos.y);

        //Update player resources
        self.ecs.insert(new_player_ent);
        self.ecs.insert(Point::new(player_pos.x, player_pos.y));

        //Update player movement comp
        let mut position_components = self.ecs.write_storage::<Position>();
        if let Some(player_pos_comp) = position_components.get_mut(new_player_ent) {
            player_pos_comp.x = player_pos.x;
            player_pos_comp.y = player_pos.y;
        }

        //Dirty players viewshed
        let mut viewsheds = self.ecs.write_storage::<Viewshed>();
        if let Some(vs) = viewsheds.get_mut(new_player_ent) {
            vs.is_dirty = true;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        particle_system::cull_dead_particles(&mut self.ecs, ctx);

        let mut next_state = *self.ecs.fetch::<RunState>();
        //Draw map & entities
        match next_state {
            RunState::MainMenu(_) => {}
            _ => {
                draw_map(&self.ecs, ctx);
                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();
                    let map = self.ecs.fetch::<Map>();
                    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
                    for (pos, render) in data.iter() {
                        let idx = map.xy_idx(pos.x, pos.y);
                        if map.is_tile_status_set(idx, TILE_VISIBLE) {
                            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                        }
                    }
                }

                //GUI
                gui::draw_ui(&self.ecs, ctx);
            }
        }

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
            RunState::ShowInventory => {
                let (item_res, selected_item) = gui::show_inventory(self, ctx);
                match item_res {
                    gui::ItemMenuResult::Selected => {
                        let selected_item = selected_item.unwrap();
                        if let Some(range) = self.ecs.read_storage::<Ranged>().get(selected_item) {
                            next_state = RunState::ShowTargeting(range.range, selected_item);
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: selected_item,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            next_state = RunState::PlayerTurn;
                        }
                    }
                    gui::ItemMenuResult::Cancel => next_state = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                }
            }
            RunState::ShowDropItem => {
                let (item_res, selected_item) = gui::show_inventory(self, ctx);
                match item_res {
                    gui::ItemMenuResult::Selected => {
                        let selected_item = selected_item.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToDropItem {
                                    item: selected_item,
                                },
                            )
                            .expect("Unable to insert intent to drop item");
                        next_state = RunState::PlayerTurn;
                    }
                    gui::ItemMenuResult::Cancel => next_state = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                }
            }
            RunState::ShowRemoveItem => {
                let (item_res, selected_item) = gui::show_remove_inventory(self, ctx);
                match item_res {
                    gui::ItemMenuResult::Selected => {
                        let selected_item = selected_item.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToRemoveItem {
                                    item: selected_item,
                                },
                            )
                            .expect("Unable to insert intent to remove item");
                        next_state = RunState::PlayerTurn;
                    }
                    gui::ItemMenuResult::Cancel => next_state = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                }
            }
            RunState::ShowTargeting(range, item) => {
                let (item_res, target) = gui::draw_range(self, ctx, range);
                match item_res {
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent
                            .insert(*self.ecs.fetch::<Entity>(), WantsToUseItem { item, target })
                            .expect("Unable to insert intent");
                        next_state = RunState::PlayerTurn;
                    }
                    gui::ItemMenuResult::Cancel => next_state = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                }
            }
            RunState::MainMenu(_) => match gui::draw_main_menu(self, ctx) {
                gui::MainMenuResult::NoSelection(prev_option) => {
                    next_state = RunState::MainMenu(prev_option)
                }
                gui::MainMenuResult::Selection(option) => match option {
                    gui::MainMenuSelection::NewGame => next_state = RunState::PreRun,
                    gui::MainMenuSelection::LoadGame => {
                        if saveload_system::does_save_exist() {
                            saveload_system::load_game(&mut self.ecs);
                            next_state = RunState::AwaitingInput;
                            saveload_system::delete_save();
                        } else {
                            next_state = RunState::MainMenu(option);
                        }
                    }
                    gui::MainMenuSelection::Quit => std::process::exit(0),
                },
            },
            RunState::GameOver => {
                let result = gui::show_game_over(ctx);
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        next_state = RunState::MainMenu(gui::MainMenuSelection::NewGame);
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                next_state = RunState::MainMenu(gui::MainMenuSelection::LoadGame);
            }
            RunState::NextLevel => {
                self.goto_next_level();
                next_state = RunState::PreRun;
            }
        }

        //Replace RunState with the new one
        self.ecs.insert::<RunState>(next_state);
        DamageSystem::delete_the_dead(&mut self.ecs);
    }
}

fn main() -> BError {
    let context = RltkBuilder::simple(80, 60)
        .unwrap()
        .with_title("Bashing Bytes")
        .with_fullscreen(true)
        .build()?;

    //Construct world
    let mut gs = State { ecs: World::new() };

    //Register the components
    register_all!(
        gs.ecs,
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
        Ranged,
        Renderable,
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

    //Insert all non-entity related resources
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    gs.ecs.insert(rex_assets::RexAssets::new());
    gs.ecs
        .insert(RunState::MainMenu(gui::MainMenuSelection::NewGame));
    gs.ecs.insert(GameLog {
        entries: vec!["Welcome to my roguelike".to_string()],
    });
    gs.ecs.insert(particle_system::ParticleBuilder::new());

    let mut builder = random_builder(1);
    builder.build_map();
    let map = builder.get_map();
    let player_pos = builder.get_starting_position();

    //Spawn baddies
    builder.spawn_entities(&mut gs.ecs);

    //Create player and get location
    let player_ent = spawn_player(&mut gs.ecs, player_pos.x, player_pos.y);

    //Insert entity related resources to insert into world
    gs.ecs.insert(map);
    gs.ecs.insert(player_ent);
    gs.ecs.insert(Point::new(player_pos.x, player_pos.y));

    //Start game
    main_loop(context, gs)
}
