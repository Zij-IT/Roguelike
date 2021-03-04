mod audio;

use crate::{
    constants::{colors, consoles},
    raws::config::CONFIGS,
    rex_assets,
    state::SettingsOption,
};
use rltk::{Rltk, RGB};
use strum::IntoEnumIterator;

pub fn show_settings_menu(
    ctx: &mut Rltk,
    current_state: SettingsOption,
    assets: &rex_assets::RexAssets,
) -> (SettingsOption, bool) {
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_settings, 0, 0);

    let yellow = RGB::named(rltk::YELLOW);
    let base_y = 3;
    let step = 2;

    for (index, option) in SettingsOption::iter().enumerate() {
        if option != SettingsOption::Back {
            ctx.print_color(
                2,
                base_y + step * index,
                if current_state == option {
                    yellow
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
            return (current_state, true);
        } else if key == keys.go_back {
            return (SettingsOption::Back, true);
        } else if key == keys.move_up {
            return match current_state {
                SettingsOption::Audio => (SettingsOption::Keybindings, false),
                SettingsOption::Visual => (SettingsOption::Audio, false),
                SettingsOption::Keybindings => (SettingsOption::Visual, false),
                SettingsOption::Back => unreachable!(),
            };
        } else if key == keys.move_down {
            return match current_state {
                SettingsOption::Audio => (SettingsOption::Visual, false),
                SettingsOption::Visual => (SettingsOption::Keybindings, false),
                SettingsOption::Keybindings => (SettingsOption::Audio, false),
                SettingsOption::Back => unreachable!(),
            };
        }
    }

    (current_state, false)
}
