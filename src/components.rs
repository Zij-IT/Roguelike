use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: rltk::RGB,
    pub bg: rltk::RGB,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub is_dirty: bool,
}
