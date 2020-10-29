use crate::assets::Assets;

use ggez::{
    event::{KeyCode, MouseButton},
    graphics::{Color, DrawParam},
    nalgebra::Point2,
    Context,
};

const POSSIBLE_DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

pub struct Board {
    board: [[Option<Counter>; 8]; 8],
    turn: bool,
    assets: Assets,
    remaining: usize,
}

impl Board {
    // Initiates a board with the default setup
    pub fn new(ctx: &mut Context) -> Self {
        let mut board = [[None; 8]; 8];
        board[3][3] = Some(Counter::White);
        board[4][3] = Some(Counter::Black);
        board[3][4] = Some(Counter::Black);
        board[4][4] = Some(Counter::White);
        Self {
            board,
            turn: false,
            assets: Assets::new(ctx),
            remaining: 8 * 8 - 4,
        }
    }
    // Counts up the amount of white and black tokens lain down
    fn count(&self) -> (usize, usize) {
        self.board
            .iter()
            .fold((0, 0), |(mut white, mut black), next_row| {
                for counter in next_row.iter() {
                    if let Some(counter) = counter {
                        match counter {
                            Counter::Black => black += 1,
                            Counter::White => white += 1,
                        }
                    }
                }
                (white, black)
            })
    }
    fn reset(&mut self) {
        self.board = [[None; 8]; 8];
        self.board[3][3] = Some(Counter::White);
        self.board[4][3] = Some(Counter::Black);
        self.board[3][4] = Some(Counter::Black);
        self.board[4][4] = Some(Counter::White);
        self.turn = false;
        self.remaining = 8 * 8 - 4;
    }
    fn end(&self) {
        let (white, black) = self.count();
        match white.cmp(&black) {
            std::cmp::Ordering::Less => println!("Black won!"),
            std::cmp::Ordering::Equal => println!("It was a draw!"),
            std::cmp::Ordering::Greater => println!("White won!"),
        }
    }
    fn place(&mut self, x: isize, y: isize, to_place: Counter) -> Vec<(usize, usize)> {
        let mut to_return = Vec::new();
        for (x_dir, y_dir) in POSSIBLE_DIRECTIONS.iter() {
            let mut temp_x = x + x_dir;
            let mut temp_y = y + y_dir;
            // If there's been a different colour counter since beginning
            let mut different = false;
            let mut temp_cache = Vec::new();

            while temp_x < 8 && temp_x >= 0 && temp_y < 8 && temp_y >= 0 {
                match self.board[temp_x as usize][temp_y as usize] {
                    Some(t) => {
                        if t == to_place {
                            if different {
                                to_return.append(&mut temp_cache);
                            } else {
                                break;
                            }
                        } else {
                            temp_cache.push((temp_x as usize, temp_y as usize));
                            different = true;
                        }
                    }
                    None => break,
                }
                temp_x += x_dir;
                temp_y += y_dir;
            }
        }
        to_return
    }
    fn next_turn(&mut self) {
        self.turn = !self.turn;
        println!("Now {}'s turn", Counter::from(self.turn));
    }
}

impl ggez::event::EventHandler for Board {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        // If the board is full
        if self.remaining == 0 {
            self.end();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, Color::from_rgb(0, 175, 0));

        for (x, row) in self.board.iter().enumerate() {
            for (y, counter) in row.iter().enumerate() {
                match counter {
                    Some(t) => match t {
                        Counter::Black => {
                            ggez::graphics::draw(
                                ctx,
                                &self.assets.black,
                                DrawParam::new().dest([x as f32 * 50., y as f32 * 50.]),
                            )
                            .unwrap();
                        }
                        Counter::White => {
                            ggez::graphics::draw(
                                ctx,
                                &self.assets.white,
                                DrawParam::new().dest(Point2::new(x as f32 * 50., y as f32 * 50.)),
                            )
                            .unwrap();
                        }
                    },
                    None => {}
                }
            }
        }
        ggez::graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let x = (x / 50.) as usize;
            let y = (y / 50.) as usize;
            if self.board[x][y].is_some() {
                println!("Already a counter there!");
            } else {
                let counter = Counter::from(self.turn);
                let to_place = self.place(x as isize, y as isize, counter);

                // If the move isn't valid
                if to_place.is_empty() {
                    println!("Invalid move");
                } else {
                    self.board[x][y] = Some(counter);
                    for (x, y) in to_place {
                        self.board[x][y] = Some(counter);
                    }
                    self.next_turn();
                    self.remaining -= 1;
                }
            }
        }
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::S => self.next_turn(),
            KeyCode::R => {
                self.end();
                self.reset();
            }
            KeyCode::Escape => ctx.continuing = false,
            _ => {}
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Counter {
    Black,
    White,
}

impl From<bool> for Counter {
    fn from(t: bool) -> Self {
        if t {
            Self::White
        } else {
            Self::Black
        }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Counter::Black => write!(f, "black"),
            Counter::White => write!(f, "white"),
        }
    }
}
