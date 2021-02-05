use rltk::{Algorithm2D, BaseMap, Point, Rltk, RGB};
use serde::{Deserialize, Serialize};
use specs::prelude::*;

pub const MAP_HEIGHT: i32 = 43;
pub const MAP_WIDTH: i32 = 80;

//Tile Statuses
pub const TILE_REVEALED: u8 = 0;
pub const TILE_VISIBLE: u8 = 1;
pub const TILE_BLOCKED: u8 = 2;

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
    pub fn new(new_depth: i32) -> Map {
        Map {
            tiles: vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize],
            tile_status: vec![0; (MAP_WIDTH * MAP_HEIGHT) as usize],
            tile_content: vec![Vec::new(); (MAP_WIDTH * MAP_HEIGHT) as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            depth: new_depth,
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn populate_blocked(&mut self) {
        for idx in 0..self.tiles.len() {
            if self.tiles[idx] == TileType::Wall {
                self.set_tile_status(idx, TILE_BLOCKED);
            } else {
                self.remove_tile_status(idx, TILE_BLOCKED);
            }
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    //100 = blocked, 010 = visible, 001 = revealed
    pub fn is_tile_status_set(&self, idx: usize, status: u8) -> bool {
        (self.tile_status[idx] & (1 << status)) != 0
    }

    pub fn set_tile_status(&mut self, idx: usize, status: u8) {
        self.tile_status[idx] |= 1 << (status);
    }

    pub fn remove_tile_status(&mut self, idx: usize, status: u8) {
        self.tile_status[idx] &= !(1 << status);
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.is_tile_status_set(idx, TILE_BLOCKED)
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
        let w = self.width as usize;

        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        exits
    }
}

///Prints out the map to the rltk::Console
pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    for (pos, tile) in map.tiles.iter().enumerate() {
        if map.is_tile_status_set(pos, TILE_REVEALED) {
            let x = pos as i32 % map.width;
            let y = pos as i32 / map.width;

            let (glyph, mut fg) = match tile {
                TileType::Wall => (35, RGB::from_f32(0.0, 1.0, 0.0)),
                TileType::Floor => (46, RGB::from_f32(0., 0.25, 0.)),
                TileType::StairsDown => (174, RGB::from_f32(0., 1.0, 1.0)),
            };

            if !map.is_tile_status_set(pos, TILE_VISIBLE) {
                fg = fg.to_greyscale();
            }

            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }
    }
}
