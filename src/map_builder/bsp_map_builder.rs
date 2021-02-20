use super::{
    common::{apply_room_to_map, connect_rooms_via_corridors, EDGE_BUFFER},
    map::{Map, TileType},
    rect::Rect,
    MapBuilder,
};
use crate::{components::Position, spawning::populate_room};
use rltk::RandomNumberGenerator;
use specs::World;

const MAX_ATTEMPTS: usize = 240;

pub struct BSPMapBuilder {
    map: Map,
    starting_position: Position,
    rects: Vec<Rect>,
    rooms: Vec<Rect>,
}

impl BSPMapBuilder {
    pub fn new(width: i32, height: i32, new_depth: i32) -> Self {
        Self {
            map: Map::new(width, height, new_depth),
            starting_position: Position { x: 0, y: 0 },
            rects: Vec::new(),
            rooms: Vec::new(),
        }
    }
}

impl MapBuilder for BSPMapBuilder {
    fn build_map(&mut self) {
        assert!(i32::checked_mul(self.map.width, self.map.height) != None);
        let mut rng = RandomNumberGenerator::new();

        self.rects.clear();
        self.rects.push(Rect::new(
            EDGE_BUFFER,
            EDGE_BUFFER,
            self.map.width - EDGE_BUFFER,
            self.map.height - EDGE_BUFFER,
        ));
        let first_room = self.rects[0];
        self.add_sub_rects(first_room);
        for _ in 0..MAX_ATTEMPTS {
            let rect = self.get_random_rect(&mut rng);
            let candidate = Self::get_random_sub_rect(rect, &mut rng);

            if self.is_possible(candidate) {
                apply_room_to_map(&mut self.map, &candidate);
                self.rooms.push(candidate);
                self.add_sub_rects(rect);
            }
        }

        //Sort left to right
        self.rooms.sort_by(|a, b| a.x1.cmp(&b.x1));

        connect_rooms_via_corridors(&mut self.map, &self.rooms, &mut rng);

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

impl BSPMapBuilder {
    fn add_sub_rects(&mut self, rect: Rect) {
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

    fn get_random_sub_rect(rect: Rect, rng: &mut RandomNumberGenerator) -> Rect {
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
}
