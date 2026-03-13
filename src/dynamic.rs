#![allow(dead_code)]

use crate::runtime::{SItem, Union};
use std::ops::Add;

pub(crate) type Vec1 = f32;

pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    pub fn to_scad(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
}

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn to_scad(&self) -> String {
        format!("[{}, {},{}]", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Colour {
    pub fn new(r:u8, g:u8, b:u8) -> Colour {
     Colour {
            r,g,b,a:255
        }
    }

    pub fn new_alpha(r:u8, g:u8, b:u8, a:u8) -> Colour {
     Colour {
            r,g,b,a
        }
    }
}

impl Scad for Colour {
    fn to_scad(&self) -> String {
        format!("[{}, {},{},{}]",
            self.r as f32/255.0,
            self.g as f32/255.0,
            self.b as f32/255.0,
            self.a as f32/255.0,
        )
    }
}

pub trait Scad {
    fn to_scad(&self) -> String;
}

pub enum Dynamic {
    Solid(Solid),
    Plane(Plane),
}

impl Dynamic {
    pub fn unwrap_solid(self) -> Option<Solid> {
        match self {
            Dynamic::Solid(s) => Some(s),
            Dynamic::Plane(_) => None,
        }
    }

    pub fn unwrap_plane(self) -> Option<Plane> {
        match self {
            Dynamic::Plane(p) => Some(p),
            Dynamic::Solid(_) => None,
        }
    }
}

pub enum Solid {
    Extrude(Box<Plane>, f32),
    RotateExtrude(Box<Plane>, f32),
    Rotate(Box<Solid>, Vec3),
    Sphere(Vec1),
    Cube(Vec3),
    Transform(Box<Solid>, Vec3),
    Scale(Box<Solid>, Vec3),
    Add(Vec<Solid>),
    Sub(Box<Solid>, Box<Solid>),
    Hull(Vec<Solid>),
    Colour(Box<Solid>, Colour),
}
impl Solid {
    pub fn rotate(self, x: f32, y: f32, z: f32) -> Self {
        Self::Rotate(Box::new(self), Vec3::new(x, y, z))
    }

    pub fn transform(self, x: f32, y: f32, z: f32) -> Self {
        match self {
            Self::Transform(inner, vec) => {
                Self::Transform(inner, Vec3::new(x + vec.x, y + vec.y, z + vec.z))
            }
            _ => Self::Transform(Box::new(self), Vec3::new(x, y, z)),
        }
    }

    pub fn hull(self, other: Solid) -> Self {
        match self {
            Self::Hull(mut inner) => {
                inner.push(other);
                Self::Hull(inner)
            }
            _ => Self::Hull(vec![self, other]),
        }
    }

    pub fn colour(self, colour: Colour) -> Self {
        Self::Colour(Box::new(self), colour)
    }

    pub fn to_scad(&self) -> String {
        match self {
            Self::Cube(size) => format!("cube([{},{},{}]);", size.x, size.y, size.z),
            Self::Sphere(size) => format!("sphere([{},0]);", size),
            Self::Transform(inner, vec) => {
                format!("translate([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Extrude(inner, depth) => {
                format!("linear_extrude({depth}) {}", inner.to_scad())
            }
            Self::Scale(inner, vec) => {
                format!("scale([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Add(lhs) => {
                format!(
                    "union() {{\n{}\n}}",
                    lhs.iter()
                        .map(|solid| solid.to_scad())
                        .reduce(|acc, a| acc + "\n" + a.as_str())
                        .unwrap()
                        .to_owned()
                )
            }
            Self::Hull(lhs) => {
                format!(
                    "hull() {{\n{}\n}}",
                    lhs.iter()
                        .map(|solid| solid.to_scad())
                        .map(|line| format!("  {}", line))
                        .reduce(|acc, a| acc + "\n" + a.as_str())
                        .unwrap()
                        .to_owned()
                )
            }
            Self::Sub(lhs, rhs) => {
                format!("difference() {{ {}  {} }}", lhs.to_scad(), rhs.to_scad())
            }
            Self::RotateExtrude(inner, angle) => {
                format!("rotate_extrude({angle}) {} ", inner.to_scad(),)
            }
            Self::Rotate(inner, angle) => {
                format!("rotate({}) {} ", angle.to_scad(), inner.to_scad(),)
            }
            Self::Colour(inner, colour) => {
                format!("color({}) {} ", colour.to_scad(), inner.to_scad(),)
            }
        }
    }
}

impl Add for Solid {
    type Output = Solid;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Solid::Add(mut vec) => {
                vec.push(rhs);
                Solid::Add(vec)
            }
            _ => Solid::Add(vec![self, rhs]),
        }
    }
}

pub enum Plane {
    Square(Vec2),
    Circle(Vec1),
    Transform(Box<Plane>, Vec2),
    Rotate(Box<Plane>, Vec1),
    Scale(Box<Plane>, Vec2),
    Add(Vec<Plane>),
    Sub(Box<Plane>, Box<Plane>),
    Nest(&'static Plane),
}

impl Plane {
    pub fn square(x: f32, y: f32) -> Plane {
        Plane::Square(Vec2::new(x, y))
    }

    pub fn circle(r: f32) -> Plane {
        Plane::Circle(r)
    }

    pub fn transform(self, x: f32, y: f32) -> Plane {
        match self {
            Plane::Transform(inner, vec) => {
                Plane::Transform(inner, Vec2::new(x + vec.x, y + vec.y))
            }
            _ => Plane::Transform(Box::new(self), Vec2::new(x, y)),
        }
    }

    pub fn rotate(self, angle: f32) -> Plane {
        Plane::Rotate(Box::new(self), angle)
    }

    pub fn scale(self, x: f32, y: f32) -> Plane {
        Plane::Scale(Box::new(self), Vec2::new(x, y))
    }

    pub fn extrude(self, length: f32) -> Solid {
        Solid::Extrude(Box::new(self), length)
    }

    pub fn rotate_extrude(self, angle: f32) -> Solid {
        Solid::RotateExtrude(Box::new(self), angle)
    }

    pub fn to_scad(&self) -> String {
        match self {
            Self::Square(size) => format!("square({});", size.to_scad()),
            Self::Circle(size) => format!("circle({});", size),
            Self::Transform(inner, vec) => {
                format!("translate([{},{}])\n  {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Scale(inner, vec) => {
                format!("scale([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Rotate(inner, angle) => {
                format!("rotate({}) {}", angle, inner.to_scad())
            }
            Self::Add(lhs) => {
                format!(
                    "union() {{\n{}\n}}",
                    lhs.iter()
                        .map(|solid| "  ".to_owned() + solid.to_scad().as_str())
                        .reduce(|acc, a| acc + "\n" + a.as_str())
                        .unwrap()
                        .to_owned()
                )
            }
            Self::Sub(lhs, rhs) => {
                format!("difference() {{ {}  {} }}", lhs.to_scad(), rhs.to_scad())
            }

            Self::Nest(inner) => inner.to_scad(),
        }
        .lines()
        .map(|line| format!("  {}\n", line))
        .collect()
    }
}

impl Add for Plane {
    type Output = Plane;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Plane::Add(mut vec) => {
                vec.push(rhs);
                Plane::Add(vec)
            }
            _ => Plane::Add(vec![self, rhs]),
        }
    }
}

impl std::ops::Sub for Plane {
    type Output = Plane;

    fn sub(self, rhs: Self) -> Self::Output {
        Plane::Sub(Box::new(self), Box::new(rhs))
    }
}

trait MyOther: SItem {
    fn mirror_dup<T: SItem + Clone>(input: T) -> Union<T, T> {
        input.clone().add(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_square() {
        let square = Plane::square(4.0, 4.0);
        assert_eq!(square.to_scad(), "square([4,4]);");
        let cube = square
            .transform(12.0, 32.0)
            .scale(3.0, 4.0)
            .transform(3.0, 4.0)
            .scale(3.0, 4.0)
            .transform(3.0, 40.0)
            .extrude(4.0);
        assert_eq!(
            cube.to_scad(),
            "linear_extrude(4) transform([3,40]) scale([3,4]) transform([3,4]) scale([3,4]) transform([12,32]) square([4,4]);"
        );
    }

    #[test]
    fn transform_square() {
        let square = Plane::square(Vec2::new(4.0, 4.0)).transform(3.0, 3.0);
        assert_eq!(square.to_scad(), "transform([3,3]) square([4,4]);");
    }

    #[test]
    fn nest() {
        let _ = Plane::Nest(&Plane::Circle(4.0));
    }
}
