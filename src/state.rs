use super::gui;
use strum::{AsRefStr, EnumIter};
use enum_cycling::{EnumCycle, IntoEnumCycle};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum State {
    Menu(Menu),
    Game(Gameplay),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Gameplay {
    AwaitingInput,
    GameOver,
    MonsterTurn,
    NextLevel,
    PlayerTurn,
    PreRun,
    SaveGame,
    Inventory(gui::inventory::InvMode),
    ShowTargeting(i32, specs::Entity),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Menu {
    Main(MainOption),
    Settings(SettingsOption),
    Audio(AudioOption),
    Visual(VisualOption),
    Keybinding(KeyBindingOption),
}

//Menu Options
#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr, EnumCycle)]
pub enum MainOption {
    #[strum(serialize = "Start Anew")]
    NewGame,
    #[strum(serialize = "Continue")]
    LoadGame,
    Settings,
    Quit,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr, EnumCycle)]
pub enum SettingsOption {
    Audio,
    Visual,
    Keybindings,
    #[skip]
    Back,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr, EnumCycle)]
pub enum AudioOption {
    #[strum(serialize = "Master Volume")]
    MasterVolume,
    #[strum(serialize = "Music Volume")]
    MusicVolume,
    #[strum(serialize = "Sound Effects")]
    SoundEffect,
    #[skip]
    Back,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum VisualOption {
    #[strum(serialize = "Full Screen")]
    FullScreen,
    #[strum(serialize = "Dynamic Color")]
    DynamicColor,
    #[strum(serialize = "Screen Shake")]
    ScreenShake,
    Back,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum KeyBindingOption {
    Up,
    Down,
    Left,
    Right,
    Wait,
    Back,
}
