use crate::{
    constants::{colors, consoles},
    raws::config::Config,
    rex_assets::RexAssets,
    state::VisualOption,
};
use enum_cycling::{EnumCycle, IntoEnumCycle};
use rltk::{Rltk, RGB};
use serde::Deserialize;
use serde::Serialize;
use strum::AsRefStr;

#[derive(Serialize, Deserialize, Clone, EnumCycle, AsRefStr)]
pub enum Font {
    Default,
    Others,
}

pub fn show(
    configs: &mut Config,
    ctx: &mut Rltk,
    current_option: VisualOption,
    assets: &RexAssets,
) -> VisualOption {
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_visual, 0, 0);

    //Set defaults
    let yellow = RGB::named(rltk::YELLOW);
    let bg = RGB::from(colors::BACKGROUND);
    let opt = current_option.as_ref();

    match current_option {
        VisualOption::FullScreen => ctx.print_color(26, 5, yellow, bg, opt),
        VisualOption::DynamicColor => ctx.print_color(26, 7, yellow, bg, opt),
        VisualOption::ScreenShake => ctx.print_color(26, 9, yellow, bg, opt),
        VisualOption::ActiveFont => ctx.print_color(26, 11, yellow, bg, opt),
        VisualOption::ColorMapping => ctx.print_color(26, 13, yellow, bg, opt),
        VisualOption::Back => (),
    }

    //Snag configs
    let active_font = configs.visual.active_font.as_ref();

    //Temp mut
    let font_print = {
        let mut font_print = String::with_capacity(active_font.len() + 2);
        font_print.push_str(active_font);
        font_print.push_str(" >");
        font_print
    };

    ctx.print_color(43, 11, RGB::named(colors::FOREGROUND), bg, font_print);

    let x_on = 41;
    let x_off = 46;
    let y = 5;

    let on_color = RGB::named((108, 217, 0));
    let off_color = RGB::named((217, 0, 54));

    let visual = &mut configs.visual;

    if visual.full_screen {
        ctx.print_color(x_on, y, on_color, bg, "On");
    } else {
        ctx.print_color(x_off, y, off_color, bg, "Off");
    }

    if visual.dynamic_color {
        ctx.print_color(x_on, y + 2, on_color, bg, "On");
    } else {
        ctx.print_color(x_off, y + 2, off_color, bg, "Off");
    }

    if visual.screen_shake {
        ctx.print_color(x_on, y + 4, on_color, bg, "On");
    } else {
        ctx.print_color(x_off, y + 4, off_color, bg, "Off");
    }

    let mut left = false;
    let mut right = false;

    let keys = &configs.keys;

    if let Some(key) = ctx.key {
        if key == keys.go_back {
            return VisualOption::Back;
        } else if key == keys.move_up {
            return current_option.up();
        } else if key == keys.move_down {
            return current_option.down();
        }

        left = key == keys.move_left;
        right = key == keys.move_right;
    }

    match current_option {
        VisualOption::FullScreen => {
            if left || right {
                visual.full_screen = !visual.full_screen;
            }
        }
        VisualOption::DynamicColor => {
            if left || right {
                visual.dynamic_color = !visual.dynamic_color;
            }
        }
        VisualOption::ScreenShake => {
            if left || right {
                visual.screen_shake = !visual.screen_shake;
            }
        }
        VisualOption::ActiveFont => {
            if left {
                visual.active_font = visual.active_font.up();
            } else if right {
                visual.active_font = visual.active_font.down();
            }
        }
        VisualOption::ColorMapping | VisualOption::Back => {}
    }

    current_option
}
