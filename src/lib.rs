use image::Rgb;

pub type Color = Rgb<u8>;

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn inner(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

pub mod colors;
pub mod graph;
pub mod shapes;
