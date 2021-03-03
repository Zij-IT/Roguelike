use crate::{
    constants::{colors, consoles},
    raws::config::CONFIGS,
    rex_assets::RexAssets,
    state::MainOption,
};
use rltk::{Rltk, RGB};
use strum::IntoEnumIterator;

pub fn show(ctx: &mut Rltk, current_state: MainOption, assets: &RexAssets) -> (MainOption, bool) {
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.title_screen, 0, 0);

    let yellow = RGB::named(rltk::YELLOW);

    let base_y = 45;
    let step = 2;

    for (index, option) in MainOption::iter().enumerate() {
        ctx.print_color_centered(
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

    let keys = &CONFIGS.lock().unwrap().keys;

    if let Some(key) = ctx.key {
        if key == keys.select {
            return (current_state, true);
        } else if key == keys.move_up {
            return match current_state {
                MainOption::NewGame => (MainOption::Quit, false),
                MainOption::LoadGame => (MainOption::NewGame, false),
                MainOption::Settings => (MainOption::LoadGame, false),
                MainOption::Quit => (MainOption::Settings, false),
            };
        } else if key == keys.move_down {
            return match current_state {
                MainOption::NewGame => (MainOption::LoadGame, false),
                MainOption::LoadGame => (MainOption::Settings, false),
                MainOption::Settings => (MainOption::Quit, false),
                MainOption::Quit => (MainOption::NewGame, false),
            };
        }
    }

    (current_state, false)
}
