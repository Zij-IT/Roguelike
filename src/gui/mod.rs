use crate::constants::{colors, consoles};
use crate::ecs::CombatStats;
use crate::rex_assets;
use rltk::{Rltk, RGB};
use specs::{Entity, World, WorldExt};

mod game_over;
mod inventory;
mod main_menu;
mod settings;
mod targeting;

pub use game_over::*;
pub use inventory::*;
pub use main_menu::*;
pub use settings::*;
pub use targeting::*;

pub fn show_hud(ecs: &World, ctx: &mut Rltk) {
    let assets = ecs.fetch::<rex_assets::RexAssets>();
    ctx.set_active_console(consoles::HUD_CONSOLE);
    ctx.render_xp_sprite(&assets.blank_ui, 0, 0);

    //Show player health
    let player_entity = ecs.fetch::<Entity>();
    let combat_stats = ecs.read_component::<CombatStats>();

    if let Some(players_stats) = combat_stats.get(*player_entity) {
        let print_x = 62;
        let base_x = 68;
        let base_y = 1;
        //Show health
        ctx.print_color(
            print_x,
            base_y,
            RGB::named(colors::FOREGROUND),
            RGB::named(colors::BACKGROUND),
            format!("{}/{}", players_stats.hp, players_stats.max_hp),
        );

        //Show health bars
        let ratio = 10.0 * (players_stats.hp as f32 / players_stats.max_hp as f32);
        for i in 0..10 {
            let foreground = if i < (ratio as i32) {
                RGB::named(rltk::GREEN)
            } else {
                RGB::named(rltk::RED)
            };
            ctx.set(
                base_x + i,
                base_y,
                foreground,
                RGB::named(colors::BACKGROUND),
                61,
            );
        }
    }
}
