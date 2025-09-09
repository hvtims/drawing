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
#[derive(Debug, Clone)]
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