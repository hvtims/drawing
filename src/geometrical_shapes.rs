use rand::Rng;
use raster::Color;
use std::f64::consts::PI;

// ------------------------------------------------------------------------ Traits ---
pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
        )
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// ------------------------------------------------------------------------ Point ---
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point::new(
            rng.gen_range(0..width),
            rng.gen_range(0..height),
        )
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut dyn Displayable) {
        image.display(self.x, self.y, self.color());
    }
}

// ------------------------------------------------------------------------ Circle ---
#[derive(Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
        );
        
        Circle { center, radius, color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Circle::new(
            Point::random(width, height),
            rng.gen_range(20..350),
        )
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut dyn Displayable) {
        let (cx, cy) = (self.center.x as f64, self.center.y as f64);
        let radius = self.radius as f64;

        let num_points = (radius * 2.0 * PI) as usize;
        let angle_step = 2.0 * PI / num_points as f64;

        for i in 0..num_points {
            let angle = angle_step * i as f64;
            let x = cx + radius * angle.cos();
            let y = cy + radius * angle.sin();
            
            image.display(x.round() as i32, y.round() as i32, self.color.clone());
        }
    }
}

// ------------------------------------------------------------------------ LINE ---
#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Color,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
        );
        
        Line { start, end, color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line::new(
            Point::random(width, height),
            Point::random(width, height),
        )
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut dyn Displayable) {
        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        
        let mut err = dx - dy;

        loop {
            image.display(x0, y0, self.color.clone());

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

// ------------------------------------------------------------------------ RECTANGLE ---
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub points: [Point;4],
    pub color: Color,
}

impl Rectangle {
    pub fn new(start: Point, end: Point) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
        );
        
        Rectangle { points:[start,
            Point::new(start.x,end.y),
            end,
            Point::new(end.x,start.y),
            ], color:color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Rectangle::new(
            Point::random(width, height),
            Point::random(width, height),
        )
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let lines = [
            Line::new(self.points[0], self.points[1]),
            Line::new(self.points[1], self.points[2]),
            Line::new(self.points[2], self.points[3]),
            Line::new(self.points[3], self.points[0]),
        ];
        for mut line in lines {
            line.color = self.color.clone();
            line.draw(image);
        }
    }
}


// ------------------------------------------------------------------------ TRIANGE ---

pub struct Triangle {
    pub points : [Point;3],
    pub color: Color,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
            rng.gen_range(1..=255),
        );
        
        Triangle { points:[a,b,c], color:color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Triangle::new(
            Point::random(width, height),
            Point::random(width, height),
            Point::random(width, height),
        )
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let lines = [
            Line::new(self.points[0], self.points[1]),
            Line::new(self.points[1], self.points[2]),
            Line::new(self.points[2], self.points[0]),
        ];
        for mut line in lines {
            line.color = self.color.clone();
            line.draw(image);
        }
    }
}
