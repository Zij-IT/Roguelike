use super::{
    camera, constants::colors, rex_assets, EcsWorld, Equipped, InBackpack, Name, RunState, Viewshed, constants::consoles
};
use rltk::{Point, Rltk, VirtualKeyCode, RGB, DrawBatch, xp_to_console};
use specs::prelude::*;

pub fn draw_hud(ecs: &World, ctx: &mut Rltk) {
    let assets = ecs.fetch::<rex_assets::RexAssets>();
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_ui, 0, 0);
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(gs: &mut EcsWorld, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
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

    //
    ctx.set_active_console(consoles::CHAR_CONSOLE);

    //Base locations
    let base_x = 17;
    let base_y = (25 - (relevant_entities.len() / 2)) as i32;

    //Draw UI BOX
    ctx.draw_box(
        base_x - 2,
        base_y - 2,
        31,
        (relevant_entities.len() + 3) as i32,
        RGB::from(colors::FOREGROUND),
        RGB::from(colors::BACKGROUND),
    );
    ctx.print_color(
        base_x + 1,
        base_y - 2,
        RGB::named(rltk::YELLOW),
        RGB::from(colors::BACKGROUND),
        match *current_state {
            RunState::ShowRemoveItem => "Remove What?",
            RunState::ShowDropItem => "Drop What?",
            RunState::ShowInventory => "Inventory",
            _ => unreachable!(),
        },
    );
    ctx.print_color(
        base_x + 1,
        base_y + relevant_entities.len() as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::from(colors::BACKGROUND),
        "ESC to cancel",
    );

    //Print out relevant items
    for (offset, (name, _)) in relevant_entities.iter().enumerate() {
        let y = base_y + offset as i32;
        ctx.set(
            base_x,
            y,
            RGB::from(colors::FOREGROUND),
            RGB::from(colors::BACKGROUND),
            rltk::to_cp437('('),
        );
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
        Some(VirtualKeyCode::Escape) => (ItemMenuResult::Cancel, None),
        Some(key) => {
            let selection = rltk::letter_to_option(key);
            if selection > -1 && selection < relevant_entities.len() as i32 {
                return (
                    ItemMenuResult::Selected,
                    Some(relevant_entities[selection as usize].1),
                );
            }
            (ItemMenuResult::NoResponse, None)
        }
        _ => (ItemMenuResult::NoResponse, None),
    }
}

pub fn show_targeting(
    gs: &mut EcsWorld,
    ctx: &mut Rltk,
    range: i32,
) -> (ItemMenuResult, Option<Point>) {
    let player_ent = gs.world.fetch::<Entity>();
    let player_pos = gs.world.fetch::<Point>();
    let views = gs.world.read_storage::<Viewshed>();
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(&gs.world);

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::from(colors::BACKGROUND),
        "Select Target: ",
    );

    let mut available_cells = Vec::new();
    if let Some(visible) = views.get(*player_ent) {
        for idx in visible.visible_tiles.iter() {
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
        return (ItemMenuResult::Cancel, None);
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
            (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_pos.0, mouse_pos.1)),
            )
        } else {
            ctx.set_bg(true_mouse_pos.0, true_mouse_pos.1, RGB::named(rltk::RED));
            (ItemMenuResult::Cancel, None)
        };
    }
    if let Some(VirtualKeyCode::Escape) = ctx.key {
        return (ItemMenuResult::Cancel, None);
    }

    (ItemMenuResult::NoResponse, None)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuSelection {
    NewGame = 0,
    LoadGame,
    Quit,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuResult {
    NoSelection(MainMenuSelection),
    Selection(MainMenuSelection),
}

pub fn draw_main_menu(gs: &mut EcsWorld, ctx: &mut Rltk) -> MainMenuResult {
    let assets = gs.world.fetch::<rex_assets::RexAssets>();
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.title_screen, 0, 0);

    if let RunState::MainMenu(current_selection) = *(gs.world.fetch::<RunState>()) {
        let selected = RGB::named(rltk::YELLOW);

        let main_menu_options = ["Begin New Game", "Load Game", "Quit"];

        let base_y = 45;
        let step = 2;

        for (index, option) in main_menu_options.iter().enumerate() {
            ctx.print_color_centered(
                base_y + step * index,
                if main_menu_options[(current_selection as usize)] == *option {
                    selected
                } else {
                    RGB::from(colors::FOREGROUND)
                },
                RGB::from(colors::BACKGROUND),
                option,
            );
        }

        return match ctx.key {
            Some(VirtualKeyCode::Return) => MainMenuResult::Selection(current_selection),
            Some(VirtualKeyCode::Up) => {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::Quit,
                    MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                    MainMenuSelection::Quit => MainMenuSelection::LoadGame,
                };
                MainMenuResult::NoSelection(new_selection)
            }
            Some(VirtualKeyCode::Down) => {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
                    MainMenuSelection::LoadGame => MainMenuSelection::Quit,
                    MainMenuSelection::Quit => MainMenuSelection::NewGame,
                };
                MainMenuResult::NoSelection(new_selection)
            }
            _ => MainMenuResult::NoSelection(current_selection),
        };
    }
    MainMenuResult::NoSelection(MainMenuSelection::NewGame)
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection,
    QuitToMenu,
}

pub fn show_game_over(ctx: &mut Rltk) -> GameOverResult {
    ctx.cls();

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
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
