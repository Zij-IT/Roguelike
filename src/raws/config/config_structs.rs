use crate::gui::settings::visual;
use rltk::VirtualKeyCode;
use serde::Deserialize;
use serde::Serialize;

//Helping VirtualKeyCode
#[derive(Serialize, Deserialize)]
#[serde(remote = "VirtualKeyCode")]
enum VirtualKeyCodeDef {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Compose,
    Caret,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

impl From<VirtualKeyCodeDef> for VirtualKeyCode {
    #[allow(clippy::too_many_lines)]
    fn from(vlc: VirtualKeyCodeDef) -> Self {
        match vlc {
            VirtualKeyCodeDef::Key1 => Self::Key1,
            VirtualKeyCodeDef::Key2 => Self::Key2,
            VirtualKeyCodeDef::Key3 => Self::Key3,
            VirtualKeyCodeDef::Key4 => Self::Key4,
            VirtualKeyCodeDef::Key5 => Self::Key5,
            VirtualKeyCodeDef::Key6 => Self::Key6,
            VirtualKeyCodeDef::Key7 => Self::Key7,
            VirtualKeyCodeDef::Key8 => Self::Key8,
            VirtualKeyCodeDef::Key9 => Self::Key9,
            VirtualKeyCodeDef::Key0 => Self::Key0,
            VirtualKeyCodeDef::A => Self::A,
            VirtualKeyCodeDef::B => Self::B,
            VirtualKeyCodeDef::C => Self::C,
            VirtualKeyCodeDef::D => Self::D,
            VirtualKeyCodeDef::E => Self::E,
            VirtualKeyCodeDef::F => Self::F,
            VirtualKeyCodeDef::G => Self::G,
            VirtualKeyCodeDef::H => Self::H,
            VirtualKeyCodeDef::I => Self::I,
            VirtualKeyCodeDef::J => Self::J,
            VirtualKeyCodeDef::K => Self::K,
            VirtualKeyCodeDef::L => Self::L,
            VirtualKeyCodeDef::M => Self::M,
            VirtualKeyCodeDef::N => Self::N,
            VirtualKeyCodeDef::O => Self::O,
            VirtualKeyCodeDef::P => Self::P,
            VirtualKeyCodeDef::Q => Self::Q,
            VirtualKeyCodeDef::R => Self::R,
            VirtualKeyCodeDef::S => Self::S,
            VirtualKeyCodeDef::T => Self::T,
            VirtualKeyCodeDef::U => Self::U,
            VirtualKeyCodeDef::V => Self::V,
            VirtualKeyCodeDef::W => Self::W,
            VirtualKeyCodeDef::X => Self::X,
            VirtualKeyCodeDef::Y => Self::Y,
            VirtualKeyCodeDef::Z => Self::Z,
            VirtualKeyCodeDef::Escape => Self::Escape,
            VirtualKeyCodeDef::F1 => Self::F1,
            VirtualKeyCodeDef::F2 => Self::F2,
            VirtualKeyCodeDef::F3 => Self::F3,
            VirtualKeyCodeDef::F4 => Self::F4,
            VirtualKeyCodeDef::F5 => Self::F5,
            VirtualKeyCodeDef::F6 => Self::F6,
            VirtualKeyCodeDef::F7 => Self::F7,
            VirtualKeyCodeDef::F8 => Self::F8,
            VirtualKeyCodeDef::F9 => Self::F9,
            VirtualKeyCodeDef::F10 => Self::F10,
            VirtualKeyCodeDef::F11 => Self::F11,
            VirtualKeyCodeDef::F12 => Self::F12,
            VirtualKeyCodeDef::F13 => Self::F13,
            VirtualKeyCodeDef::F14 => Self::F14,
            VirtualKeyCodeDef::F15 => Self::F15,
            VirtualKeyCodeDef::F16 => Self::F16,
            VirtualKeyCodeDef::F17 => Self::F17,
            VirtualKeyCodeDef::F18 => Self::F18,
            VirtualKeyCodeDef::F19 => Self::F19,
            VirtualKeyCodeDef::F20 => Self::F20,
            VirtualKeyCodeDef::F21 => Self::F21,
            VirtualKeyCodeDef::F22 => Self::F22,
            VirtualKeyCodeDef::F23 => Self::F23,
            VirtualKeyCodeDef::F24 => Self::F24,
            VirtualKeyCodeDef::Snapshot => Self::Snapshot,
            VirtualKeyCodeDef::Scroll => Self::Scroll,
            VirtualKeyCodeDef::Pause => Self::Pause,
            VirtualKeyCodeDef::Insert => Self::Insert,
            VirtualKeyCodeDef::Home => Self::Home,
            VirtualKeyCodeDef::Delete => Self::Delete,
            VirtualKeyCodeDef::End => Self::End,
            VirtualKeyCodeDef::PageDown => Self::PageDown,
            VirtualKeyCodeDef::PageUp => Self::PageUp,
            VirtualKeyCodeDef::Left => Self::Left,
            VirtualKeyCodeDef::Up => Self::Up,
            VirtualKeyCodeDef::Right => Self::Right,
            VirtualKeyCodeDef::Down => Self::Down,
            VirtualKeyCodeDef::Back => Self::Back,
            VirtualKeyCodeDef::Return => Self::Return,
            VirtualKeyCodeDef::Space => Self::Space,
            VirtualKeyCodeDef::Compose => Self::Compose,
            VirtualKeyCodeDef::Caret => Self::Caret,
            VirtualKeyCodeDef::Numlock => Self::Numlock,
            VirtualKeyCodeDef::Numpad0 => Self::Numpad0,
            VirtualKeyCodeDef::Numpad1 => Self::Numpad1,
            VirtualKeyCodeDef::Numpad2 => Self::Numpad2,
            VirtualKeyCodeDef::Numpad3 => Self::Numpad3,
            VirtualKeyCodeDef::Numpad4 => Self::Numpad4,
            VirtualKeyCodeDef::Numpad5 => Self::Numpad5,
            VirtualKeyCodeDef::Numpad6 => Self::Numpad6,
            VirtualKeyCodeDef::Numpad7 => Self::Numpad7,
            VirtualKeyCodeDef::Numpad8 => Self::Numpad8,
            VirtualKeyCodeDef::Numpad9 => Self::Numpad9,
            VirtualKeyCodeDef::NumpadAdd => Self::NumpadAdd,
            VirtualKeyCodeDef::NumpadComma => Self::NumpadComma,
            VirtualKeyCodeDef::NumpadDecimal => Self::NumpadDecimal,
            VirtualKeyCodeDef::NumpadDivide => Self::NumpadDivide,
            VirtualKeyCodeDef::NumpadEnter => Self::NumpadEnter,
            VirtualKeyCodeDef::NumpadEquals => Self::NumpadEquals,
            VirtualKeyCodeDef::NumpadMultiply => Self::NumpadMultiply,
            VirtualKeyCodeDef::NumpadSubtract => Self::NumpadSubtract,
            VirtualKeyCodeDef::AbntC1 => Self::AbntC1,
            VirtualKeyCodeDef::AbntC2 => Self::AbntC2,
            VirtualKeyCodeDef::Apostrophe => Self::Apostrophe,
            VirtualKeyCodeDef::Apps => Self::Apps,
            VirtualKeyCodeDef::Asterisk => Self::Asterisk,
            VirtualKeyCodeDef::At => Self::At,
            VirtualKeyCodeDef::Ax => Self::Ax,
            VirtualKeyCodeDef::Backslash => Self::Backslash,
            VirtualKeyCodeDef::Calculator => Self::Calculator,
            VirtualKeyCodeDef::Capital => Self::Capital,
            VirtualKeyCodeDef::Colon => Self::Colon,
            VirtualKeyCodeDef::Comma => Self::Comma,
            VirtualKeyCodeDef::Convert => Self::Convert,
            VirtualKeyCodeDef::Equals => Self::Equals,
            VirtualKeyCodeDef::Grave => Self::Grave,
            VirtualKeyCodeDef::Kana => Self::Kana,
            VirtualKeyCodeDef::Kanji => Self::Kanji,
            VirtualKeyCodeDef::LAlt => Self::LAlt,
            VirtualKeyCodeDef::LBracket => Self::LBracket,
            VirtualKeyCodeDef::LControl => Self::LControl,
            VirtualKeyCodeDef::LShift => Self::LShift,
            VirtualKeyCodeDef::LWin => Self::LWin,
            VirtualKeyCodeDef::Mail => Self::Mail,
            VirtualKeyCodeDef::MediaSelect => Self::MediaSelect,
            VirtualKeyCodeDef::MediaStop => Self::MediaStop,
            VirtualKeyCodeDef::Minus => Self::Minus,
            VirtualKeyCodeDef::Mute => Self::Mute,
            VirtualKeyCodeDef::MyComputer => Self::MyComputer,
            VirtualKeyCodeDef::NavigateForward => Self::NavigateForward,
            VirtualKeyCodeDef::NavigateBackward => Self::NavigateBackward,
            VirtualKeyCodeDef::NextTrack => Self::NextTrack,
            VirtualKeyCodeDef::NoConvert => Self::NoConvert,
            VirtualKeyCodeDef::OEM102 => Self::OEM102,
            VirtualKeyCodeDef::Period => Self::Period,
            VirtualKeyCodeDef::PlayPause => Self::PlayPause,
            VirtualKeyCodeDef::Plus => Self::Plus,
            VirtualKeyCodeDef::Power => Self::Power,
            VirtualKeyCodeDef::PrevTrack => Self::PrevTrack,
            VirtualKeyCodeDef::RAlt => Self::RAlt,
            VirtualKeyCodeDef::RBracket => Self::RBracket,
            VirtualKeyCodeDef::RControl => Self::RControl,
            VirtualKeyCodeDef::RShift => Self::RShift,
            VirtualKeyCodeDef::RWin => Self::RWin,
            VirtualKeyCodeDef::Semicolon => Self::Semicolon,
            VirtualKeyCodeDef::Slash => Self::Slash,
            VirtualKeyCodeDef::Sleep => Self::Sleep,
            VirtualKeyCodeDef::Stop => Self::Stop,
            VirtualKeyCodeDef::Sysrq => Self::Sysrq,
            VirtualKeyCodeDef::Tab => Self::Tab,
            VirtualKeyCodeDef::Underline => Self::Underline,
            VirtualKeyCodeDef::Unlabeled => Self::Unlabeled,
            VirtualKeyCodeDef::VolumeDown => Self::VolumeDown,
            VirtualKeyCodeDef::VolumeUp => Self::VolumeUp,
            VirtualKeyCodeDef::Wake => Self::Wake,
            VirtualKeyCodeDef::WebBack => Self::WebBack,
            VirtualKeyCodeDef::WebFavorites => Self::WebFavorites,
            VirtualKeyCodeDef::WebForward => Self::WebForward,
            VirtualKeyCodeDef::WebHome => Self::WebHome,
            VirtualKeyCodeDef::WebRefresh => Self::WebRefresh,
            VirtualKeyCodeDef::WebSearch => Self::WebSearch,
            VirtualKeyCodeDef::WebStop => Self::WebStop,
            VirtualKeyCodeDef::Yen => Self::Yen,
            VirtualKeyCodeDef::Copy => Self::Copy,
            VirtualKeyCodeDef::Paste => Self::Paste,
            VirtualKeyCodeDef::Cut => Self::Cut,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyBinds {
    //Movement Keys
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_up: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_down: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_left: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_right: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_up_left: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_up_right: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_down_left: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub move_down_right: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub descend: VirtualKeyCode,

    //Item Related keys
    #[serde(with = "VirtualKeyCodeDef")]
    pub grab_item: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub drop_item: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub remove_item: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub open_inventory: VirtualKeyCode,

    //Other keys
    #[serde(with = "VirtualKeyCodeDef")]
    pub go_back: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub wait_turn: VirtualKeyCode,
    #[serde(with = "VirtualKeyCodeDef")]
    pub select: VirtualKeyCode,
}
impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            //Movement
            move_up: VirtualKeyCode::K,
            move_down: VirtualKeyCode::J,
            move_left: VirtualKeyCode::H,
            move_right: VirtualKeyCode::L,
            move_up_left: VirtualKeyCode::Z,
            move_up_right: VirtualKeyCode::U,
            move_down_left: VirtualKeyCode::B,
            move_down_right: VirtualKeyCode::N,
            descend: VirtualKeyCode::Period,

            //Item related
            grab_item: VirtualKeyCode::G,
            drop_item: VirtualKeyCode::D,
            remove_item: VirtualKeyCode::R,
            open_inventory: VirtualKeyCode::I,

            //Other
            go_back: VirtualKeyCode::Escape,
            wait_turn: VirtualKeyCode::Space,
            select: VirtualKeyCode::Return,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VisualConfigs {
    pub full_screen: bool,
    pub screen_shake: bool,
    pub dynamic_color: bool,
    pub active_font: visual::Font,
    pub color_mapping: ColorMapping,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorMapping {
    pub player: (u8, u8, u8),
    pub enemy: (u8, u8, u8),
    pub collectable: (u8, u8, u8),
    pub grass: (u8, u8, u8),
    pub water: (u8, u8, u8),
    pub deep_water: (u8, u8, u8),
    pub lava: (u8, u8, u8),
}

impl Default for ColorMapping {
    fn default() -> Self {
        Self {
            player: (178, 178, 0),
            enemy: (140, 0, 35),
            collectable: (0, 140, 140),
            grass: (70, 140, 0),
            water: (0, 70, 140),
            deep_water: (0, 0, 140),
            lava: (140, 0, 35),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AudioConfigs {
    pub master_volume: usize,
    pub music_volume: usize,
    pub sfx_volume: usize,
}

impl Default for AudioConfigs {
    fn default() -> Self {
        Self {
            master_volume: 0,
            music_volume: 0,
            sfx_volume: 0,
        }
    }
}
