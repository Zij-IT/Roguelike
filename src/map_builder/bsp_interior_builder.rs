use super::{common::EDGE_BUFFER, Map, MapBuilder, Position, Rect, TileType};
use crate::spawner::populate_room;
use rltk::RandomNumberGenerator;
use specs::World;

const MIN_ROOM_SIZE: i32 = 8;

pub struct BSPInteriorBuilder {
    map: Map,
    starting_position: Position,
    rects: Vec<Rect>,
    rooms: Vec<Rect>,
}

impl BSPInteriorBuilder {
    pub fn new(width: i32, height: i32, new_depth: i32) -> BSPInteriorBuilder {
        BSPInteriorBuilder {
            map: Map::new(width, height, new_depth),
            starting_position: Position { x: 0, y: 0 },
            rects: Vec::new(),
            rooms: Vec::new(),
        }
    }

    pub fn add_subrects(&mut self, rect: Rect, rng: &mut RandomNumberGenerator) {
        let width = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;

        if rng.roll_dice(1, 4) <= 2 {
            //Horizontal
            let half_width = width / 2;
            let h1 = Rect::new(rect.x1, rect.y1, half_width - 1, height);
            let h2 = Rect::new(rect.x1 + half_width, rect.y1, half_width, height);

            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h1, rng);
                self.add_subrects(h2, rng);
            } else {
                self.rects.push(h1);
                self.rects.push(h2);
            }
        } else {
            let half_height = height / 2;
            let v1 = Rect::new(rect.x1, rect.y1, width, half_height - 1);
            let v2 = Rect::new(rect.x1, rect.y1 + half_height, width, half_height);

            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v1, rng);
                self.add_subrects(v2, rng);
            } else {
                self.rects.push(v1);
                self.rects.push(v2);
            }
        }
    }

    fn draw_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            } else if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }
            let idx = self.map.xy_idx(x, y);
            self.map.tiles[idx] = TileType::Floor;
        }
    }
}

impl MapBuilder for BSPInteriorBuilder {
    fn build_map(&mut self) {
        assert!(i32::checked_mul(self.map.width, self.map.height) != None);

        let mut rng = RandomNumberGenerator::new();
        self.rects.clear();

        let first_room = Rect::new(
            EDGE_BUFFER,
            EDGE_BUFFER,
            self.map.width - EDGE_BUFFER * 2,
            self.map.height - EDGE_BUFFER * 2,
        );
        self.add_subrects(first_room, &mut rng);

        for room in self.rects.clone().iter() {
            self.rooms.push(*room);
            //Slightly different from apply_room_to_map
            for y in room.y1..room.y2 {
                for x in room.x1..room.x2 {
                    let idx = self.map.xy_idx(x, y);
                    self.map.tiles[idx] = TileType::Floor;
                }
            }
        }

        for i in 0..self.rooms.len() - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2)) - 1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2)) - 1);
            let end_x =
                next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2)) - 1);
            let end_y =
                next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2)) - 1);
            self.draw_corridor(start_x, start_y, end_x, end_y);
        }

        //Get stairs in!
        let stairs = self.rooms[self.rooms.len() - 1].center();
        let stairs_idx = self.map.xy_idx(stairs.0, stairs.1);
        self.map.tiles[stairs_idx] = TileType::StairsDown;

        // Set player start
        let start = self.rooms[0].center();
        self.starting_position = Position {
            x: start.0,
            y: start.1,
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
