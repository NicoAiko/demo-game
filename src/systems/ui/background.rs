use ggez::{
    graphics::{
        self,
        Image,
        DrawParam,
    },
    Context,
    GameResult,
};
use cgmath;
use std::collections::HashMap;

type Vector2 = cgmath::Vector2<f32>;

#[derive(Debug, PartialEq)]
struct ImageStoreItem {
    image: Image,
}

impl ImageStoreItem {
    fn new(ctx: &mut Context, path: &str) -> ImageStoreItem {
        if path.is_empty() {
            panic!("Expected an identifier and image path!");
        }

        let image = Image::new(ctx, path).expect("Image couldn't be loaded!");

        ImageStoreItem {
            image,
        }
    }
}

pub struct Background {
    image_store: HashMap<String, ImageStoreItem>,
    active_image: String,
}

impl Background {
    pub fn new() -> GameResult<Background> {
        let image_store = HashMap::new();
        let active_image = String::from("");

        Ok(Background { image_store, active_image })
    }

    pub fn load_image(&mut self, ctx: &mut Context, identifier: &str, path: &str) {
        if self.image_store.contains_key(identifier) {
            println!("[WARN] Interrupted trying to load image with duplicate identifier: {}", identifier);

            return;
        }

        let item = ImageStoreItem::new(ctx, path);

        self.image_store.insert(identifier.to_string(), item);
    }

    pub fn unload_image(&mut self, identifier: &str) {
        self.image_store.remove(identifier);
    }

    pub fn set_active_image(&mut self, identifier: &str) {
        if !self.image_store.contains_key(identifier) {
            println!("Couldn't find image \"{}\" in memory!", identifier);

            return;
        }

        self.active_image = identifier.to_string();
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if !self.active_image.is_empty() {
            let image_pos = cgmath::Point2::new(0.0, 0.0);
            let window_size = graphics::size(ctx);
            let image_item = self.image_store.get(&self.active_image).unwrap();
            let scale = Vector2::new(
                window_size.0 as f32 / image_item.image.width() as f32,
                window_size.1 as f32 / image_item.image.height() as f32,
            );

            graphics::draw(ctx, &image_item.image, DrawParam::default().dest(image_pos).scale(scale)).expect("Background image could not be drawn!");
        }

        Ok(())
    }
}