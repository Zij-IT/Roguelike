use crate::{
    constants::{colors, consoles},
    raws::config::Config,
    rex_assets::RexAssets,
    state::KeyBindingOption,
};
use enum_cycling::IntoEnumCycle;
use rltk::{Rltk, VirtualKeyCode, RGB};
use strum::IntoEnumIterator;

pub fn show(
    configs: &mut Config,
    ctx: &mut Rltk,
    current_option: KeyBindingOption,
    assets: &RexAssets,
) -> KeyBindingOption {
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_keybindings, 0, 0);

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
            let key = vlc_to_str(option_to_vlc(configs, option));
            ctx.print_color(42, y, RGB::named(colors::FOREGROUND), bg, key);
        }
    }

    let keys = &configs.keys;

    if let Some(key) = ctx.key {
        if key == keys.move_up {
            return current_option.up();
        } else if key == keys.move_down {
            return current_option.down();
        } else if key == keys.go_back {
            return KeyBindingOption::Back;
        }
    }
    current_option
}

fn option_to_vlc(configs: &mut Config, current_option: KeyBindingOption) -> VirtualKeyCode {
    let keys = &configs.keys;
    match current_option {
        KeyBindingOption::Right => keys.move_right,
        KeyBindingOption::Left => keys.move_left,
        KeyBindingOption::Up => keys.move_up,
        KeyBindingOption::Down => keys.move_down,
        KeyBindingOption::UpRight => keys.move_up_right,
        KeyBindingOption::UpLeft => keys.move_up_left,
        KeyBindingOption::DownRight => keys.move_down_right,
        KeyBindingOption::DownLeft => keys.move_down_left,
        KeyBindingOption::Descend => keys.descend,
        KeyBindingOption::Inventory => keys.open_inventory,
        KeyBindingOption::GrabItem => keys.grab_item,
        KeyBindingOption::DropItem => keys.drop_item,
        KeyBindingOption::RemoveItem => keys.remove_item,
        KeyBindingOption::GoBack => keys.go_back,
        KeyBindingOption::WaitTurn => keys.wait_turn,
        KeyBindingOption::Select => keys.select,
        KeyBindingOption::Back => keys.go_back,
    }
}

fn vlc_to_str(vlc: VirtualKeyCode) -> &'static str {
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
