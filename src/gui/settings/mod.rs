mod keybindings;
mod audio;
mod visual;

use crate::{
    constants::{colors, consoles},
    raws::config::CONFIGS,
    rex_assets, RunState,
};
use rltk::{Rltk, RGB};
use specs::World;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

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

        let keys = &CONFIGS.lock().unwrap().keys;

        if let Some(key) = ctx.key {
            if key == keys.select {
                return SettingsMenuResult::Selection(current_selection);
            } else if key == keys.go_back {
                return SettingsMenuResult::Selection(SettingsSelection::Back);
            } else if key == keys.move_up {
                let new_selection = match current_selection {
                    SettingsSelection::Audio => SettingsSelection::Keybindings,
                    SettingsSelection::Visual => SettingsSelection::Audio,
                    SettingsSelection::Keybindings => SettingsSelection::Visual,
                    SettingsSelection::Back => unreachable!(),
                };
                return SettingsMenuResult::NoSelection(new_selection);
            } else if key == keys.move_down {
                let new_selection = match current_selection {
                    SettingsSelection::Audio => SettingsSelection::Visual,
                    SettingsSelection::Visual => SettingsSelection::Keybindings,
                    SettingsSelection::Keybindings => SettingsSelection::Audio,
                    SettingsSelection::Back => unreachable!(),
                };
                return SettingsMenuResult::NoSelection(new_selection);
            }
        }
        return SettingsMenuResult::NoSelection(current_selection);
    }

    SettingsMenuResult::NoSelection(SettingsSelection::iter().next().unwrap())
}
