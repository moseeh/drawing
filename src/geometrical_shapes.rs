// traits.rs
use raster::Color;
use raster::Image;

/// Trait for objects that can be drawn on an image
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

/// Trait for objects that can be displayed at specific coordinates
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// point.rs
use rand::Rng;
use raster::Color;
use super::traits::{Drawable, Displayable};

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(50, 255, 50) // Lime green
    }
}