use super::super::Color;
use super::super::Point;
use super::Draw;
use super::Stroke;
use image::RgbImage;

pub struct Rectangle {
    pub(crate) beg: Point,
    pub(crate) end: Point,
    pub(crate) fill: Color,
    pub(crate) stroke: Stroke,
}

impl Rectangle {
    pub fn new(beg: Point, end: Point, fill: Color, stroke: Stroke) -> Self {
        Self {
            beg,
            end,
            fill,
            stroke,
        }
    }
}

impl Draw for Rectangle {
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
