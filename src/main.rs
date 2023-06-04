use std::error::Error;

use image::RgbImage;
use oleplot::{
    colors::{BLACK, GREY, WHITE},
    shapes::{Rectangle, Stroke},
};

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
