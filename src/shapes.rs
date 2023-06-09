use image::RgbImage;

use super::Color;
use crate::colors;

pub mod circle;
pub mod rectangle;

pub trait Draw {
    fn draw(&self, img: &mut RgbImage);
}

#[derive(Debug)]
pub struct Stroke {
    pub(crate) color: Color,
    pub(crate) width: u32,
}

impl Stroke {
    pub fn new(color: Color, width: u32) -> Self {
        Self { color, width }
    }

    pub fn none() -> Self {
        Self::new(colors::WHITE, 0)
    }
}
