use ggez::{self, ContextBuilder};
use std::{env, path};
use ggez::conf::NumSamples;

pub struct GameContextBuilder {}

impl GameContextBuilder {
    pub fn new() -> ContextBuilder {
        let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        } else {
            path::PathBuf::from("./resources")
        };

        ggez::ContextBuilder::new("demo_game", "Nico [NicoAiko] Linnemann")
            .add_resource_path(resource_dir)
            .window_setup(ggez::conf::WindowSetup::default().title("Demo Game").samples(NumSamples::Sixteen))
            .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0))
    }
}