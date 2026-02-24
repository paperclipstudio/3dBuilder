use open3d::dynamic::*;
use std::fs::File;
use std::io::Write;
use std::ops::Add;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("example.scad")?;
    let scad = Plane::square(13.0, 13.0)
        .transform(10.0, 50.0)
        .add(Plane::circle(10.0).transform(10.9, 13.3))
        .add(Plane::circle(12.0))
        .extrude(10.0)
        .to_scad();

    write!(file, "{}", scad)?;
    println!("File written");
    Ok(())
}
