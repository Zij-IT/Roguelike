use super::{
    camera, constants::colors, constants::consoles, rex_assets, EcsWorld, Equipped, InBackpack,
    Name, RunState, Viewshed,
};
use crate::ecs::CombatStats;
use rltk::{Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::convert::AsRef;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

pub fn show_hud(ecs: &World, ctx: &mut Rltk) {
    let assets = ecs.fetch::<rex_assets::RexAssets>();
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_ui, 0, 0);

    //Show player health
    let player_entity = ecs.fetch::<Entity>();
    let combat_stats = ecs.read_component::<CombatStats>();

    if let Some(players_stats) = combat_stats.get(*player_entity) {
        let print_x = 62;
        let base_x = 68;
        let base_y = 1;
        //Show health
        ctx.print_color(
            print_x,
            base_y,
            RGB::named(colors::FOREGROUND),
            RGB::named(colors::BACKGROUND),
            format!("{}/{}", players_stats.hp, players_stats.max_hp),
        );

        //Show health bars
        let ratio = 10.0 * (players_stats.hp as f32 / players_stats.max_hp as f32);
        for i in 0..10 {
            let foreground = if i < (ratio as i32) {
                RGB::named(rltk::GREEN)
            } else {
                RGB::named(rltk::RED)
            };
            ctx.set(
                base_x + i,
                base_y,
                foreground,
                RGB::named(colors::BACKGROUND),
                61,
            );
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected(Entity),
}

pub fn show_inventory(gs: &mut EcsWorld, ctx: &mut Rltk) -> ItemMenuResult {
    let player_ent = gs.world.fetch::<Entity>();
    let current_state = gs.world.fetch::<RunState>();
    let names = gs.world.read_storage::<Name>();
    let entities = gs.world.entities();

    //Get all relevant items
    //Unable to simplify to avoid the duplication of the lines .join() .. .collect() because the
    //if arms are of different types.
    let relevant_entities = {
        if *current_state == RunState::ShowRemoveItem {
            let equipped_items = gs.world.read_storage::<Equipped>();
            (&equipped_items, &names, &entities)
                .join()
                .filter(|item| item.0.owner == *player_ent)
                .map(|item| (item.1, item.2))
                .collect::<Vec<_>>()
        } else {
            let backpack_items = gs.world.read_storage::<InBackpack>();
            (&backpack_items, &names, &entities)
                .join()
                .filter(|item| item.0.owner == *player_ent)
                .map(|item| (item.1, item.2))
                .collect::<Vec<_>>()
        }
    };

    ctx.set_active_console(consoles::HUD_CONSOLE);
    let assets = gs.world.fetch::<rex_assets::RexAssets>();
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
    match ctx.key {
        Some(VirtualKeyCode::Escape) => ItemMenuResult::Cancel,
        Some(key) => {
            let selection = rltk::letter_to_option(key);
            if selection > -1 && selection < relevant_entities.len() as i32 {
                return ItemMenuResult::Selected(relevant_entities[selection as usize].1);
            }
            ItemMenuResult::NoResponse
        }
        _ => ItemMenuResult::NoResponse,
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TargetResult {
    Cancel,
    NoResponse,
    Selected(Point),
}

pub fn show_targeting(gs: &mut EcsWorld, ctx: &mut Rltk, range: i32) -> TargetResult {
    let player_ent = gs.world.fetch::<Entity>();
    let player_pos = gs.world.fetch::<Point>();
    let views = gs.world.read_storage::<Viewshed>();
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(&gs.world);

    ctx.set_active_console(consoles::MAP_CONSOLE);

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::from(colors::BACKGROUND),
        "Select Target: ",
    );

    let mut available_cells = Vec::new();
    if let Some(visible) = views.get(*player_ent) {
        for idx in &visible.visible_tiles {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance < range as f32 {
                let screen_x = idx.x - min_x;
                let screen_y = idx.y - min_y;
                if screen_x > 1
                    && screen_x < max_x - min_x - 1
                    && screen_y > 1
                    && screen_y < max_y - min_y - 1
                {
                    ctx.set_bg(screen_x, screen_y, RGB::named(rltk::BLUE));
                    available_cells.push(idx);
                }
            }
        }
    } else {
        return TargetResult::Cancel;
    }

    //Draw Cursor
    let true_mouse_pos = ctx.mouse_pos();
    let mouse_pos = { (true_mouse_pos.0 + min_x, true_mouse_pos.1 + min_y) };
    if ctx.left_click {
        return if available_cells
            .iter()
            .any(|tile| tile.x == mouse_pos.0 && tile.y == mouse_pos.1)
        {
            ctx.set_bg(true_mouse_pos.0, true_mouse_pos.1, RGB::named(rltk::CYAN));
            TargetResult::Selected(Point::new(mouse_pos.0, mouse_pos.1))
        } else {
            ctx.set_bg(true_mouse_pos.0, true_mouse_pos.1, RGB::named(rltk::RED));
            TargetResult::Cancel
        };
    }
    if let Some(VirtualKeyCode::Escape) = ctx.key {
        return TargetResult::Cancel;
    }

    TargetResult::NoResponse
}

//Main Menu related
#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum MainMenuSelection {
    #[strum(serialize = "Start Anew")]
    NewGame,
    #[strum(serialize = "Continue")]
    LoadGame,
    Settings,
    Quit,
}

pub enum MainMenuResult {
    NoSelection(MainMenuSelection),
    Selection(MainMenuSelection),
}

pub fn show_main_menu(world: &mut World, ctx: &mut Rltk) -> MainMenuResult {
    if let RunState::MainMenu(current_selection) = *world.fetch::<RunState>() {
        let assets = world.fetch::<rex_assets::RexAssets>();
        ctx.set_active_console(consoles::HUD_CONSOLE);
        ctx.render_xp_sprite(&assets.title_screen, 0, 0);

        let selected = RGB::named(rltk::YELLOW);

        let base_y = 45;
        let step = 2;

        for (index, option) in MainMenuSelection::iter().enumerate() {
            ctx.print_color_centered(
                base_y + step * index,
                if current_selection == option {
                    selected
                } else {
                    RGB::from(colors::FOREGROUND)
                },
                RGB::from(colors::BACKGROUND),
                option.as_ref(),
            );
        }

        return match ctx.key {
            Some(VirtualKeyCode::Return) => MainMenuResult::Selection(current_selection),
            Some(VirtualKeyCode::Up) => {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::Quit,
                    MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                    MainMenuSelection::Settings => MainMenuSelection::LoadGame,
                    MainMenuSelection::Quit => MainMenuSelection::Settings,
                };
                MainMenuResult::NoSelection(new_selection)
            }
            Some(VirtualKeyCode::Down) => {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
                    MainMenuSelection::LoadGame => MainMenuSelection::Settings,
                    MainMenuSelection::Settings => MainMenuSelection::Quit,
                    MainMenuSelection::Quit => MainMenuSelection::NewGame,
                };
                MainMenuResult::NoSelection(new_selection)
            }
            _ => MainMenuResult::NoSelection(current_selection),
        };
    }

    MainMenuResult::NoSelection(MainMenuSelection::NewGame)
}

