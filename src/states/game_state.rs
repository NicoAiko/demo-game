use cgmath;
use ggez::{
    graphics::{
        self,
        Font,
        Align,
    },
    GameResult,
    Context,
    event::{
        self,
        KeyCode,
        KeyMods,
    },
};
use std::f32;

use crate::systems::FPSState;
use crate::systems::Background;

pub struct GameState {
    fps_state: FPSState,
    background: Background,
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
        let background = Background::new()?;

        Ok(GameState { fps_state, background, font, font_size: 24.0, text_length, characters_left: text_length, original_text })
    }

    // For testing purposes
    pub fn load_sequence(&mut self, ctx: &mut Context) {
        // loading background images
        self.background.load_image(ctx, "test_1", "/test_image_1_compressed.jpg");
        self.background.load_image(ctx, "test_2", "/test_image_2.jpg");
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.background.draw(ctx).expect("Background Draw failed");
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

        text.set_bounds(cgmath::Point2::new(width - 40.0, f32::INFINITY), Align::Left);

        graphics::draw(ctx, &text, (dest_point,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key_code: KeyCode, _key_mods: KeyMods, _repeat: bool) {
        if key_code == KeyCode::Key1 {
            self.background.set_active_image("test_1");
        } else if key_code == KeyCode::Key2 {
            self.background.set_active_image("test_2");
        } else if key_code == KeyCode::Escape {
            std::process::exit(0);
        }
    }
}