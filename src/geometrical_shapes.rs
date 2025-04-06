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