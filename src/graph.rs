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

        // TODO take this from input. kinda makes more sense to use margins
        // instead of fractions as input
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

        // draw major ticks
        let tick_num = 5;
        let tick_len = 4;
        self.draw_ticks(beg, end, tick_num, tick_len);

        // TODO minor ticks between majors. can't just call draw_ticks again
        // because the intervals don't quite match up. going to rework the
        // step_by stuff
    }

    fn draw_ticks(
        &mut self,
        beg: (u32, u32),
        end: (u32, u32),
        tick_num: u32,
        tick_len: u32,
    ) {
        let step = (end.0 - beg.0) / (tick_num + 1);
        for x in (beg.0..end.0).step_by(step as usize).skip(1) {
            for y in end.1 - tick_len..end.1 + tick_len {
                self.image.put_pixel(x, y, BLACK);
            }
        }
        let step = (end.1 - beg.1) / (tick_num + 1);
        for y in (beg.1..end.1).rev().step_by(step as usize).skip(1) {
            for x in beg.0 - tick_len..beg.0 + tick_len {
                self.image.put_pixel(x, y, BLACK);
            }
        }
    }

    /// plot `self.data`. TODO take a shape for the markers
    pub fn plot(&mut self) {
        // TODO take x and y range from input
        let (min_x, min_y, max_x, max_y) = min_max(&self.data);

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

/// return the min_x, min_y, max_x, and max_y coordinates in `data`
fn min_max(data: &[Data]) -> (f64, f64, f64, f64) {
    let (mut min_x, mut min_y) = data.first().unwrap_or(&(0.0, 0.0));
    let (mut max_x, mut max_y) = data.first().unwrap_or(&(0.0, 0.0));
    for (x, y) in data {
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
    (min_x, min_y, max_x, max_y)
}
