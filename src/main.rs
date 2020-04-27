use ggez;
use ggez::{event, GameResult};
use std::env;
use std::path;

mod systems;
mod states;

use states::game_state::GameState;

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
        .build()
        .expect("Expected the game to build!");

    let state = &mut GameState::new(ctx)?;
    state.load_sequence(ctx);

    event::run(ctx, events_loop, state)
}