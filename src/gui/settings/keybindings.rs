use crate::{
    constants::{colors, consoles},
    raws::config::Config,
    rex_assets::RexAssets,
    state::KeyBindingOption,
};
use enum_cycling::IntoEnumCycle;
use rltk::{Rltk, VirtualKeyCode, RGB};
use std::sync::atomic::{AtomicBool, Ordering};
use strum::IntoEnumIterator;

pub fn show(
    configs: &mut Config,
    ctx: &mut Rltk,
    current_option: KeyBindingOption,
    assets: &RexAssets,
) -> (KeyBindingOption, bool) {
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.keybindings, 0, 0);

    draw_all_keys(configs, ctx, current_option);

    let keys = &configs.keys;

    if let Some(key) = ctx.key {
        if key == keys.move_up {
            return (current_option.up(), false);
        } else if key == keys.move_down {
            return (current_option.down(), false);
        } else if key == keys.go_back {
            return (KeyBindingOption::Back, false);
        } else if key == keys.select {
            return (current_option, true);
        }
    }
    (current_option, false)
}

pub fn key_selected(
    configs: &mut Config,
    ctx: &mut Rltk,
    current_option: KeyBindingOption,
    assets: &RexAssets,
) -> bool {
    lazy_static::lazy_static! {
        static ref BAD_KEY_CHOSEN: AtomicBool = AtomicBool::new(false);
    }

    ctx.render_xp_sprite(&assets.keybindings, 0, 0);

    let (half_width, half_height) = {
        let (w, h) = ctx.get_char_size();
        (w as i32 / 2, h as i32 / 2)
    };

    let box_width = 22;
    let box_height = 10;

    draw_all_keys(configs, ctx, current_option);

    ctx.draw_box(
        half_width - box_width / 2,
        half_height - box_height,
        box_width,
        box_height,
        RGB::named(colors::FOREGROUND),
        RGB::named(colors::BACKGROUND),
    );

    ctx.print_color_centered(
        half_height - box_height / 2 - 4,
        RGB::named(colors::FOREGROUND),
        RGB::named(colors::BACKGROUND),
        "Press A Key",
    );

    if BAD_KEY_CHOSEN.load(Ordering::Relaxed) {
        ctx.print_color_centered(
            half_height - 4,
            RGB::named(rltk::RED),
            RGB::named(colors::BACKGROUND),
            "Key already assigned.",
        );
        ctx.print_color_centered(
            half_height - 3,
            RGB::named(rltk::RED),
            RGB::named(colors::BACKGROUND),
            "Please try again.",
        );
    }

    if let Some(key) = ctx.key {
        if KeyBindingOption::iter().all(|option| *option_to_config(configs, option) != key) {
            *option_to_config(configs, current_option) = key;
            BAD_KEY_CHOSEN.store(false, Ordering::Relaxed);
            return true;
        }
        BAD_KEY_CHOSEN.store(true, Ordering::Relaxed);
    }
    false
}

fn draw_all_keys(configs: &mut Config, ctx: &mut Rltk, current_option: KeyBindingOption) {
    let yellow = RGB::named(rltk::YELLOW);
    let bg = colors::BACKGROUND;

    let base_y = 5;
    let x = 26;

    for (i, option) in KeyBindingOption::iter().enumerate() {
        if option != KeyBindingOption::Back {
            let y = base_y + 2 * i;
            if current_option == option {
                ctx.print_color(x, y, yellow, bg, current_option.as_ref());
            }
            let key = vlc_to_str(*option_to_config(configs, option));
            ctx.print_color(42, y, RGB::named(colors::FOREGROUND), bg, key);
        }
    }
}

fn option_to_config(configs: &mut Config, current_option: KeyBindingOption) -> &mut VirtualKeyCode {
    match current_option {
        KeyBindingOption::Right => &mut configs.keys.move_right,
        KeyBindingOption::Left => &mut configs.keys.move_left,
        KeyBindingOption::Up => &mut configs.keys.move_up,
        KeyBindingOption::Down => &mut configs.keys.move_down,
        KeyBindingOption::UpRight => &mut configs.keys.move_up_right,
        KeyBindingOption::UpLeft => &mut configs.keys.move_up_left,
        KeyBindingOption::DownRight => &mut configs.keys.move_down_right,
        KeyBindingOption::DownLeft => &mut configs.keys.move_down_left,
        KeyBindingOption::Descend => &mut configs.keys.descend,
        KeyBindingOption::Inventory => &mut configs.keys.open_inventory,
        KeyBindingOption::GrabItem => &mut configs.keys.grab_item,
        KeyBindingOption::DropItem => &mut configs.keys.drop_item,
        KeyBindingOption::RemoveItem => &mut configs.keys.remove_item,
        KeyBindingOption::WaitTurn => &mut configs.keys.wait_turn,
        KeyBindingOption::Select => &mut configs.keys.select,
        KeyBindingOption::Back | KeyBindingOption::GoBack => &mut configs.keys.go_back,
    }
}

