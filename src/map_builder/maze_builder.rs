use super::{common::*, Map, MapBuilder, TileType};
use crate::{spawner, Position};
use rltk::RandomNumberGenerator;
use specs::World;
use std::collections::HashMap;

pub struct MazeBuilder {
    map: Map,
    starting_position: Position,
    noise_areas: HashMap<i32, Vec<(i32, i32)>>,
}

impl MazeBuilder {
    pub fn new(width: i32, height: i32, new_depth: i32) -> MazeBuilder {
        MazeBuilder {
            map: Map::new(width, height, new_depth),
            starting_position: Position { x: 0, y: 0 },
            noise_areas: HashMap::new(),
        }
    }
}

impl MapBuilder for MazeBuilder {
    fn build_map(&mut self) {
        assert!(i32::checked_mul(self.map.width, self.map.height) != None);

        //generate maze copies the generated maze to the map of argument, "self" in this case
        let mut rng = RandomNumberGenerator::new();
        Grid::new(
            self.map.width / 2 - EDGE_BUFFER,
            self.map.height / 2 - EDGE_BUFFER,
            &mut rng,
        )
        .generate_maze(self);
        self.starting_position = Position {
            x: EDGE_BUFFER,
            y: EDGE_BUFFER,
        };

        let exit_tile = self
            .map
            .xy_idx(self.map.width - EDGE_BUFFER, self.map.height - EDGE_BUFFER);
        self.map.tiles[exit_tile] = TileType::StairsDown;
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
enum CellStatus {
    LeftWall = 0,
    RightWall,
    TopWall,
    BottomWall,
    BeenVisited,
}

struct Grid<'a> {
    width: i32,
    height: i32,
    cells: Vec<u8>,
    backtrace: Vec<usize>,
    rng: &'a mut RandomNumberGenerator,
}

impl<'a> Grid<'a> {
    pub fn new(width: i32, height: i32, rng: &mut RandomNumberGenerator) -> Grid {
        Grid {
            width,
            height,
            cells: vec![0b1111; (width * height) as usize], //0b1111 represents a cell that has all four walls
            backtrace: Vec::new(),
            rng,
        }
    }

    fn generate_maze(&mut self, generator: &mut MazeBuilder) {
        let mut current = 0;
        loop {
            Self::set_cell_status(&mut self.cells[current], CellStatus::BeenVisited);
            if let Some(next) = self.find_next_cell(current) {
                Self::set_cell_status(&mut self.cells[next], CellStatus::BeenVisited);
                self.backtrace.push(current);

                //Because of the borrower rules, you cannot have mut access to two elements in
                //an array at the same time, and because of this, you have to temporarily split the
                //array in two parts, and then access one from both parts.
                let max_idx = std::cmp::max(current, next);
                let min_idx = std::cmp::min(current, next);
                let (lo, hi) = self.cells.split_at_mut(max_idx);
                let cell_1 = &mut lo[min_idx];
                let cell_2 = &mut hi[0]; //Grabs cell at max_idx
                Self::connect_cells(
                    cell_1,
                    min_idx as i32 % self.width,
                    min_idx as i32 / self.width,
                    cell_2,
                    max_idx as i32 % self.width,
                    max_idx as i32 / self.width,
                );
                current = next;
            } else if !self.backtrace.is_empty() {
                current = self.backtrace.remove(0);
            } else {
                break;
            }
        }
        self.copy_to_map(&mut generator.map);
    }

    fn find_next_cell(&mut self, current: usize) -> Option<usize> {
        let neighbors = self.get_neighbors(current);
        if !neighbors.is_empty() {
            if neighbors.len() == 1 {
                return Some(neighbors[0]);
            } else {
                return Some(
                    neighbors[(self.rng.roll_dice(1, neighbors.len() as i32) - 1) as usize],
                );
            }
        }
        None
    }

    fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let col = index as i32 % self.width;
        let row = index as i32 / self.width;
        let neighbor_indices: [(i32, i32); 4] = [
            (col - 1, row),
            (col + 1, row),
            (col, row - 1),
            (col, row + 1),
        ];

        for neighbor in neighbor_indices.iter() {
            if self.in_bounds(*neighbor) {
                let neigh_idx = (neighbor.0 + self.width * neighbor.1) as usize;
                if !Self::is_cell_status_set(&self.cells[neigh_idx], CellStatus::BeenVisited) {
                    neighbors.push(neigh_idx);
                }
            }
        }
        neighbors
    }

    fn in_bounds(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn copy_to_map(&self, map: &mut Map) {
        for (idx, cell) in self.cells.iter().enumerate() {
            let x = idx as i32 % self.width + 1;
            let y = idx as i32 / self.width + 1;
            //In a grid, cells have walls. This isn't true in maps. To get the additional room, we
            //double the x and y
            let map_idx = map.xy_idx(x * 2, y * 2);

            map.tiles[map_idx] = TileType::Floor;
            if !Self::is_cell_status_set(cell, CellStatus::LeftWall) {
                map.tiles[map_idx - 1] = TileType::Floor;
            }
            if !Self::is_cell_status_set(cell, CellStatus::RightWall) {
                map.tiles[map_idx + 1] = TileType::Floor;
            }
            if !Self::is_cell_status_set(cell, CellStatus::TopWall) {
                map.tiles[map_idx - map.width as usize] = TileType::Floor;
            }
            if !Self::is_cell_status_set(cell, CellStatus::BottomWall) {
                map.tiles[map_idx + map.width as usize] = TileType::Floor;
            }
        }
    }

    //Static methods
    fn connect_cells(
        start: &mut u8,
        col_s: i32,
        row_s: i32,
        next: &mut u8,
        col_n: i32,
        row_n: i32,
    ) {
        match col_s - col_n {
            -1 => {
                Self::remove_cell_status(start, CellStatus::RightWall);
                Self::remove_cell_status(next, CellStatus::LeftWall);
            }
            1 => {
                Self::remove_cell_status(start, CellStatus::LeftWall);
                Self::remove_cell_status(next, CellStatus::RightWall);
            }
            0 => {}
            _ => unreachable!(),
        }
        match row_s - row_n {
            -1 => {
                Self::remove_cell_status(start, CellStatus::BottomWall);
                Self::remove_cell_status(next, CellStatus::TopWall);
            }
            1 => {
                Self::remove_cell_status(start, CellStatus::TopWall);
                Self::remove_cell_status(next, CellStatus::BottomWall);
            }
            0 => {}
            _ => unreachable!(),
        }
    }

    fn is_cell_status_set(cell: &u8, status: CellStatus) -> bool {
        (*cell & (1 << status as u8)) != 0
    }

    fn set_cell_status(cell: &mut u8, status: CellStatus) {
        *cell |= 1 << (status as u8);
    }

    fn remove_cell_status(cell: &mut u8, status: CellStatus) {
        *cell &= !(1 << status as u8);
    }
}
