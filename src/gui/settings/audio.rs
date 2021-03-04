use crate::{
    state::{
        AudioOption,
    },
    raws::config::CONFIGS,
    rex_assets::RexAssets,
    constants::{consoles, colors},
};
use rltk::{Rltk, RGB};
use strum::IntoEnumIterator;

pub fn show(ctx: &mut Rltk, current_option: AudioOption, assets: &RexAssets) -> (AudioOption, bool){
    ctx.set_active_console(consoles::HUD_CONSOLE);
    //ctx.render_xp_sprite(&assets.blank_audio, 0, 0);

    let yellow = RGB::named(rltk::YELLOW);
    let base_y = 20;
    let step = 2;

    for (index, option) in AudioOption::iter().enumerate() {
        if option != AudioOption::Back {
            ctx.print_color(
                20,
                base_y + step * index,
                if current_option == option {
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
        return if key == keys.select {
            (current_option, true)
        } else if key == keys.go_back {
            (AudioOption::Back, true)
        } else if key == keys.move_up {
            match current_option {
                AudioOption::MasterVolume => (AudioOption::MusicVolume, false),
                AudioOption::MusicVolume => (AudioOption::SoundEffect, false),
                AudioOption::SoundEffect => (AudioOption::MasterVolume, false),
                AudioOption::Back => unreachable!(),
            }
        } else if key == keys.move_down {
            match current_option {
                AudioOption::MasterVolume => (AudioOption::SoundEffect, false),
                AudioOption::MusicVolume =>  (AudioOption::MasterVolume, false),
                AudioOption::SoundEffect => (AudioOption::MusicVolume, false),
                AudioOption::Back => unreachable!(),
            }
        } else {
            (current_option, false)
        };
    }

    (current_option, false)
}
