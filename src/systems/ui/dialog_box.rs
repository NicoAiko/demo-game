use ggez::{
    Context,
    GameResult,
    nalgebra,
    graphics::{
        self,
        DrawMode,
        DrawParam,
        Mesh,
        MeshBuilder,
        Font,
        Align,
        Color,
        Text,
    },
};
use cgmath::Vector2;
use std::f32;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;
const TOP_MULTIPLIER: f32 = 0.7;
const BOTTOM_MULTIPLIER: f32 = 0.95;
const LEFT_MULTIPLIER: f32 = 0.125;
const RIGHT_MULTIPLIER: f32 = 0.275;

pub struct DialogBox {
    actor_box_mesh: Mesh,
    dialog_box_mesh: Mesh,
    dialog_text: String,
    actor: String,
    font: Font,
    dialog_text_font_size: f32,
    dialog_text_alignment: Align,
    actor_text_font_size: f32,
}

impl DialogBox {
    pub fn new(ctx: &mut Context, dialog_font_size: f32, actor_font_size: f32) -> GameResult<DialogBox> {
        let font = Font::new(ctx, "/micross.ttf").expect("Font loading failed");

        let dialog_box_mesh: Mesh = MeshBuilder::new()
            .polyline(
                DrawMode::fill(),
                &[
                    nalgebra::Point2::new(0.0, HEIGHT * TOP_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH, HEIGHT * TOP_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH, HEIGHT * BOTTOM_MULTIPLIER),
                    nalgebra::Point2::new(0.0, HEIGHT * BOTTOM_MULTIPLIER),
                ],
                Color::new(1.0, 1.0, 1.0, 0.5),
            )?
            .line(
                &[
                    nalgebra::Point2::new(0.0, HEIGHT * TOP_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH, HEIGHT * TOP_MULTIPLIER),
                ],
                2.0,
                graphics::WHITE,
            )?
            .line(
                &[
                    nalgebra::Point2::new(0.0, HEIGHT * BOTTOM_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH, HEIGHT * BOTTOM_MULTIPLIER),
                ],
                5.0,
                graphics::WHITE,
            )?
            .build(ctx)?;

        let actor_box_mesh: Mesh = MeshBuilder::new()
            .polyline(
                DrawMode::fill(),
                &[
                    nalgebra::Point2::new(WIDTH * LEFT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER - 0.025)),
                    nalgebra::Point2::new(WIDTH * RIGHT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER - 0.025)),
                    nalgebra::Point2::new(WIDTH * (RIGHT_MULTIPLIER + 0.025), HEIGHT * TOP_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH * RIGHT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER + 0.025)),
                    nalgebra::Point2::new(WIDTH * LEFT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER + 0.025)),
                    nalgebra::Point2::new(WIDTH * (LEFT_MULTIPLIER - 0.025), HEIGHT * TOP_MULTIPLIER),
                ],
                Color::new(1.0, 1.0, 1.0, 0.5),
            )?
            .polyline(
                DrawMode::stroke(2.0),
                &[
                    nalgebra::Point2::new(WIDTH * LEFT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER - 0.025)),
                    nalgebra::Point2::new(WIDTH * RIGHT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER - 0.025)),
                    nalgebra::Point2::new(WIDTH * (RIGHT_MULTIPLIER + 0.025), HEIGHT * TOP_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH * RIGHT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER + 0.025)),
                    nalgebra::Point2::new(WIDTH * LEFT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER + 0.025)),
                    nalgebra::Point2::new(WIDTH * (LEFT_MULTIPLIER - 0.025), HEIGHT * TOP_MULTIPLIER),
                    nalgebra::Point2::new(WIDTH * LEFT_MULTIPLIER, HEIGHT * (TOP_MULTIPLIER - 0.025)),
                ],
                graphics::WHITE,
            )?
            .build(ctx)?;


        Ok(DialogBox {
            dialog_box_mesh,
            dialog_text: String::from(""),
            actor_box_mesh,
            actor: String::from(""),
            font,
            dialog_text_alignment: Align::Left,
            dialog_text_font_size: dialog_font_size,
            actor_text_font_size: actor_font_size,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // No need to draw anything, if it's all empty
    	if self.actor.is_empty() && self.dialog.is_empty() {
            Ok(())
        }

        let (width, height) = graphics::drawable_size(ctx);
        let scale: Vector2<f32> = Vector2::<f32>::new(
            width / WIDTH,
            height / HEIGHT,
        );
        let position = nalgebra::Point2::new(0.0, 0.0);
        let draw_params = DrawParam::default().dest(position).scale(scale);

        // First draw the dialog box
        graphics::draw(ctx, &self.dialog_box_mesh, draw_params)?;

        if !self.actor.is_empty() {
            let mut text = Text::new((self.actor.clone(), self.font, self.actor_text_font_size));
            let text_position = nalgebra::Point2::new(width * LEFT_MULTIPLIER, height * (TOP_MULTIPLIER - 0.025));

            text.set_bounds(nalgebra::Point2::new((width * RIGHT_MULTIPLIER - width * LEFT_MULTIPLIER), f32::INFINITY), Align::Center);

            // Draw Actor box and Actor text
            graphics::draw(ctx, &self.actor_box_mesh, draw_params)?;
            graphics::draw(ctx, &text, DrawParam::default().color(graphics::BLACK).dest(text_position))?;
        }

        Ok(())
    }

    pub fn set_actor(&mut self, actor: &str) {
        self.actor = actor.to_string();
    }
}