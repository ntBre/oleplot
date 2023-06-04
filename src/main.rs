use std::error::Error;

use image::{Rgb, RgbImage};

type Color = Rgb<u8>;

pub const WHITE: Color = Rgb([255, 255, 255]);
pub const BLACK: Color = Rgb([0, 0, 0]);
pub const GREEN: Color = Rgb([0, 255, 0]);
pub const GREY: Color = Rgb([222, 222, 222]);

type Point = (u32, u32);

pub struct Stroke {
    color: Color,
    width: u32,
}

impl Stroke {
    fn new(color: Color, width: u32) -> Self {
        Self { color, width }
    }

    pub fn none() -> Self {
        Self {
            color: WHITE,
            width: 0,
        }
    }
}

struct Rectangle {
    beg: Point,
    end: Point,
    fill: Color,
    stroke: Stroke,
}

impl Rectangle {
    fn new(beg: Point, end: Point, fill: Color, stroke: Stroke) -> Self {
        Self {
            beg,
            end,
            fill,
            stroke,
        }
    }

    /// draw `self` onto `img`
    fn draw(&self, img: &mut RgbImage) {
        let (bx, by) = self.beg;
        let (ex, ey) = self.end;

        let sx = bx.min(ex);
        let ex = bx.max(ex);

        let sy = by.min(ey);
        let ey = by.max(ey);

        let w = self.stroke.width;

        // filling
        for x in sx + w..ex - w {
            for y in sy + w..ey - w {
                img.put_pixel(x, y, self.fill);
            }
        }

        for x in sx..ex {
            // top stroke
            for y in sy..sy + w {
                img.put_pixel(x, y, self.stroke.color);
            }

            // bottom stroke
            for y in ey - w..ey {
                img.put_pixel(x, y, self.stroke.color);
            }
        }

        for y in sy..ey {
            // left stroke
            for x in sx..sx + w {
                img.put_pixel(x, y, self.stroke.color);
            }

            // right side
            for x in ex - w..ex {
                img.put_pixel(x, y, self.stroke.color);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;

    let mut img = RgbImage::from_pixel(WIDTH, HEIGHT, WHITE);

    let axes =
        Rectangle::new((75, 38), (525, 272), GREY, Stroke::new(BLACK, 2));

    axes.draw(&mut img);

    img.save("test.png")?;
    Ok(())
}