//Settings related
#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum SettingsSelection {
    Audio,
    Visual,
    Keybindings,
    Back,
}

pub enum SettingsMenuResult {
    NoSelection(SettingsSelection),
    Selection(SettingsSelection),
}

pub fn show_settings_menu(world: &mut World, ctx: &mut Rltk) -> SettingsMenuResult {
    if let RunState::SettingsMenu(current_selection) = *world.fetch::<RunState>() {
        let assets = world.fetch::<rex_assets::RexAssets>();
        ctx.set_active_console(consoles::HUD_CONSOLE);
        ctx.render_xp_sprite(&assets.blank_settings, 0, 0);

        let selected = RGB::named(rltk::YELLOW);

        let base_y = 3;
        let step = 2;

        for (index, option) in SettingsSelection::iter().enumerate() {
            if option != SettingsSelection::Back {
                ctx.print_color(
                    2,
                    base_y + step * index,
                    if current_selection == option {
                        selected
                    } else {
                        RGB::from(colors::FOREGROUND)
                    },
                    RGB::from(colors::BACKGROUND),
                    option.as_ref(),
                );
            }
        }

        return match ctx.key {
            Some(VirtualKeyCode::Return) => SettingsMenuResult::Selection(current_selection),
            Some(VirtualKeyCode::Down) => {
                let new_selection = match current_selection {
                    SettingsSelection::Audio => SettingsSelection::Visual,
                    SettingsSelection::Visual => SettingsSelection::Keybindings,
                    SettingsSelection::Keybindings => SettingsSelection::Accessibility,
                    SettingsSelection::Accessibility => SettingsSelection::Audio,
                    SettingsSelection::Back => unreachable!(),
                };
                SettingsMenuResult::NoSelection(new_selection)
            }
            Some(VirtualKeyCode::Up) => {
                let new_selection = match current_selection {
                    SettingsSelection::Audio => SettingsSelection::Accessibility,
                    SettingsSelection::Visual => SettingsSelection::Audio,
                    SettingsSelection::Keybindings => SettingsSelection::Visual,
                    SettingsSelection::Accessibility => SettingsSelection::Keybindings,
                    SettingsSelection::Back => unreachable!(),
                };
                SettingsMenuResult::NoSelection(new_selection)
            }
            Some(VirtualKeyCode::Escape) => SettingsMenuResult::Selection(SettingsSelection::Back),
            _ => SettingsMenuResult::NoSelection(current_selection),
        };
    }

    SettingsMenuResult::NoSelection(SettingsSelection::iter().next().unwrap())
}

//Game over related
#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoResponse,
    QuitToMenu,
}

pub fn show_game_over(ctx: &mut Rltk) -> GameOverResult {
    for i in 0..consoles::NUM_OF_CONSOLES {
        ctx.set_active_console(i);
        ctx.cls();
    }

    ctx.set_active_console(consoles::HUD_CONSOLE);

    let lines = [
        "Your journey has ended!",
        "One day, we'll tell you all about how you did.",
        "That day, sadly, is not in this chapter..",
        "Press any key to return to the menu.",
    ];

    let y_base = 15;
    let step = 2;
    for (index, line) in lines.iter().enumerate() {
        ctx.print_color_centered(
            y_base + step * index,
            RGB::from(colors::FOREGROUND),
            RGB::from(colors::BACKGROUND),
            line,
        );
    }

    match ctx.key {
        None => GameOverResult::NoResponse,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
