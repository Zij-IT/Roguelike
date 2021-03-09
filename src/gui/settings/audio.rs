use crate::{
    constants::{colors, consoles},
    raws::config::Config,
    rex_assets::RexAssets,
    state::AudioOption,
};
use enum_cycling::IntoEnumCycle;
use rltk::{Rltk, RGB};

pub fn show(
    configs: &mut Config,
    music_sink: &rodio::Sink,
    sfx_sink: &rodio::Sink,
    ctx: &mut Rltk,
    current_option: AudioOption,
    assets: &RexAssets,
) -> AudioOption {
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_audio, 0, 0);

    let yellow = RGB::named(rltk::YELLOW);
    let bg = RGB::from(colors::BACKGROUND);
    let opt = current_option.as_ref();

    match current_option {
        AudioOption::MasterVolume => ctx.print_color(26, 5, yellow, bg, opt),
        AudioOption::MusicVolume => ctx.print_color(26, 7, yellow, bg, opt),
        AudioOption::SfxVolume => ctx.print_color(26, 9, yellow, bg, opt),
        AudioOption::Back => (),
    }

    let audio = &mut configs.audio;

    for i in 0..audio.master_volume {
        ctx.set(41 + i, 5, RGB::named((0, 0, 255)), bg, 254);
    }

    for i in 0..audio.music_volume {
        ctx.set(41 + i, 7, RGB::named((0, 0, 255)), bg, 254);
    }

    for i in 0..audio.sfx_volume {
        ctx.set(41 + i, 9, RGB::named((0, 0, 255)), bg, 254);
    }

    let keys = &configs.keys;
    if let Some(key) = ctx.key {
        if key == keys.go_back {
            return AudioOption::Back;
        } else if key == keys.move_up {
            return current_option.up();
        } else if key == keys.move_down {
            return current_option.down();
        } else if key == keys.move_left {
            match current_option {
                AudioOption::MasterVolume => {
                    if audio.master_volume > 0 {
                        audio.master_volume -= 1;
                        let master_ratio = audio.master_volume as f32 / 25.0;
                        let music_ratio = audio.music_volume as f32 / 25.0 * master_ratio;
                        music_sink.set_volume(music_ratio);
                        let sfx_ratio = audio.sfx_volume as f32 / 25.0 * master_ratio;
                        sfx_sink.set_volume(sfx_ratio);
                    }
                }
                AudioOption::MusicVolume => {
                    if audio.music_volume > 0 {
                        audio.music_volume -= 1;
                        let master_ratio = audio.master_volume as f32 / 25.0;
                        let music_ratio = audio.music_volume as f32 / 25.0 * master_ratio;
                        music_sink.set_volume(music_ratio);
                    }
                }
                AudioOption::SfxVolume => {
                    if audio.sfx_volume > 0 {
                        audio.sfx_volume -= 1;
                        let master_ratio = audio.master_volume as f32 / 25.0;
                        let sfx_ratio = audio.sfx_volume as f32 / 25.0 * master_ratio;
                        sfx_sink.set_volume(sfx_ratio);
                    }
                }
                AudioOption::Back => (),
            }
        } else if key == keys.move_right {
            match current_option {
                AudioOption::MasterVolume => {
                    if audio.master_volume < 25 {
                        audio.master_volume += 1;
                        let master_ratio = audio.master_volume as f32 / 25.0;
                        let music_ratio = audio.music_volume as f32 / 25.0 * master_ratio;
                        music_sink.set_volume(music_ratio);
                        let sfx_ratio = audio.sfx_volume as f32 / 25.0 * master_ratio;
                        sfx_sink.set_volume(sfx_ratio);
                    }
                }
                AudioOption::MusicVolume => {
                    if audio.music_volume < 25 {
                        audio.music_volume += 1;
                        let master_ratio = audio.master_volume as f32 / 25.0;
                        let music_ratio = audio.music_volume as f32 / 25.0 * master_ratio;
                        music_sink.set_volume(music_ratio);
                    }
                }
                AudioOption::SfxVolume => {
                    if audio.sfx_volume < 25 {
                        audio.sfx_volume += 1;
                        let master_ratio = audio.master_volume as f32 / 25.0;
                        let sfx_ratio = audio.sfx_volume as f32 / 25.0 * master_ratio;
                        sfx_sink.set_volume(sfx_ratio);
                    }
                }
                AudioOption::Back => (),
            }
        }
    }

    current_option
}
