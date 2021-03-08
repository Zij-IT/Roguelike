use super::gui;
use enum_cycling::{EnumCycle, IntoEnumCycle};
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

#[derive(PartialEq, Copy, Clone, Debug, AsRefStr, EnumCycle)]
pub enum AudioOption {
    #[strum(serialize = "Master Volume")]
    MasterVolume,
    #[strum(serialize = "Music Volume")]
    MusicVolume,
    #[strum(serialize = "Sound Effects")]
    SfxVolume,
    #[skip]
    Back,
}

#[derive(PartialEq, Copy, Clone, Debug, AsRefStr, EnumCycle)]
pub enum VisualOption {
    #[strum(serialize = "Full Screen")]
    FullScreen,
    #[strum(serialize = "Dynamic Color")]
    DynamicColor,
    #[strum(serialize = "Screen Shake")]
    ScreenShake,
    #[strum(serialize = "Active Font")]
    ActiveFont,
    #[strum(serialize = "Color Mapping")]
    ColorMapping,
    #[skip]
    Back,
}

#[derive(PartialEq, Copy, Clone, Debug, AsRefStr, EnumCycle, EnumIter)]
pub enum KeyBindingOption {
    Right,
    Left,
    Up,
    Down,
    #[strum(serialize = "Up & Right")]
    UpRight,
    #[strum(serialize = "Up & Left")]
    UpLeft,
    #[strum(serialize = "Down & Right")]
    DownRight,
    #[strum(serialize = "Down & Left")]
    DownLeft,
    Descend,
    Inventory,
    #[strum(serialize = "Grab Item")]
    GrabItem,
    #[strum(serialize = "Drop Item")]
    DropItem,
    #[strum(serialize = "Remove Item")]
    RemoveItem,
    #[strum(serialize = "Back")]
    GoBack,
    #[strum(serialize = "Wait a Turn")]
    WaitTurn,
    Select,
    #[skip]
    Back,
}
