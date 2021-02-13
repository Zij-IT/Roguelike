use super::{common::*, map::*, rect::Rect, MapBuilder};
use crate::{spawner::populate_room, Position};
use rltk::RandomNumberGenerator;
use specs::World;

const MAX_ATTEMPTS: usize = 240;
const EDGE_BUFFER: i32 = 2;

pub struct BSPMapBuilder {
    map: Map,
    starting_position: Position,
    history: Vec<Map>,
    rects: Vec<Rect>,
    rooms: Vec<Rect>,
}

impl BSPMapBuilder {
    pub fn new(new_depth: i32) -> BSPMapBuilder {
        BSPMapBuilder {
            map: Map::new(new_depth),
            starting_position: Position { x: 0, y: 0 },
            history: Vec::new(),
            rects: Vec::new(),
            rooms: Vec::new(),
        }
    }
}

impl MapBuilder for BSPMapBuilder {
    fn build_map(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        self.rects.clear();
        self.rects.push(Rect::new(
            EDGE_BUFFER,
            EDGE_BUFFER,
            self.map.width - EDGE_BUFFER,
            self.map.height - EDGE_BUFFER,
        ));
        let first_room = self.rects[0];
        self.add_subrects(first_room);
        for _ in 0..MAX_ATTEMPTS {
            let rect = self.get_random_rect(&mut rng);
            let candidate = self.get_random_sub_rect(rect, &mut rng);

            if self.is_possible(candidate) {
                apply_room_to_map(&mut self.map, &candidate);
                self.rooms.push(candidate);
                self.add_subrects(rect);
                self.take_snapshot();
            }
        }

        //Sort left to right
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));

        //Connect rooms via corridors
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
            self.take_snapshot();
        }

        //Get stairs in!
        let stairs = self.rooms[self.rooms.len() - 1].center();
        let stairs_idx = self.map.xy_idx(stairs.0, stairs.1);
        self.map.tiles[stairs_idx] = TileType::StairsDown;
        self.take_snapshot();

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

    fn take_snapshot(&mut self) {
        if crate::SHOW_MAPGEN {
            let mut snapshot = self.get_map();
            for tile in 0..snapshot.tile_status.len() {
                snapshot.set_tile_status(tile, TileStatus::Revealed);
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

impl BSPMapBuilder {
    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.x1 - rect.x2);
        let height = i32::abs(rect.y1 - rect.y2);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);

        self.rects
            .push(Rect::new(rect.x1, rect.y1, half_width, half_height));
        self.rects.push(Rect::new(
            rect.x1,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1,
            half_width,
            half_height,
        ));
        self.rects.push(Rect::new(
            rect.x1 + half_width,
            rect.y1 + half_height,
            half_width,
            half_height,
        ));
    }

    fn get_random_rect(&mut self, rng: &mut RandomNumberGenerator) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = (rng.roll_dice(1, self.rects.len() as i32) - 1) as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, rect: Rect, rng: &mut RandomNumberGenerator) -> Rect {
        let rect_width = i32::abs(rect.x1 - rect.x2);
        let rect_height = i32::abs(rect.y1 - rect.y2);

        let w = i32::max(3, rng.roll_dice(1, i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, rng.roll_dice(1, i32::min(rect_height, 10)) - 1) + 1;

        let rand_width = rect.x1 + rng.roll_dice(1, 6);
        let rand_height = rect.y1 + rng.roll_dice(1, 6);

        Rect {
            x1: rand_width,
            x2: rand_width + w,
            y1: rand_height,
            y2: rand_height + h,
        }
    }

    fn is_possible(&self, rect: Rect) -> bool {
        let expanded = Rect {
            x1: rect.x1 - 2,
            x2: rect.x2 + 2,
            y1: rect.y1 - 2,
            y2: rect.y2 + 2,
        };

        for y in expanded.y1..=expanded.y2 {
            for x in expanded.x1..=expanded.x2 {
                if y < 1 || x < 1 || y > self.map.height - 2 || x > self.map.width - 2 {
                    return false;
                }

                let idx = self.map.xy_idx(x, y);
                if self.map.tiles[idx] != TileType::Wall {
                    return false;
                }
            }
        }

        true
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
