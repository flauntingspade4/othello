use crate::assets::Assets;

use ggez::{
    graphics::{Color, DrawParam},
    nalgebra::Point2,
    Context,
};

pub struct Board {
    pub board: [[Option<Counter>; 8]; 8],
    pub turn: bool,
    assets: Assets,
    remaining: usize,
}

impl Board {
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
}

impl ggez::event::EventHandler for Board {
    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {
        // If the board is empty
        if self.remaining == 0 {
            let (white, black) =
                self.board
                    .iter()
                    .fold((0, 0), |(mut white, mut black), next_row| {
                        for counter in next_row.iter() {
                            match counter.unwrap() {
                                Counter::Black => black += 1,
                                Counter::White => white += 1,
                            }
                        }
                        (white, black)
                    });
            match white.cmp(&black) {
                std::cmp::Ordering::Less => println!("Black won!"),
                std::cmp::Ordering::Equal => println!("White won!"),
                std::cmp::Ordering::Greater => println!("It was a draw!"),
            }
            ctx.continuing = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, Color::from_rgb(0, 205, 0));

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

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        let x = (x / 50.) as usize;
        let y = (y / 50.) as usize;
        if self.board[x][y].is_some() {
            println!("Some");
        } else {
            let counter = Counter::from(self.turn);
            self.board[x][y] = Some(counter);
            self.turn = !self.turn;
            self.remaining -= 1;
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Counter {
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
