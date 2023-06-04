use std::path::Path;

use image::RgbImage;

use crate::{
    colors::{BLACK, GREEN, GREY},
    shapes::{circle::Circle, rectangle::Rectangle, Draw, Stroke},
    Color,
};

pub type Data = (f64, f64);

// TODO embed some kind of Canvas to define the actual plot area. use this to
// draw the axes below and also to normalize the data when plotting that
pub struct Graph {
    /// total width of the output image (in pixels)
    width: u32,

    /// total height of the output image (in pixels)
    height: u32,

    /// internal image buffer
    image: RgbImage,

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
        }
    }

    /// draw plot elements like axes
    pub fn draw(&mut self) {
        let Graph {
            width: w,
            height: h,
            ..
        } = *self;

        let ox = w / 4;
        let oy = h / 4;

        let axes = Rectangle::new(
            (ox, oy),
            (w - ox, h - oy),
            GREY,
            Stroke::new(BLACK, 2),
        );

        axes.draw(&mut self.image);
    }

    /// plot `self.data`. TODO take a shape for the markers
    pub fn plot(&mut self) {
        for (x, y) in &self.data {
            // TODO normalize the coordinates of the data. might be a good idea
            // to do this at the beginning unless we only access it once

            // let x = (x / self.width as f64).round() as u32;
            // let y = (y / self.height as f64).round() as u32;

            let c = Circle::new((*x as u32, *y as u32), 4, GREEN);
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
