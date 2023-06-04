use std::error::Error;

use oleplot::{colors::WHITE, graph::Graph};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;

    let mut img = Graph::new(
        WIDTH,
        HEIGHT,
        WHITE,
        vec![
            //
            (100., 100.),
            (150., 150.),
            (200., 200.),
        ],
    );
    img.draw();
    img.plot();
    img.save("test.png")?;

    Ok(())
}
