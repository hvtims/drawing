mod geometrical_shapes;

use geometrical_shapes::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // Draw a random point
    geometrical_shapes::Point::random(image.width, image.height).draw(&mut image);

    // Draw multiple random circles
    for _ in 0..50 {
        geometrical_shapes::Circle::random(image.width, image.height).draw(&mut image);
    }
    //draw lines
    for _ in 0..5 {
        geometrical_shapes::Line::random(image.width, image.height).draw(&mut image);
    }
    //draw rectangles
    for _ in 0..5 {
        geometrical_shapes::Rectangle::random(image.width, image.height).draw(&mut image);
    }
    //draw triangles
    for _ in 0..5 {
        geometrical_shapes::Triangle::random(image.width, image.height).draw(&mut image);
    }
    raster::save(&image, "image.png").expect("Failed to save image");
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            let _ = self.set_pixel(x, y, color);
        }
    }
}
