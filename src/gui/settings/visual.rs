pub enum ColorTargets {
    Player,
    Enemies,
    Collectables,
    Grass,
    Water,
    DeepWater,
    Lava,
}

pub enum VisualSettings {
    FullScreen,
    DynamicColor,
    ScreenShake,
    ActiveFont(String),
    ColorMapping(ColorTargets),
}

pub fn show_vis() {}
