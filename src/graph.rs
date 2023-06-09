use std::path::Path;

use image::RgbImage;

use crate::{
    colors::{BLACK, BLUE, WHITE},
    shapes::{circle::Circle, rectangle::Rectangle, Draw, Stroke},
    Color, Point,
};

pub type Data = (f64, f64);

#[derive(Debug)]
struct Canvas {
    beg: Point,
    end: Point,
}

impl Canvas {
    fn new<P>(beg: P, end: P) -> Self
    where
        P: Into<Point>,
    {
        Self {
            beg: beg.into(),
            end: end.into(),
        }
    }

    fn width(&self) -> u32 {
        self.end.x - self.beg.x
    }

    fn height(&self) -> u32 {
        self.end.y - self.beg.y
    }
}

pub struct Graph {
    /// total width of the output image (in pixels)
    width: u32,

    /// total height of the output image (in pixels)
    height: u32,

    /// internal image buffer
    image: RgbImage,

    /// section of the image for plotting data
    canvas: Canvas,

    /// plot data
    data: Vec<Data>,
}

impl Graph {
    pub fn new(
        width: u32,
        height: u32,
        background: Color,
        data: Vec<Data>,
    ) -> Self {
        let image = RgbImage::from_pixel(width, height, background);

        Self {
            width,
            height,
            image,
            data,
            canvas: Canvas::new((0, 0), (width, height)),
        }
    }

    /// draw plot elements like axes
    pub fn draw(&mut self) {
        let Graph {
            width: w,
            height: h,
            ..
        } = *self;

        let ox = w / 8;
        let oy = h / 8;

        let beg = (ox, oy);
        let end = (w - ox, h - oy);

        let axes = Rectangle::new(
            beg.into(),
            end.into(),
            WHITE,
            Stroke::new(BLACK, 2),
        );

        self.canvas = Canvas::new(beg, end);

        axes.draw(&mut self.image);
    }

    /// plot `self.data`. TODO take a shape for the markers
    pub fn plot(&mut self) {
        // TODO take x and y range from input
        let (mut min_x, mut min_y) = self.data.first().unwrap_or(&(0.0, 0.0));
        let (mut max_x, mut max_y) = self.data.first().unwrap_or(&(0.0, 0.0));
        for (x, y) in &self.data {
            if *x < min_x {
                min_x = *x
            }
            if *y < min_y {
                min_y = *y
            }
            if *x > max_x {
                max_x = *x
            }
            if *y > max_y {
                max_y = *y
            }
        }

        let cw = self.canvas.width();
        let ch = self.canvas.height();

        let dw = max_x - min_x;
        let dh = max_y - min_y;

        for (x, y) in &self.data {
            // normalize to drawing area
            let mx = (cw as f64 * (x - min_x) / dw).round() as u32;
            let my = (ch as f64 * (y - min_y) / dh).round() as u32;

            // convert to screen coordinates
            let mx = mx + self.canvas.beg.x;
            let my = self.canvas.end.y - my;

            let c = Circle::new((mx, my).into(), 2, BLUE);
            c.draw(&mut self.image);
        }
    }

    pub fn save<P>(&self, path: P) -> Result<(), image::ImageError>
    where
        P: AsRef<Path>,
    {
        self.image.save(path)
    }
}
