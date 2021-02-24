use crate::{
    constants::{colors, consoles},
    ecs::Viewshed,
    raws::config::CONFIGS,
    {camera, EcsWorld},
};
use rltk::{Point, Rltk, RGB};
use specs::{Entity, WorldExt};

#[derive(PartialEq, Copy, Clone)]
pub enum TargetResult {
    Cancel,
    NoResponse,
    Selected(Point),
}

pub fn show_targeting(gs: &mut EcsWorld, ctx: &mut Rltk, range: i32) -> TargetResult {
    let player_ent = gs.world.fetch::<Entity>();
    let player_pos = gs.world.fetch::<Point>();
    let views = gs.world.read_storage::<Viewshed>();
    let (min_x, max_x, min_y, max_y) = camera::get_screen_bounds(&gs.world);

    ctx.set_active_console(consoles::MAP_CONSOLE);

    ctx.print_color(
        5,
        0,
        RGB::named(rltk::YELLOW),
        RGB::from(colors::BACKGROUND),
        "Select Target: ",
    );

    let mut available_cells = Vec::new();
    if let Some(visible) = views.get(*player_ent) {
        for idx in &visible.visible_tiles {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if distance < range as f32 {
                let screen_x = idx.x - min_x;
                let screen_y = idx.y - min_y;
                if screen_x > 1
                    && screen_x < max_x - min_x - 1
                    && screen_y > 1
                    && screen_y < max_y - min_y - 1
                {
                    ctx.set_bg(screen_x, screen_y, RGB::named(rltk::BLUE));
                    available_cells.push(idx);
                }
            }
        }
    } else {
        return TargetResult::Cancel;
    }

    //Draw Cursor
    let true_mouse_pos = ctx.mouse_pos();
    let mouse_pos = { (true_mouse_pos.0 + min_x, true_mouse_pos.1 + min_y) };
    if ctx.left_click {
        return if available_cells
            .iter()
            .any(|tile| tile.x == mouse_pos.0 && tile.y == mouse_pos.1)
        {
            ctx.set_bg(true_mouse_pos.0, true_mouse_pos.1, RGB::named(rltk::CYAN));
            TargetResult::Selected(Point::new(mouse_pos.0, mouse_pos.1))
        } else {
            ctx.set_bg(true_mouse_pos.0, true_mouse_pos.1, RGB::named(rltk::RED));
            TargetResult::Cancel
        };
    }

    if Some(CONFIGS.lock().unwrap().keys.go_back) == ctx.key {
        return TargetResult::Cancel;
    }

    TargetResult::NoResponse
}
