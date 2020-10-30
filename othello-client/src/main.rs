mod assets;
mod board;
mod board_online;
mod util;

use std::io::Read;

use board::Board;
use board_online::BoardOnline;

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

    println!(
        "Press 'r' to restart the game\nPress 's' to skip your go\nPress 'esc' to end the game"
    );

    let env: Vec<String> = std::env::args().collect();
    if env.len() != 1 {
        match env[1].to_ascii_lowercase().as_str() {
            "-o" => {
                let team;
                let stream;
                loop {
                    let mut ip = String::new();
                    match std::io::stdin().read_line(&mut ip) {
                        Ok(_) => match util::turn_to_nums(&ip.trim().to_string()) {
                            Ok(ip) => match util::connect(ip) {
                                Ok(mut temp_stream) => {
                                    let mut buf = [0];
                                    temp_stream.read_exact(&mut buf)?;
                                    team = buf[0];
                                    stream = temp_stream;
                                    break;
                                }
                                Err(e) => println!("{}", e),
                            },
                            Err(e) => println!("{}", e),
                        },
                        Err(e) => println!("{}", e),
                    }
                }

                let mut state = BoardOnline::new(ctx, team, stream);
                event::run(ctx, events_loop, &mut state).unwrap();
            }
            _ => {
                let mut state = Board::new(ctx);

                event::run(ctx, events_loop, &mut state).unwrap();
            }
        }
    } else {
        let mut state = Board::new(ctx);

        event::run(ctx, events_loop, &mut state).unwrap();
    }

    Ok(())
}
