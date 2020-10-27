use ggez::graphics::Image;
use ggez::Context;

pub struct Assets {
    pub black: Image,
    pub white: Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> Assets {
        let black = Image::new(ctx, "/black.png").unwrap();
        let white = Image::new(ctx, "/white.png").unwrap();

        Assets { black, white }
    }
}
