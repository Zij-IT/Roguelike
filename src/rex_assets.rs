use rltk::rex::XpFile;

macro_rules! xp_from_path {
    ($filename : expr) => {{
        let byte_vec = include_bytes!($filename);
        let vec = {
            let mut temp = Vec::new();
            temp.extend_from_slice(byte_vec);
            temp
        };

        //Being passed to XpFile::read is a:
        //Mutable reference to an immutable reference to a slice of bytes
        rltk::XpFile::read(&mut &*vec).expect("Unable to read resource as XpFile")
    }};
}

pub struct RexAssets {
    pub title_screen: XpFile,
    pub ui: XpFile,
    pub inventory: XpFile,
    pub settings: XpFile,
    pub audio: XpFile,
    pub visual: XpFile,
    pub keybindings: XpFile,
    pub color_mapping: XpFile,
}

impl RexAssets {
    pub fn load() -> Self {
        Self {
            title_screen: xp_from_path!("../resources/xp_files/title_screen.xp"),
            ui: xp_from_path!("../resources/xp_files/ui.xp"),
            inventory: xp_from_path!("../resources/xp_files/inventory.xp"),
            settings: xp_from_path!("../resources/xp_files/settings.xp"),
            audio: xp_from_path!("../resources/xp_files/audio.xp"),
            visual: xp_from_path!("../resources/xp_files/visual.xp"),
            keybindings: xp_from_path!("../resources/xp_files/keybindings.xp"),
            color_mapping: xp_from_path!("../resources/xp_files/color_mapping_scene.xp"),
        }
    }
}
