use rltk::{ RGB, Rltk };
use super::rect;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH + x) as usize
}

///Creates a map with solid boundries and up to 400 randomly placed walls.
///Its not pretty :P
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (MAP_HEIGHT * MAP_WIDTH) as usize];

    // Make the boundaries walls
    for x in 0..MAP_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, MAP_HEIGHT - 1)] = TileType::Wall;
    }
    for y in 0..MAP_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(MAP_WIDTH - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..(MAP_HEIGHT * MAP_WIDTH) {
        let x = rng.roll_dice(1, MAP_WIDTH - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(MAP_WIDTH / 2, MAP_HEIGHT / 2) && rng.roll_dice(1, 4) < 2 {
            map[idx] = TileType::Wall;
        }
    }

    map
} 

fn apply_room_to_map(room: &rect::Rect, map: &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x,y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in std::cmp::min(x1, x2) ..= std::cmp::max(x1, x2) {
        let idx = xy_idx(x, y);
        if (0..MAP_WIDTH*MAP_HEIGHT).contains(&(idx as i32)) {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x1: i32) {
    for y in std::cmp::min(y1, y2) ..= std::cmp::max(y1, y2) {
        let idx = xy_idx(x1, y);
        if (0..MAP_WIDTH*MAP_HEIGHT).contains(&(idx as i32)) {
            map[idx as usize] = TileType::Floor;
        }
    }
}

///Creates a pretty looking map made of rooms connected by corridors
pub fn new_map_rooms_and_corridors() -> (Vec<TileType>, Vec<rect::Rect>) {
    let mut map = vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize];
    let mut rooms : Vec<rect::Rect> = Vec::new();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, MAP_WIDTH - w - 1) - 1;
        let y = rng.roll_dice(1, MAP_HEIGHT - h - 1) - 1;

        let new_room = rect::Rect::new(x, y, w, h);
        if !rooms.iter().any(|room| room.intersect(&new_room)) {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                }
                else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                }
            }

            rooms.push(new_room);
        } 
    }

    (map, rooms)
}

///Goes through each TileType and lays out the following characters
pub fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    map.iter().enumerate().for_each(|(pos, tile)| {
        let x = pos as i32 % MAP_WIDTH;
        let y = pos as i32 / MAP_WIDTH;
        match tile {
            TileType::Floor => {},
            TileType::Wall => {
                ctx.set(
                    x, 
                    y, 
                    RGB::from_f32(0.0, 1.0, 0.0), 
                    RGB::from_f32(0., 0., 0.), 
                    rltk::to_cp437('#')
                );
            },
        }
    });
}
