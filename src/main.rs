use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use oleplot::{
    colors::WHITE,
    graph::{Data, Graph},
};

/// load a sequence of x, y data pairs from `path`, skipping any lines that
/// begin with `#` as comments
fn load_data<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<Data>> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let mut ret = Vec::new();
    for line in r.lines().flatten() {
        if line.starts_with('#') {
            continue;
        }
        let mut s = line.split_ascii_whitespace().flat_map(|s| s.parse());
        ret.push((s.next().unwrap(), s.next().unwrap()));
    }
    Ok(ret)
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;

    let mut img =
        Graph::new(WIDTH, HEIGHT, WHITE, load_data("testfiles/x.dat")?);
    img.draw();
    img.plot();
    img.save("test.png")?;

    Ok(())
}
