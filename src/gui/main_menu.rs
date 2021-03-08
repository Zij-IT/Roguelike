use crate::{
    constants::{colors, consoles},
    raws::config::Config,
    rex_assets::RexAssets,
    state::MainOption,
};
use enum_cycling::IntoEnumCycle;
use rltk::{Rltk, RGB};
use strum::IntoEnumIterator;

pub fn show(
    configs: &Config,
    ctx: &mut Rltk,
    current_state: MainOption,
    assets: &RexAssets,
) -> (MainOption, bool) {
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

    let keys = &configs.keys;

    if let Some(key) = ctx.key {
        if key == keys.select {
            return (current_state, true);
        } else if key == keys.move_up {
            return (current_state.up(), false);
        } else if key == keys.move_down {
            return (current_state.down(), false);
        }
    }

    (current_state, false)
}
