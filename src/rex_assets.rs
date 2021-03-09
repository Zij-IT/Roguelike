use rltk::rex::XpFile;

//This bypasses having to load the resource in the rltk::EMBED struct which is normally done when a resource
//is loaded. As I am just collecting the files here, I do not need the files being stored in two
//separate locations.

macro_rules! xp_from_path {
    ($filename : expr) => {{
        let byte_vec = include_bytes!($filename)
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<u8>>();

        //Being passed to XpFile::read is a:
        //Mutable reference to an immutable reference to a slice of bytes
        rltk::XpFile::read(&mut &*byte_vec).expect("Unable to read resource as XpFile")
    }};
}

pub struct RexAssets {
    pub title_screen: XpFile,
    pub blank_ui: XpFile,
    pub blank_inv: XpFile,
    pub blank_settings: XpFile,
    pub blank_audio: XpFile,
    pub blank_visual: XpFile,
    pub blank_keybindings: XpFile,
}

impl RexAssets {
    pub fn new() -> Self {
        Self {
            title_screen: xp_from_path!("../resources/title_screen.xp"),
            blank_ui: xp_from_path!("../resources/b_ui.xp"),
            blank_inv: xp_from_path!("../resources/b_inventory.xp"),
            blank_settings: xp_from_path!("../resources/b_settings.xp"),
            blank_audio: xp_from_path!("../resources/b_audio.xp"),
            blank_visual: xp_from_path!("../resources/b_visual.xp"),
            blank_keybindings: xp_from_path!("../resources/b_keybindings.xp"),
        }
    }
}
