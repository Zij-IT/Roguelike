use super::{CombatStats, GameLog, InBackpack, Name, Player, RunState, State, Viewshed};
use rltk::{Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
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
            RGB::named(rltk::BLACK),
            &health,
        );
        ctx.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );
        //Map Deets
        let depth = (*ecs.fetch::<super::Map>()).depth;
        ctx.print_color(
            2,
            43,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
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
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_ent);
    let count = inventory.count();
    let mut equippable = Vec::new();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        if *current_state != RunState::ShowDropItem {
            "Inventory"
        } else {
            "Drop What?"
        },
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    //Print out everything in inventory
    for (j, (_, name, entity)) in (&backpack, &names, &entities)
        .join()
        .filter(|item| item.0.owner == *player_ent)
        .enumerate()
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
    }

    match ctx.key {
        Some(VirtualKeyCode::Escape) => (ItemMenuResult::Cancel, None),
        Some(key) => {
            let selection = rltk::letter_to_option(key);
            if selection > -1 && selection < count as i32 {
                return (
                    ItemMenuResult::Selected,
                    Some(equippable[selection as usize]),
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
        RGB::named(rltk::BLACK),
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

    (ItemMenuResult::NoResponse, None)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuResult {
    NoSelection(MainMenuSelection),
    Selection(MainMenuSelection),
}

pub fn draw_main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    ctx.print_color_centered(
        15,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Bashing Bytes",
    );

    if let RunState::MainMenu(current_selection) = *(gs.ecs.fetch::<RunState>()) {
        let selected = RGB::named(rltk::MAGENTA);
        let not_selected = RGB::named(rltk::WHITE);
        let background = RGB::named(rltk::BLACK);

        ctx.print_color_centered(
            24,
            if current_selection == MainMenuSelection::NewGame {
                selected
            } else {
                not_selected
            },
            background,
            "Begin New Game",
        );

        ctx.print_color_centered(
            25,
            if current_selection == MainMenuSelection::LoadGame {
                selected
            } else {
                not_selected
            },
            background,
            "Load Game",
        );

        ctx.print_color_centered(
            26,
            if current_selection == MainMenuSelection::Quit {
                selected
            } else {
                not_selected
            },
            background,
            "Quit",
        );

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
