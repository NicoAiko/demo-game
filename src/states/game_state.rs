use cgmath;
use ggez::{
    graphics::{
        self,
        Font,
        Align,
    },
    GameResult,
    Context,
    event,
};
use std::f32;

use crate::systems::FPSState;

pub struct GameState {
    fps_state: FPSState,
    font: Font,
    font_size: f32,
    characters_left: usize,
    text_length: usize,
    original_text: String,
}
impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/micross.ttf")?;
        let original_text = "Mach dir darüber keine Sorgen, das kriegen wir geregelt!\nHast du schon die Polizei gerufen?".to_string();
        let text_length = original_text.len();
        let fps_state = FPSState::new(font, 24.0).expect("Couldn't create FPS State Manager");

        Ok(GameState { fps_state, font, font_size: 24.0, text_length, characters_left: text_length, original_text })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.fps_state.draw(ctx).expect("FPS Draw failed");

        let mut text: graphics::Text;
        let (width, height) = graphics::drawable_size(ctx);
        let dest_point = cgmath::Point2::new(20.0, height - 80.0);
        if self.characters_left == 0 as usize {
            let text_content = self.original_text.clone();
            text = graphics::Text::new((text_content, self.font, self.font_size));
        } else {
            let text_len = self.text_length - (self.characters_left - 1);
            let text_content: String = self.original_text.chars().take(text_len).collect();
            text = graphics::Text::new((text_content, self.font, self.font_size));

            self.characters_left -= 1;
        }

        text.set_bounds(cgmath::Point2::new(width - 40.0, f32::INFINITY), Align::Center);

        graphics::draw(ctx, &text, (dest_point,))?;

        graphics::present(ctx)?;
        Ok(())
    }
}