mod assets;
mod board;

use board::Board;

use ggez::{conf::*, event};

fn main() -> ggez::GameResult {
    let setup = WindowSetup {
        title: "Othello".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };
    let mode = WindowMode {
        width: 400.,
        height: 400.,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };
    let cb = ggez::ContextBuilder::new("Othello", "flauntingspade4");
    let (ctx, events_loop) = &mut cb.window_setup(setup).window_mode(mode).build()?;
    let state = &mut Board::new(ctx);

    println!(
        "Press 'r' to restart the game\nPress 's' to skip your go\nPress 'esc' to end the game"
    );

    event::run(ctx, events_loop, state)
}
