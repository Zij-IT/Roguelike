use rltk::rex::XpFile;

rltk::embedded_resource!(TITLE_SCREEN, "../resources/titlescreen.xp");

pub struct RexAssets {
    pub title_screen: XpFile,
}

impl RexAssets {
    pub fn new() -> RexAssets {
        rltk::link_resource!(TITLE_SCREEN, "../resources/titlescreen.xp");

        RexAssets {
            title_screen: XpFile::from_resource("../resources/titlescreen.xp").unwrap(),
        }
    }
}
