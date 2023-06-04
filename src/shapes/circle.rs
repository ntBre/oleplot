use super::Draw;
use crate::{Color, Point};

// TODO handle stroke
pub struct Circle {
    center: Point,
    radius: u32,
    fill: Color,
}

impl Circle {
    pub fn new(center: Point, radius: u32, fill: Color) -> Self {
        Self {
            center,
            radius,
            fill,
        }
    }
}

impl Draw for Circle {
    fn draw(&self, img: &mut image::RgbImage) {
        let (cx, cy) = self.center;
        let r = self.radius;

        for x in cx - r..=cx + r {
            let dx = x as i32 - cx as i32;
            for y in cy - r..=cy + r {
                let dy = y as i32 - cy as i32;
                if dx * dx + dy * dy <= r as i32 * r as i32 {
                    img.put_pixel(x, y, self.fill);
                }
            }
        }
    }
}
