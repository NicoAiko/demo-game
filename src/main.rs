use ggez;
use ggez::{ContextBuilder, event, GameResult};

mod systems;
mod states;

use states::game_state::GameState;
use systems::GameContextBuilder;

fn main() -> GameResult {
    let game_context_builder = GameContextBuilder::new();
    let (ctx, events_loop) = &mut game_context_builder
        .build()
        .expect("Expected the game to build");

    let state = &mut GameState::new(ctx)?;

    state.load_sequence(ctx);

    event::run(ctx, events_loop, state)
}