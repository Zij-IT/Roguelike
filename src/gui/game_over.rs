use crate::constants::{colors, consoles};
use rltk::{Rltk, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoResponse,
    QuitToMenu,
}

pub fn show_game_over(ctx: &mut Rltk) -> GameOverResult {
    for i in 0..consoles::NUM_OF_CONSOLES {
        ctx.set_active_console(i);
        ctx.cls();
    }

    ctx.set_active_console(consoles::HUD_CONSOLE);

    let lines = [
        "Your journey has ended!",
        "One day, we'll tell you all about how you did.",
        "That day, sadly, is not in this chapter..",
        "Press any key to return to the menu.",
    ];

    let y_base = 15;
    let step = 2;
    for (index, line) in lines.iter().enumerate() {
        ctx.print_color_centered(
            y_base + step * index,
            RGB::from(colors::FOREGROUND),
            RGB::from(colors::BACKGROUND),
            line,
        );
    }

    match ctx.key {
        None => GameOverResult::NoResponse,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
