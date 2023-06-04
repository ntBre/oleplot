use std::error::Error;

use oleplot::{colors::WHITE, graph::Graph};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;

    let mut img = Graph::new(WIDTH, HEIGHT, WHITE);
    img.draw();
    img.save("test.png")?;

    Ok(())
}
