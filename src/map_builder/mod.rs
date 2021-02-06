pub mod bsp_map_builder;
pub mod common;
pub mod map;
pub mod rect;
pub mod simple_map_builder;

pub use bsp_map_builder::*;
pub use common::*;
pub use map::*;
pub use rect::*;
pub use simple_map_builder::*;

use crate::Position;
use specs::World;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    //Box::new(SimpleMapBuilder::new(new_depth))
    Box::new(BSPMapBuilder::new(new_depth))
}
