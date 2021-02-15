use rltk::rex::XpFile;

//In rust you are not able to use const string slices inside of macros, and because I don't want to
//type the same thing multiple times and have an error result out of that, I am using a macro as a
//constant. If that is too dirty for you, I suggest you avert your eyes
#[rustfmt::skip]
macro_rules! title_screen_path {
    () => ("../resources/title_screen.xp")
}

#[rustfmt::skip]
macro_rules! blank_ui_path {
    () => ("../resources/blank_ui.xp")
}

rltk::embedded_resource!(TITLE_SCREEN, title_screen_path!());
rltk::embedded_resource!(BLANK_UI, blank_ui_path!());

pub struct RexAssets {
    pub title_screen: XpFile,
    pub blank_ui: XpFile,
}

impl RexAssets {
    pub fn new() -> RexAssets {
        rltk::link_resource!(TITLE_SCREEN, title_screen_path!());
        rltk::link_resource!(BLANK_UI, blank_ui_path!());

        RexAssets {
            title_screen: XpFile::from_resource(title_screen_path!()).unwrap(),
            blank_ui: XpFile::from_resource(blank_ui_path!()).unwrap(),
        }
    }
}
