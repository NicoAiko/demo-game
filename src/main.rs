use cgmath;
use ggez;
use ggez::graphics;
use ggez::graphics::{Font};
use ggez::{event, Context, GameResult};
use std::env;
use std::path;

struct GameState {
    font: Font,
    font_size: f32,
    frames: usize,
    characters_left: usize,
    text_length: usize,
    original_text: String,
}
impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/micross.ttf")?;
        let original_text = "Mach dir darüber keine Sorgen, das kriegen wir geregelt!\nHast du schon die Polizei gerufen?".to_string();
        let text_length = original_text.len();

        Ok(GameState { font, font_size: 48.0, frames: 0, text_length, characters_left: text_length, original_text })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let text: graphics::Text;
        let offset = self.frames as f32 / 10.0;
        let dest_point = cgmath::Point2::new(offset, offset);
        if self.characters_left == 0 as usize {
            let text_content = self.original_text.clone();
            text = graphics::Text::new((text_content, self.font, self.font_size));
        } else {
            let text_len = self.text_length - (self.characters_left - 1);
            let text_content: String = self.original_text.chars().take(text_len).collect();
            text = graphics::Text::new((text_content, self.font, self.font_size));

            self.characters_left -= 1;
        }

        graphics::draw(ctx, &text, (dest_point,))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("demo_game", "Nico [NicoAiko] Linnemann")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Demo Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0))
        .build()?;

    let state = &mut GameState::new(ctx)?;

    event::run(ctx, events_loop, state)
}