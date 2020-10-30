use ggez::graphics::Image;
use ggez::Context;

const BLACK: &[u8; 10000] = include_bytes!("../resources/black.bmp");
const WHITE: &[u8; 10000] = include_bytes!("../resources/white.bmp");

pub struct Assets {
    pub black: Image,
    pub white: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        let black = Image::from_rgba8(ctx, 50, 50, BLACK).unwrap();
        let white = Image::from_rgba8(ctx, 50, 50, WHITE).unwrap();

        Assets { black, white }
    }
}
