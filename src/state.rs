use super::gui;
use strum::{AsRefStr, EnumIter};

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
#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum MainOption {
    #[strum(serialize = "Start Anew")]
    NewGame,
    #[strum(serialize = "Continue")]
    LoadGame,
    Settings,
    Quit,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum SettingsOption {
    Audio,
    Visual,
    Keybindings,
    Back,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum AudioOption {
    MasterVolume,
    SoundEffect,
    MusicVolume,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum VisualOption {
    FullScreen,
    DynamicColor,
    ScreenShake,
}

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, AsRefStr)]
pub enum KeyBindingOption {
    Up,
    Down,
    Left,
    Right,
    Wait,
}
