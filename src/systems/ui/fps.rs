use cgmath;
use ggez::{
    self,
    Context,
    GameResult,
    timer,
    graphics::{
        self,
        Text,
        Font
    },
};

pub struct FPSState {
    font: Font,
    font_size: f32,
}

impl FPSState {
    pub fn new(font: Font, font_size: f32) -> GameResult<FPSState> {
        Ok(FPSState { font, font_size })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let text_content = format!("FPS: {:.2}", timer::fps(ctx));
        let text = Text::new((text_content, self.font, self.font_size));
        let (width, _) = graphics::drawable_size(ctx);
        let point = cgmath::Point2::new(width - 120.0, 10.0);

        graphics::draw(ctx, &text, (point,)).expect("FPS label could not be drawn to screen!");
        Ok(())
    }
}