use std::path::Path;

use image::RgbImage;

use crate::{
    colors::{BLACK, GREY},
    shapes::{Rectangle, Stroke},
    Color,
};

pub struct Graph {
    width: u32,
    height: u32,
    image: RgbImage,
}

impl Graph {
    pub fn new(width: u32, height: u32, background: Color) -> Self {
        let image = RgbImage::from_pixel(width, height, background);

        Self {
            width,
            height,
            image,
        }
    }

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

    pub fn save<P>(&self, path: P) -> Result<(), image::ImageError>
    where
        P: AsRef<Path>,
    {
        self.image.save(path)
    }
}
