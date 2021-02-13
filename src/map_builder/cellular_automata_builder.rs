use super::{map::*, MapBuilder};
use crate::{spawner, Position};
use rltk::RandomNumberGenerator;
use specs::World;
use std::collections::HashMap;

const MAX_ITERATIONS: usize = 15;
const EDGE_BUFFER: i32 = 2;
const MAX_STEPS: f32 = 200.0;

pub struct CellularAutomataBuilder {
    map: Map,
    starting_position: Position,
    history: Vec<Map>,
    noise_areas: HashMap<i32, Vec<(i32, i32)>>,
}

impl CellularAutomataBuilder {
    pub fn new(new_depth: i32) -> CellularAutomataBuilder {
        CellularAutomataBuilder {
            map: Map::new(new_depth),
            starting_position: Position { x: 0, y: 0 },
            history: Vec::new(),
            noise_areas: HashMap::new(),
        }
    }
}

impl MapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self) {
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
        self.take_snapshot();

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
            self.take_snapshot();
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

        //Finding exit
        let dijkstra_map = rltk::DijkstraMap::new(
            self.map.width,
            self.map.height,
            &[start_idx],
            &self.map,
            MAX_STEPS, //Stop counting at max steps
        );
        let mut exit_tile = (0, 0.0);

        for (i, tile) in self.map.tiles.iter_mut().enumerate() {
            if *tile == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                if distance_to_start == f32::MAX {
                    //If unreachable, make it a wall
                    *tile = TileType::Wall;
                } else {
                    //Put exit furthest point away from player
                    if distance_to_start > exit_tile.1 {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        self.take_snapshot();
        self.map.tiles[exit_tile.0] = TileType::StairsDown;
        self.take_snapshot();

        //Build noise map for use in spawning entiites
        let mut noise = rltk::FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
        noise.set_noise_type(rltk::NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(rltk::CellularDistanceFunction::Manhattan);

        for y in 1..self.map.height - 1 {
            for x in 1..self.map.width - 1 {
                let idx = self.map.xy_idx(x, y);
                if self.map.tiles[idx] == TileType::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value_i = cell_value_f as i32;
                    if self.noise_areas.contains_key(&cell_value_i) {
                        self.noise_areas
                            .get_mut(&cell_value_i)
                            .unwrap()
                            .push((x, y));
                    } else {
                        self.noise_areas.insert(cell_value_i, vec![(x, y)]);
                    }
                }
            }
        }
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1, self.map.depth);
        }
    }

    fn take_snapshot(&mut self) {
        if crate::SHOW_MAPGEN {
            let mut snapshot = self.get_map();
            for tile in 0..snapshot.tile_status.len() {
                snapshot.set_tile_status(tile, TILE_REVEALED);
            }
            self.history.push(snapshot);
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }
}

impl CellularAutomataBuilder {}
