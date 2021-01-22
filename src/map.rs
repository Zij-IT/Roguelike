use rltk::{ RGB, Rltk, Point, Algorithm2D, BaseMap };
use super::{rect};
use specs::prelude::*;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>,
    pub rooms : Vec<rect::Rect>,
    pub width : i32,
    pub height : i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn apply_room_to_map(&mut self, room: &rect::Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x,y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in std::cmp::min(x1, x2) ..= std::cmp::max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if (0..(self.width*self.height)).contains(&(idx as i32)) {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in std::cmp::min(y1, y2) ..= std::cmp::max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if (0..(self.width*self.height)).contains(&(idx as i32)) {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    ///Creates a pretty looking map made of rooms connected by corridors
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles : vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize], 
            revealed_tiles : vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            visible_tiles : vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            rooms : vec![],
            width : MAP_WIDTH,
            height : MAP_HEIGHT,
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
                    }
                    else {
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    }
                }

                map.rooms.push(new_room);
            } 
        }
        map
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

///Goes through each TileType and lays out the following characters
pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
    let map = ecs.fetch::<Map>();

    for (pos, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[pos] {
            let x = pos as i32 % map.width;
            let y = pos as i32 / map.width;

            match tile {
                TileType::Floor => {},
                TileType::Wall => {
                    let fg = { 
                        let mut fg = RGB::from_f32(0.0, 1.0, 0.0);
                        if !map.visible_tiles[pos] {
                            fg = fg.to_greyscale();
                        } 
                        fg
                    };                    
                    ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
                },
            }
        }
    }
}
