mod bsp_interior_builder;
mod bsp_map_builder;
mod cellular_automata_builder;
mod common;
mod drunkard_builder;
mod maze_builder;
mod simple_map_builder;

pub mod map;
pub mod rect;

use bsp_interior_builder::BSPInteriorBuilder;
use bsp_map_builder::BSPMapBuilder;
use cellular_automata_builder::CellularAutomataBuilder;
use drunkard_builder::{DrunkardSpawnMode, DrunkardsBuilder};
use map::Map;
use maze_builder::MazeBuilder;
use simple_map_builder::SimpleMapBuilder;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut specs::World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> super::ecs::Position;
}

pub fn random_builder(width: i32, height: i32, depth: i32) -> Box<dyn MapBuilder> {
    let mut rng = rltk::RandomNumberGenerator::new();
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
