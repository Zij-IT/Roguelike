use crate::constants::{colors, consoles};
use crate::raws::config::CONFIGS;
use crate::{rex_assets, RunState};
use rltk::{Rltk, RGB};
use specs::World;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

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

        let keys = &CONFIGS.lock().unwrap().keys;

        if let Some(key) = ctx.key {
            return if key == keys.select {
                MainMenuResult::Selection(current_selection)
            } else if key == keys.move_up {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::Quit,
                    MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                    MainMenuSelection::Settings => MainMenuSelection::LoadGame,
                    MainMenuSelection::Quit => MainMenuSelection::Settings,
                };
                MainMenuResult::NoSelection(new_selection)
            } else if key == keys.move_down {
                let new_selection = match current_selection {
                    MainMenuSelection::NewGame => MainMenuSelection::LoadGame,
                    MainMenuSelection::LoadGame => MainMenuSelection::Settings,
                    MainMenuSelection::Settings => MainMenuSelection::Quit,
                    MainMenuSelection::Quit => MainMenuSelection::NewGame,
                };
                MainMenuResult::NoSelection(new_selection)
            } else {
                MainMenuResult::NoSelection(current_selection)
            };
        }
        return MainMenuResult::NoSelection(current_selection);
    }
    MainMenuResult::NoSelection(MainMenuSelection::iter().next().unwrap())
}
