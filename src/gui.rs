use super::{
    rex_assets, CombatStats, Equipped, GameLog, InBackpack, Name, Player, RunState, State, Viewshed,
};
use rltk::{Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

const BACKGROUND: &str = "#110016";
const FOREGROUND: &str = "#f3fbf1";

pub fn draw_ingame_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::from_hex(FOREGROUND).unwrap(),
        RGB::from_hex(BACKGROUND).unwrap(),
    );
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let mut log = ecs.write_resource::<GameLog>();
    for (_, stats) in (&players, &combat_stats).join() {
        let mut y = 44;
        for entry in log.entries.iter().rev() {
            if y < 49 {
                ctx.print(2, y, entry);
            }
            y += 1;
        }
        if log.entries.len() > 5 {
            let len = log.entries.len();
            log.entries.drain(0..len - 5);
        }
        let health = format!(" HP {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(
            12,
            43,
            RGB::named(rltk::YELLOW),
            RGB::from_hex(BACKGROUND).unwrap(),
            &health,
        );
        ctx.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::from_hex(BACKGROUND).unwrap(),
        );
        //Map Deets
        let depth = (*ecs.fetch::<super::Map>()).depth;
        ctx.print_color(
            2,
            43,
            RGB::named(rltk::YELLOW),
            RGB::from_hex(BACKGROUND).unwrap(),
            &format!("Depth: {}", depth),
        );
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_ent = gs.ecs.fetch::<Entity>();
    let current_state = gs.ecs.fetch::<RunState>();
    let names = gs.ecs.read_storage::<Name>();
    let entities = gs.ecs.entities();

    //Get all relevant items
    let relevant_ents = match *current_state {
        RunState::ShowRemoveItem => {
            let equipped_items = gs.ecs.read_storage::<Equipped>();
            (&equipped_items, &names, &entities)
                .join()
                .filter(|item| item.0.owner == *player_ent)
                .map(|item| (item.1, item.2))
                .collect::<Vec<_>>()
        }
        _ => {
            let backpack_items = gs.ecs.read_storage::<InBackpack>();
            (&backpack_items, &names, &entities)
                .join()
                .filter(|item| item.0.owner == *player_ent)
                .map(|item| (item.1, item.2))
                .collect::<Vec<_>>()
        }
    };

    //Base locations
    let base_x = 17;
    let base_y = (25 - (relevant_ents.len() / 2)) as i32;

    //Draw UI BOX
    ctx.draw_box(
        base_x - 2,
        base_y - 2,
        31,
        (relevant_ents.len() + 3) as i32,
        RGB::from_hex(FOREGROUND).unwrap(),
        RGB::from_hex(BACKGROUND).unwrap(),
    );
    ctx.print_color(
        base_x + 1,
        base_y - 2,
        RGB::named(rltk::YELLOW),
        RGB::from_hex(BACKGROUND).unwrap(),
        match *current_state {
            RunState::ShowRemoveItem => "Remove What?",
            RunState::ShowDropItem => "Drop What?",
            RunState::ShowInventory => "Inventory",
            _ => unreachable!(),
        },
    );
    ctx.print_color(
        base_x + 1,
        base_y + relevant_ents.len() as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::from_hex(BACKGROUND).unwrap(),
        "ESC to cancel",
    );

    //Print out relevant items
    for (offset, (name, _)) in relevant_ents.iter().enumerate() {
        let y = base_y + offset as i32;
        ctx.set(
            base_x,
            y,
            RGB::from_hex(FOREGROUND).unwrap(),
            RGB::from_hex(BACKGROUND).unwrap(),
            rltk::to_cp437('('),
        );
        ctx.set(
            base_x + 1,
            y,
            RGB::named(rltk::YELLOW),
            RGB::from_hex(BACKGROUND).unwrap(),
            97 + offset as rltk::FontCharType,
        );
        ctx.set(
            base_x + 2,
            y,
            RGB::from_hex(FOREGROUND).unwrap(),
            RGB::from_hex(BACKGROUND).unwrap(),
            rltk::to_cp437(')'),
        );
        ctx.print(base_x + 4, y, &name.name.to_string());
    }

    //Respond to players response
    match ctx.key {
        Some(VirtualKeyCode::Escape) => (ItemMenuResult::Cancel, None),
        Some(key) => {
            let selection = rltk::letter_to_option(key);
            if selection > -1 && selection < relevant_ents.len() as i32 {
                return (
                    ItemMenuResult::Selected,
                    Some(relevant_ents[selection as usize].1),
                );
            }
            (ItemMenuResult::NoResponse, None)
        }
        _ => (ItemMenuResult::NoResponse, None),
    }
}

pub fn draw_range(gs: &mut State, ctx: &mut Rltk, range: i32) -> (ItemMenuResult, Option<Point>) {
    let player_ent = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::from_hex(BACKGROUND).unwrap(),
        "Select Target: ",
    );

    let mut available_cells = Vec::new();
    if let Some(visible) = viewsheds.get(*player_ent) {
        for idx in visible.visible_tiles.iter() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance < range as f32 {
                ctx.set_bg(idx.x, idx.y, RGB::named(rltk::BLUE));
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    //Draw Cursor
    let mouse_pos = ctx.mouse_pos();
    if ctx.left_click {
        if available_cells
            .iter()
            .any(|tile| tile.x == mouse_pos.0 && tile.y == mouse_pos.1)
        {
            ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::CYAN));
            return (
                ItemMenuResult::Selected,
                Some(Point::new(mouse_pos.0, mouse_pos.1)),
            );
        } else {
            ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::RED));
            return (ItemMenuResult::Cancel, None);
        }
    }
    match ctx.key {
        Some(VirtualKeyCode::Escape) => return (ItemMenuResult::Cancel, None),
        _ => {}
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

pub fn draw_main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let assets = gs.ecs.fetch::<rex_assets::RexAssets>();
    ctx.render_xp_sprite(&assets.title_screen, 0, 0);

    if let RunState::MainMenu(current_selection) = *(gs.ecs.fetch::<RunState>()) {
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
                    RGB::from_hex(FOREGROUND).unwrap()
                },
                RGB::from_hex(BACKGROUND).unwrap(),
                option,
            );
        }

        match ctx.key {
            Some(VirtualKeyCode::Up) => {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::Quit,
                    MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                    MainMenuSelection::Quit => MainMenuSelection::LoadGame,
                };
                return MainMenuResult::NoSelection(new_selection);
            }
            Some(VirtualKeyCode::Down) => {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
                    MainMenuSelection::LoadGame => MainMenuSelection::Quit,
                    MainMenuSelection::Quit => MainMenuSelection::NewGame,
                };
                return MainMenuResult::NoSelection(new_selection);
            }
            Some(VirtualKeyCode::Return) => return MainMenuResult::Selection(current_selection),
            _ => return MainMenuResult::NoSelection(current_selection),
        }
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
            RGB::from_hex(FOREGROUND).unwrap(),
            RGB::from_hex(BACKGROUND).unwrap(),
            line,
        );
    }

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
