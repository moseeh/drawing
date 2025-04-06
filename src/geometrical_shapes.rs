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
    // Implements Bresenham's line drawing algorithm for efficient line rendering
    fn draw(&self, image: &mut Image) {
        let mut x = self.start.x;
        let mut y = self.start.y;
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        let sx = if self.start.x < self.end.x { 1 } else { -1 };
        let sy = if self.start.y < self.end.y { 1 } else { -1 };
        
        let mut err = if dx > dy { dx } else { -dy } / 2;
        
        loop {
            image.display(x, y, self.color());
            if x == self.end.x && y == self.end.y { break; }
            
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

       // Bright magenta for lines
       fn color(&self) -> Color {
        Color::rgb(255, 0, 255)
    }
}

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
    // Draws rectangle by connecting its four corners with lines
    fn draw(&self, image: &mut Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        
        // Get the rectangle's color - use a reference to avoid moves
        let rect_color = self.color();
        
        // Draw each line segment with custom color instead of using Line's default color
        // Mark the closure as mutable because it modifies the image
        let mut draw_line = |start: &Point, end: &Point| {
            let dx = (end.x - start.x).abs();
            let dy = -(end.y - start.y).abs();
            let sx = if start.x < end.x { 1 } else { -1 };
            let sy = if start.y < end.y { 1 } else { -1 };
            let mut err = dx + dy;

            let mut x = start.x;
            let mut y = start.y;

            loop {
                // Clone the color for each use to avoid moving ownership
                image.display(x, y, rect_color.clone());
                if x == end.x && y == end.y { break; }
                let e2 = 2 * err;
                if e2 >= dy {
                    err += dy;
                    x += sx;
                }
                if e2 <= dx {
                    err += dx;
                    y += sy;
                }
            }
        };
        
        // Draw the four sides using our custom line drawing function
        draw_line(&self.top_left, &top_right);
        draw_line(&top_right, &self.bottom_right);
        draw_line(&self.bottom_right, &bottom_left);
        draw_line(&bottom_left, &self.top_left);
    }

    // Bright yellow for rectangles
    fn color(&self) -> Color {
        Color::rgb(255, 255, 0)
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

