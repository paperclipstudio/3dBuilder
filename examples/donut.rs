#![allow(dead_code)]

use open3d::dynamic::*;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::ops::Sub;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("example.scad")?;
    let scad = donut().to_scad();
    write!(file, "{}", scad)?;
    Ok(())
}

fn sprinkle(colour: Colour) -> Solid {
    Solid::Sphere(3.0)
        .transform(3.0, 0.0, 0.0)
        .hull(Solid::Sphere(3.0).transform(-3.0, 0.0, 0.0))
        .colour(colour)
}

const SIZE: f32 = 60.0;
fn donut() -> Solid {
    let colours: Vec<_> = vec![
        Colour::new(49, 222, 89),  // Green
        Colour::new(222, 49, 164), // Red
        Colour::new(202, 222, 49), // Yellow
        Colour::new(49, 158, 222), // Light Blue
        Colour::new(49, 158, 222), // Light Blue
        Colour::new(126, 46, 225), // Purple
    ];
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(1348230942912398093);

    let graph = (0..360)
        .map(|i| i as f32)
        .map(|i| i * std::f32::consts::PI / 180.0)
        .map(|i| {
            Vec2::new(
                (4.0 + i / 90.0) * (i * 5.0).sin() + 8.0,
                i * 180.0 / std::f32::consts::PI,
            )
        })
        .collect::<Vec<_>>();

    let graph2 = [Vec2::new(-50.0, 0.0)]
        .iter()
        .chain(graph.iter())
        .chain([Vec2::new(-50.0, 101.0)].iter())
        .copied()
        .collect::<Vec<Vec2>>();
    let dribble = Plane::polygon(graph2)
        .extrude_twist(300.0, 3.0)
        .rotate(0.0, 90.0, 0.0)
        .transform(-170.0, -130.0, 10.0);

    let sprinkles = (0..100)
        .map(|_| {
            sprinkle(Clone::clone(colours.choose(&mut rng).unwrap()))
                .rotate(0.0, rng.random::<f32>() * 360.0, 90.0)
                .transform(SIZE / 3.0 + 2.0, 0.0, 0.0)
                .rotate(0.0, rng.random::<f32>() * 100.0 + 30.0, 0.0)
                .transform(2.0 * SIZE / 3.0, 0.0, 0.0)
                .rotate(0.0, 0.0, rng.random::<f32>() * 360.0)
        })
        .reduce(|acc, x| acc + x)
        .unwrap();
    let body = Plane::circle(SIZE / 3.0) // Body
        .transform(2.0 / 3.0 * SIZE, 0.0)
        .rotate_extrude(360.0)
        .colour(Colour::new(0xAB, 0x75, 0x59));
    let icing = Plane::circle(SIZE / 3.0 + 2.0) // Icing
        .sub(Plane::square(SIZE, SIZE).transform(-SIZE / 2.0, 0.0))
        .rotate(360.0)
        .transform(2.0 * SIZE / 3.0, 0.0)
        .rotate_extrude(360.0)
        .sub(dribble)
        .colour(Colour::new(0xFF, 0xC5, 0xC9));
    (body + icing + sprinkles).transform(0.0, 0.0, -SIZE / 3.0)
}
