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

// line.rs
use super::point::Point;
use raster::Color;
use raster::Image;
use super::traits::Drawable;

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            start: p1.clone(),
            end: p2.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Self::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        // Bresenham's algorithm implementation...
        // (Same as original file)
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 255) // Magenta
    }
}

// rectangle.rs
use super::point::Point;
use raster::Color;
use raster::Image;
use super::traits::Drawable;

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            top_left: p1.clone(),
            bottom_right: p2.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        // Custom rectangle drawing logic...
        // (Same as original file)
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 0) // Yellow
    }
}