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
    Rotate(Box<Solid>, f32),
    Sphere(Vec3),
    Cube(Vec3),
    Transform(Box<Solid>, Vec3),
    Scale(Box<Solid>, Vec3),
    Add(Box<Solid>, Box<Solid>),
    Sub(Box<Solid>, Box<Solid>),
}
impl Solid {
    fn to_scad(&self) -> String {
        match self {
            Self::Cube(size) => format!("cube([{},{},{}]);", size.x, size.y, size.z),
            Self::Sphere(size) => format!("circle([{},0]);", size.x),
            Self::Transform(inner, vec) => {
                format!("transform([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Extrude(inner, depth) => {
                format!("linear_extrude({depth}) {}", inner.to_scad())
            }
            Self::Scale(inner, vec) => {
                format!("scale([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Add(lhs, rhs) => {
                format!("union() {{ {}  {} }}", lhs.to_scad(), rhs.to_scad())
            }
            Self::Sub(lhs, rhs) => {
                format!("difference() {{ {}  {} }}", lhs.to_scad(), rhs.to_scad())
            }
            Self::RotateExtrude(inner, angle) => {
                format!("rotate_extrude({angle}) {{ {} }}", inner.to_scad(),)
            }
            Self::Rotate(inner, angle) => {
                format!("rotate({angle}) {{ {} }}", inner.to_scad(),)
            }
        }
    }
}

pub enum Plane {
    Square(Vec2),
    Circle(Vec1),
    Transform(Box<Plane>, Vec2),
    Scale(Box<Plane>, Vec2),
    Add(Box<Plane>, Box<Plane>),
    Sub(Box<Plane>, Box<Plane>),
    Nest(&'static Plane),
}

impl Plane {
    fn square(size: Vec2) -> Plane {
        Plane::Square(size)
    }

    fn transform(self, x: f32, y: f32) -> Plane {
        match self {
            Plane::Transform(inner, vec) => {
                Plane::Transform(inner, Vec2::new(x + vec.x, y + vec.y))
            }
            _ => Plane::Transform(Box::new(self), Vec2::new(x, y)),
        }
    }

    fn scale(self, x: f32, y: f32) -> Plane {
        Plane::Scale(Box::new(self), Vec2::new(x, y))
    }

    fn extrude(self, length: f32) -> Solid {
        Solid::Extrude(Box::new(self), length)
    }

    fn rotate_extrude(self, angle: f32) -> Solid {
        Solid::Extrude(Box::new(self), angle)
    }

    fn to_scad(&self) -> String {
        match self {
            Self::Square(size) => format!("square([{},{}]);", size.x, size.y),
            Self::Circle(size) => format!("circle([{},0]);", size),
            Self::Transform(inner, vec) => {
                format!("transform([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Scale(inner, vec) => {
                format!("scale([{},{}]) {}", vec.x, vec.y, inner.to_scad())
            }
            Self::Add(lhs, rhs) => {
                format!("union() {{ {}  {} }}", lhs.to_scad(), rhs.to_scad())
            }
            Self::Sub(lhs, rhs) => {
                format!("difference() {{ {}  {} }}", lhs.to_scad(), rhs.to_scad())
            }

            Self::Nest(inner) => inner.to_scad(),
        }
    }
}

impl Add for Plane {
    type Output = Plane;

    fn add(self, rhs: Self) -> Self::Output {
        Plane::Add(Box::new(self), Box::new(rhs))
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
        let square = Plane::square(Vec2::new(4.0, 4.0));
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
