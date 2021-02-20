use super::{
    common::{apply_horizontal_tunnel, apply_room_to_map, apply_vertical_tunnel},
    map::{Map, TileType},
    rect, MapBuilder,
};
use crate::{components::Position, spawning::populate_room};
use specs::World;

const MAX_ROOMS: i32 = 30;
const MIN_SIZE: i32 = 6;
const MAX_SIZE: i32 = 10;

pub struct SimpleMapBuilder {
    map: Map,
    starting_position: Position,
    rooms: Vec<rect::Rect>,
}

impl SimpleMapBuilder {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        Self {
            map: Map::new(width, height, depth),
            starting_position: Position { x: 0, y: 0 },
            rooms: Vec::new(),
        }
    }
}

impl MapBuilder for SimpleMapBuilder {
    fn build_map(&mut self) {
        assert!(i32::checked_mul(self.map.width, self.map.height) != None);

        let mut rng = rltk::RandomNumberGenerator::new();
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, self.map.width - w - 1) - 1;
            let y = rng.roll_dice(1, self.map.height - h - 1) - 1;
            let new_room = rect::Rect::new(x, y, w, h);

            if !self.rooms.iter().any(|room| room.intersect(&new_room)) {
                apply_room_to_map(&mut self.map, &new_room);
                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                    }
                }
                self.rooms.push(new_room);
            }
        }

        //Apply stairs to center of last room
        let stairs_pos = self.rooms.last().unwrap().center();
        let stairs_idx = self.map.xy_idx(stairs_pos.0, stairs_pos.1);
        self.map.tiles[stairs_idx] = TileType::StairsDown;

        let start_pos = self.rooms[0].center();
        self.starting_position = Position {
            x: start_pos.0,
            y: start_pos.1,
        };
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for room in self.rooms.iter().skip(1) {
            populate_room(ecs, room);
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }
}
