use super::{common::*, map::*, MapBuilder};
use crate::{spawner, Position};
use rltk::RandomNumberGenerator;
use specs::World;
use std::collections::HashMap;

const MAX_ITERATIONS: usize = 15;

pub struct CellularAutomataBuilder {
    map: Map,
    starting_position: Position,
    noise_areas: HashMap<i32, Vec<(i32, i32)>>,
}

impl CellularAutomataBuilder {
    pub fn new(width: i32, height: i32, new_depth: i32) -> CellularAutomataBuilder {
        CellularAutomataBuilder {
            map: Map::new(width, height, new_depth),
            starting_position: Position { x: 0, y: 0 },
            noise_areas: HashMap::new(),
        }
    }
}

impl MapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self) {
        assert!(i32::checked_mul(self.map.width, self.map.height) != None);
        let mut rng = RandomNumberGenerator::new();

        //Randomize map
        for y in EDGE_BUFFER..self.map.height - EDGE_BUFFER {
            for x in EDGE_BUFFER..self.map.width - EDGE_BUFFER {
                let idx = self.map.xy_idx(x, y);
                if rng.roll_dice(1, 100) > 55 {
                    self.map.tiles[idx] = TileType::Floor;
                } else {
                    self.map.tiles[idx] = TileType::Wall;
                }
            }
        }

        let mut new_tiles = self.map.tiles.clone();
        for _ in 0..MAX_ITERATIONS {
            for y in EDGE_BUFFER..self.map.height - EDGE_BUFFER {
                for x in EDGE_BUFFER..self.map.width - EDGE_BUFFER {
                    let idx = self.map.xy_idx(x, y);
                    let mut neighbors = 0;
                    neighbors += (self.map.tiles[idx - 1] == TileType::Wall) as usize;
                    neighbors += (self.map.tiles[idx + 1] == TileType::Wall) as usize;
                    neighbors +=
                        (self.map.tiles[idx + self.map.width as usize] == TileType::Wall) as usize;
                    neighbors +=
                        (self.map.tiles[idx - self.map.width as usize] == TileType::Wall) as usize;
                    neighbors += (self.map.tiles[idx + self.map.width as usize + 1]
                        == TileType::Wall) as usize;
                    neighbors += (self.map.tiles[idx - self.map.width as usize + 1]
                        == TileType::Wall) as usize;
                    neighbors += (self.map.tiles[idx + self.map.width as usize - 1]
                        == TileType::Wall) as usize;
                    neighbors += (self.map.tiles[idx - self.map.width as usize - 1]
                        == TileType::Wall) as usize;
                    if neighbors > 4 || neighbors == 0 {
                        new_tiles[idx] = TileType::Wall;
                    } else {
                        new_tiles[idx] = TileType::Floor;
                    }
                }
            }
            self.map.tiles = new_tiles.clone();
        }

        //Find start tile. Go left up until a floor tile is found. Go up after x = 0
        let (center_x, center_y) = (self.map.width / 2, self.map.height / 2);
        let mut start_idx = self.map.xy_idx(center_x, center_y);
        while self.map.tiles[start_idx] != TileType::Floor {
            start_idx -= 1;
        }

        //Creating start pos
        self.starting_position = Position {
            x: start_idx as i32 % self.map.width,
            y: start_idx as i32 / self.map.width,
        };

        cull_and_set_exit(&mut self.map, start_idx);

        //Build noise map for use in spawning entiites
        self.noise_areas = gen_voronoi_regions(&self.map, &mut rng);
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1, self.map.depth);
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }
}

impl CellularAutomataBuilder {}
