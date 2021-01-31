use super::rect;
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
    pub rooms: Vec<rect::Rect>,
    pub width: i32,
    pub height: i32,
    pub depth: i32,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new_map_rooms_and_corridors(new_depth: i32) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize],
            tile_status: vec![0; (MAP_WIDTH * MAP_HEIGHT) as usize],
            tile_content: vec![Vec::new(); (MAP_WIDTH * MAP_HEIGHT) as usize],
            rooms: vec![],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            depth: new_depth,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = rltk::RandomNumberGenerator::new();
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = rect::Rect::new(x, y, w, h);

            if !map.rooms.iter().any(|room| room.intersect(&new_room)) {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        //Apply stairs to center of last room
        let stairs_pos = map.rooms.last().unwrap().center();
        let stairs_idx = map.xy_idx(stairs_pos.0, stairs_pos.1);
        map.tiles[stairs_idx] = TileType::StairsDown;

        map
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

    fn apply_room_to_map(&mut self, room: &rect::Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in std::cmp::min(x1, x2)..=std::cmp::max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if (0..(self.width * self.height)).contains(&(idx as i32)) {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in std::cmp::min(y1, y2)..=std::cmp::max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if (0..(self.width * self.height)).contains(&(idx as i32)) {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
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
pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for (pos, tile) in map.tiles.iter().enumerate() {
        if map.is_tile_status_set(pos, TILE_REVEALED) {
            let x = pos as i32 % map.width;
            let y = pos as i32 / map.width;

            let (glyph, mut fg) = match tile {
                TileType::Wall => ('#', RGB::from_f32(0.0, 1.0, 0.0)),
                TileType::Floor => ('.', RGB::from_f32(0., 0.25, 0.)),
                TileType::StairsDown => ('>', RGB::from_f32(0., 1.0, 1.0)),
            };

            if !map.is_tile_status_set(pos, TILE_VISIBLE) {
                fg = fg.to_greyscale();
            }

            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), rltk::to_cp437(glyph));
        }
    }
}
