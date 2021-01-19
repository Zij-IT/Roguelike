use rltk::{ GameState, Rltk };
use specs::prelude::*;
use super::{
    components::{
        Position,
        Renderable
    },
    map::{ 
        TileType,
        draw_map,
    },
    player::{
        player_input,
    }
};

pub struct State {
    pub ecs: specs::prelude::World,
}

impl State {
    fn run_systems(&mut self) {
        //For all systems s in self.systems s.run_now
    } 
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        //Scans player input and tries to move player
        player_input(self, ctx);

        //Runs all non-player systems (for now)
        self.run_systems();

        //Draw map
        draw_map(&self.ecs.fetch::<Vec<TileType>>(), ctx);

        //Displays all entities with positions and renderables
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
