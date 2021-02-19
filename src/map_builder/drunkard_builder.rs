use super::{
    common::{cull_and_set_exit, gen_voronoi_regions, EDGE_BUFFER},
    map::{Map, TileType},
    MapBuilder,
};
use crate::{spawning::spawn_region, Position};
use rltk::RandomNumberGenerator;
use specs::World;
use std::collections::HashMap;

#[allow(dead_code)]
pub enum DrunkardSpawnMode {
    Random,
    Centered,
}

pub struct DrunkardsBuilder {
    map: Map,
    starting_position: Position,
    noise_areas: HashMap<i32, Vec<(i32, i32)>>,
    spawn_mode: DrunkardSpawnMode,
    lifetime: i32,
}

impl DrunkardsBuilder {
    pub fn new(
        width: i32,
        height: i32,
        new_depth: i32,
        spawn_mode: DrunkardSpawnMode,
        lifetime: i32,
    ) -> DrunkardsBuilder {
        DrunkardsBuilder {
            map: Map::new(width, height, new_depth),
            starting_position: Position { x: 0, y: 0 },
            noise_areas: HashMap::new(),
            spawn_mode,
            lifetime,
        }
    }
}

impl MapBuilder for DrunkardsBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for area in self.noise_areas.iter() {
            spawn_region(ecs, area.1, self.map.depth);
        }
    }

    fn build_map(&mut self) {
        assert!(i32::checked_mul(self.map.width, self.map.height) != None);
        let mut rng = RandomNumberGenerator::new();

        //Always start in the center
        self.starting_position = Position {
            x: self.map.width / 2,
            y: self.map.height / 2,
        };

        let start_idx = self
            .map
            .xy_idx(self.starting_position.x, self.starting_position.y);

        self.map.tiles[start_idx] = TileType::Floor;

        //Begin the drunken digging
        let total_tiles = self.map.width * self.map.height;
        let min_floor_tiles = (total_tiles / 2) as usize;
        let mut floor_tile_count = self
            .map
            .tiles
            .iter()
            .filter(|&a| *a == TileType::Floor)
            .count();
        let mut drunk_x = self.starting_position.x;
        let mut drunk_y = self.starting_position.y;
        while floor_tile_count < min_floor_tiles {
            //Get starting locations
            for _ in 0..self.lifetime {
                let drunk_idx = self.map.xy_idx(drunk_x, drunk_y);
                self.map.tiles[drunk_idx] = TileType::Floor;
                match rng.roll_dice(1, 4) {
                    1 => {
                        if drunk_x > EDGE_BUFFER {
                            drunk_x -= 1
                        }
                    }
                    2 => {
                        if drunk_y > EDGE_BUFFER {
                            drunk_y -= 1;
                        }
                    }
                    3 => {
                        if drunk_x < self.map.width - EDGE_BUFFER {
                            drunk_x += 1;
                        }
                    }
                    _ => {
                        if drunk_y < self.map.height - EDGE_BUFFER {
                            drunk_y += 1;
                        }
                    }
                }
            }

            floor_tile_count = self
                .map
                .tiles
                .iter()
                .filter(|&a| *a == TileType::Floor)
                .count();

            match self.spawn_mode {
                DrunkardSpawnMode::Random => {
                    drunk_x = rng.roll_dice(1, self.map.width - 3) + 1;
                    drunk_y = rng.roll_dice(1, self.map.height - 3) + 1;
                }
                DrunkardSpawnMode::Centered => {
                    drunk_x = self.starting_position.x;
                    drunk_y = self.starting_position.y;
                }
            }
        }

        cull_and_set_exit(&mut self.map, start_idx);
        self.noise_areas = gen_voronoi_regions(&self.map, &mut rng);
    }
}