const fn vlc_to_str(vlc: VirtualKeyCode) -> &'static str {
    match vlc {
        VirtualKeyCode::Key1 => "1",
        VirtualKeyCode::Key2 => "2",
        VirtualKeyCode::Key3 => "3",
        VirtualKeyCode::Key4 => "4",
        VirtualKeyCode::Key5 => "5",
        VirtualKeyCode::Key6 => "6",
        VirtualKeyCode::Key7 => "7",
        VirtualKeyCode::Key8 => "8",
        VirtualKeyCode::Key9 => "9",
        VirtualKeyCode::Key0 => "0",
        VirtualKeyCode::A => "A",
        VirtualKeyCode::B => "B",
        VirtualKeyCode::C => "C",
        VirtualKeyCode::D => "D",
        VirtualKeyCode::E => "E",
        VirtualKeyCode::F => "F",
        VirtualKeyCode::G => "G",
        VirtualKeyCode::H => "H",
        VirtualKeyCode::I => "I",
        VirtualKeyCode::J => "J",
        VirtualKeyCode::K => "K",
        VirtualKeyCode::L => "L",
        VirtualKeyCode::M => "M",
        VirtualKeyCode::N => "N",
        VirtualKeyCode::O => "O",
        VirtualKeyCode::P => "P",
        VirtualKeyCode::Q => "Q",
        VirtualKeyCode::R => "R",
        VirtualKeyCode::S => "S",
        VirtualKeyCode::T => "T",
        VirtualKeyCode::U => "U",
        VirtualKeyCode::V => "V",
        VirtualKeyCode::W => "W",
        VirtualKeyCode::X => "X",
        VirtualKeyCode::Y => "Y",
        VirtualKeyCode::Z => "Z",
        VirtualKeyCode::Escape => "Esc",
        VirtualKeyCode::F1 => "F1",
        VirtualKeyCode::F2 => "F2",
        VirtualKeyCode::F3 => "F3",
        VirtualKeyCode::F4 => "F4",
        VirtualKeyCode::F5 => "F5",
        VirtualKeyCode::F6 => "F6",
        VirtualKeyCode::F7 => "F7",
        VirtualKeyCode::F8 => "F8",
        VirtualKeyCode::F9 => "F9",
        VirtualKeyCode::F10 => "F10",
        VirtualKeyCode::F11 => "F11",
        VirtualKeyCode::F12 => "F12",
        VirtualKeyCode::Left => "Left",
        VirtualKeyCode::Up => "Up",
        VirtualKeyCode::Right => "Right",
        VirtualKeyCode::Down => "Down",
        VirtualKeyCode::Back => "Back",
        VirtualKeyCode::Return => "Return",
        VirtualKeyCode::Space => "Space",
        VirtualKeyCode::Numlock => "Numlock",
        VirtualKeyCode::Numpad0 => "0 (Num)",
        VirtualKeyCode::Numpad1 => "1 (Num)",
        VirtualKeyCode::Numpad2 => "2 (Num)",
        VirtualKeyCode::Numpad3 => "3 (Num)",
        VirtualKeyCode::Numpad4 => "4 (Num)",
        VirtualKeyCode::Numpad5 => "5 (Num)",
        VirtualKeyCode::Numpad6 => "6 (Num)",
        VirtualKeyCode::Numpad7 => "7 (Num)",
        VirtualKeyCode::Numpad8 => "8 (Num)",
        VirtualKeyCode::Numpad9 => "9 (Num)",
        VirtualKeyCode::NumpadAdd => "+ (Num)",
        VirtualKeyCode::NumpadDivide => "/ (Num)",
        VirtualKeyCode::NumpadDecimal => ". (Num)",
        VirtualKeyCode::NumpadComma => ", (Num)",
        VirtualKeyCode::NumpadEquals => "= (Num)",
        VirtualKeyCode::NumpadMultiply => "* (Num)",
        VirtualKeyCode::NumpadSubtract => "- (Num)",
        VirtualKeyCode::NumpadEnter => "Enter (Num)",
        VirtualKeyCode::Backslash => "\\",
        VirtualKeyCode::Comma => ",",
        VirtualKeyCode::Minus => "-",
        VirtualKeyCode::Period => ".",
        VirtualKeyCode::Plus => "+",
        VirtualKeyCode::Tab => "Tab",
        _ => "Haven't gotten there yet",
    }
}
