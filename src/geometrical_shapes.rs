// External crate imports for graphics and random number generation
use raster::{Color, Image};
use rand::Rng;


/// Trait for objects that can be drawn on an image
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

/// Trait for objects that can be displayed at specific coordinates
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}


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

// triangle.rs
use super::point::Point;
use super::line::Line;
use raster::Color;
use super::traits::Drawable;

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(&self.p1, &self.p2).draw(image);
        Line::new(&self.p2, &self.p3).draw(image);
        Line::new(&self.p3, &self.p1).draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 165, 0) // Orange
    }
}

// circle.rs
use super::point::Point;
use rand::Rng;
use raster::Color;
use raster::Image;
use super::traits::Drawable;

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(5..50);
        let center = Point::random(width, height);
        Self::new(&center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        // Midpoint circle algorithm...
        // (Same as original file)
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255) // Blue
    }
}

