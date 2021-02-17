pub mod bsp_interior_builder;
pub mod bsp_map_builder;
pub mod cellular_automata_builder;
pub mod common;
pub mod drunkard_builder;
pub mod map;
pub mod maze_builder;
pub mod rect;
pub mod simple_map_builder;

pub use bsp_interior_builder::*;
pub use bsp_map_builder::*;
pub use cellular_automata_builder::*;
pub use common::*;
pub use drunkard_builder::*;
pub use map::*;
pub use maze_builder::*;
pub use rect::*;
pub use simple_map_builder::*;

use crate::Position;
use rltk::RandomNumberGenerator;
use specs::World;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
}

pub fn random_builder(width: i32, height: i32, depth: i32) -> Box<dyn MapBuilder> {
    let mut rng = RandomNumberGenerator::new();
    match rng.roll_dice(1, 6) {
        1 => Box::new(SimpleMapBuilder::new(width, height, depth)),
        2 => Box::new(BSPMapBuilder::new(width, height, depth)),
        3 => Box::new(BSPInteriorBuilder::new(width, height, depth)),
        4 => Box::new(CellularAutomataBuilder::new(width, height, depth)),
        5 => Box::new(DrunkardsBuilder::new(
            width,
            height,
            depth,
            DrunkardSpawnMode::Random,
            200,
        )),
        6 => Box::new(MazeBuilder::new(width, height, depth)),
        _ => unreachable!(),
    }
}
