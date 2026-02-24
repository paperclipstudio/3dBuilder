#![allow(dead_code)]

use open3d::dynamic::*;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::ops::Sub;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("example.scad")?;
    let scad = donut().to_scad();

    println!("{}", scad);

    write!(file, "{}", scad)?;
    Ok(())
}

fn sprinkle(_color: f32) -> Solid {
    Solid::Sphere(3.0)
        .transform(3.0, 0.0, 0.0)
        .hull(Solid::Sphere(3.0).transform(-3.0, 0.0, 0.0))
}

const SIZE: f32 = 60.0;
fn donut() -> Solid {
    let mut rng = rand::rng();
    let sprinkles = (0..100)
        .map(|i| i as f32)
        .map(|i| {
            sprinkle(0.0)
                .rotate(0.0, rng.random::<f32>() * 360.0, 90.0)
                .transform(SIZE / 3.0 + 2.0, 0.0, 0.0)
                .rotate(0.0, rng.random::<f32>() * 100.0 + 30.0, 0.0)
                .transform(2.0 * SIZE / 3.0, 0.0, 0.0)
                .rotate(0.0, 0.0, i * 10.0)
        })
        .reduce(|acc, x| acc + x)
        .unwrap();
    let body = Plane::circle(SIZE / 3.0) // Body
        .transform(2.0 / 3.0 * SIZE, 0.0)
        .rotate_extrude(360.0);
    let icing = Plane::circle(SIZE / 3.0 + 2.0) // Icing
        .sub(Plane::square(SIZE, SIZE).transform(-SIZE / 2.0, 0.0))
        .rotate(360.0)
        .transform(2.0 * SIZE / 3.0, 0.0)
        .rotate_extrude(360.0);
    (body + sprinkles + icing).transform(0.0, 0.0, -SIZE / 3.0) // END
}
/*
$colours = ["red", "green", "blue", "yellow", "orange", "pink"];
module donut($seed) {
  mirror([0,0,1])
    translate([0,0,-$size/3])
    union() {
  color("#ab8759")
      rotate_extrude() translate([2*$size/3,0]) circle($size/3);
      for (i=[0:100]) {

        rotate([0,0,i*10])
          translate([2*$size/3,0])
          rotate(concat([0], rands(0,100,1, $seed+i), [0])+[0,30,0])
          translate([$size/3+3,0,0]) rotate(concat([0], rands(0,360,1, $seed+i), [90])) sprinkle(rands(0,1,3));
      }
      // Icing
      color("pink")

        difference() {
          rotate_extrude() {
            translate([2*$size/3,0,0])
            rotate([0,360,0])
            difference() {
              circle($size/3+2);
              translate([-$size/2,0,0]) square([$size,$size]);
            };
          };
          $graph = concat([[-50,0]], [for (i=[0:360]) [(4+i/90) * sin(i*5)+ 8, i]], [[-50,101]]);

          translate([-150,-130,0]) rotate([0,90,0]) linear_extrude(300, convexity = 20,twist=3) polygon($graph);
        }
    }
}

donut(2);
 translate([$size,0,$size/2])
rotate([0,39,0])
  donut(4);
*/
