use rltk::{Algorithm2D, BaseMap, Point};
use serde::{Deserialize, Serialize};
use specs::prelude::*;

//Tile Statuses
pub enum TileStatus {
    Revealed = 0,
    Visible,
    Blocked,
}

#[derive(PartialEq, Copy, Clone, Deserialize, Serialize)]
pub enum TileType {
    Floor,
    StairsDown,
    Wall,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub tile_status: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        Self {
            tiles: vec![TileType::Wall; (width * height) as usize],
            tile_status: vec![0; (width * height) as usize],
            tile_content: vec![Vec::new(); (width * height) as usize],
            width,
            height,
            depth,
        }
    }

    pub const fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn populate_blocked(&mut self) {
        for idx in 0..self.tiles.len() {
            if self.tiles[idx] == TileType::Wall {
                self.set_tile_status(idx, TileStatus::Blocked);
            } else {
                self.remove_tile_status(idx, TileStatus::Blocked);
            }
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in &mut self.tile_content {
            content.clear();
        }
    }

    //100 = blocked, 010 = visible, 001 = revealed
    pub fn is_tile_status_set(&self, idx: usize, status: TileStatus) -> bool {
        (self.tile_status[idx] & (1 << status as u8)) != 0
    }

    pub fn set_tile_status(&mut self, idx: usize, status: TileStatus) {
        self.tile_status[idx] |= 1 << (status as u8);
    }

    pub fn remove_tile_status(&mut self, idx: usize, status: TileStatus) {
        self.tile_status[idx] &= !(1 << status as u8);
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.is_tile_status_set(idx, TileStatus::Blocked)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x < self.width && pos.y < self.height && pos.x >= 0 && pos.y >= 0
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall => true,
            TileType::StairsDown | TileType::Floor => false,
        }
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);

        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();

        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if self.is_exit_valid(x + dx, y + dy) {
                    let distance: f32 = match dx * dx + dy * dy {
                        0 => continue,
                        1 => 1.0,
                        2 => 1.45,
                        _ => {
                            unreachable!()
                        },
                    };
                    let offset_index = (idx as i32 + dx + self.width * dy) as usize; //Safe because of is_exit_valid
                    exits.push((offset_index, distance))
                }
            }
        }

        exits
    }
}
