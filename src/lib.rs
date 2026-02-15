#![allow(dead_code)]
#![feature(generic_const_exprs)]

use std::{marker::PhantomData, ops::Add};

pub struct Union<T, U> {
    phantom_data: PhantomData<T>,
    phantom_data2: PhantomData<U>,
}

impl<T, U> Union<T, U>
where
    T: SItem,
{
    fn new() -> Union<T, U> {
        Union::<T, U> {
            phantom_data: PhantomData,
            phantom_data2: PhantomData,
        }
    }
}

impl<X, Y> SItem for Union<X, Y>
where
    X: SItem,
    Y: SItem,
{
    fn print() -> String {
        format!("union() {{\n{} \n{}}}", X::print(), Y::print())
    }

    fn new() -> Self {
        Self {
            phantom_data: PhantomData::<X>,
            phantom_data2: PhantomData::<Y>,
        }
    }

         fn to_dynamic() -> Dynamic {
        Dynamic::Solid(Solid::Add(
            Box::new(X::to_dynamic().unwrap_solid()),
            Box::new(Y::to_dynamic().unwrap_solid()),
        ))
    }
}

pub struct Test<const X: u8, const Y: u8> {}


impl<const X: u8, const Y: u8> Test<X, Y> {
    fn new() -> Test<X, Y> {
        Test {}
    }
}
pub trait SSolid: SItem {
    fn tran<const X: i8, const Y: i8>(self) -> Tran<X, Y, Self> {
        Tran::<X, Y, Self>::new()
    }
}
pub trait SPlane: SItem {
    fn tran<const X: i8, const Y: i8>(self) -> Tran<X, Y, Self> {
        Tran::<X, Y, Self>::new()
    }
}

pub trait SItem: Sized {
    fn rot(self) -> Rot<Self>
    where
        Self: Sized,
    {
        Rot::<Self>::new()
    }

    fn print() -> String;
    fn new() -> Self;
    fn to_dynamic() -> Dynamic;

    fn print2(&self) -> String {
        Self::print()
    }

    fn add<T: SItem>(self, _: T) -> Union<Self, T> {
        Union::<Self, T>::new()
    }

    fn into<T: SItem>(self) -> T {
        T::new()
    }
}

pub struct Tran<const X: i8, const Y: i8, T> {
    phantom_data1: std::marker::PhantomData<T>,
}

impl<T: SItem, const X1: i8, const X2: i8, const Y1: i8, const Y2: i8>
    Tran<X1, Y1, Tran<X2, Y2, T>>
{
    fn reduce(self) -> Tran<{ X1 + X2 }, { Y1 + Y2 }, T> {
        Tran::<{ X1 + X2 }, { Y1 + Y2 }, T>::new()
    }
}

impl<const X: i8, const Y: i8, T> Tran<X, Y, T>
where
    T: SItem,
{
    fn new() -> Tran<X, Y, T> {
        Tran::<X, Y, T> {
            phantom_data1: PhantomData,
        }
    }
}

pub struct Rot<T> {
    phantom_data: std::marker::PhantomData<T>,
}

impl<T> Rot<T>
where
    T: SItem,
{
    fn new() -> Rot<T> {
        Rot::<T> {
            phantom_data: PhantomData,
        }
    }
}

impl<T> SSolid for Rot<T>
where
    T: SItem,
{}

struct Circle<const X: i8> {}

impl<const X: i8> Circle<X> {
    fn new() -> Circle<X> {
        Circle::<X> {}
    }
}

impl<const X: i8> SItem for Circle<{ X }> {
    fn print() -> String {
        format!("circle({});", X)
    }

    fn new() -> Self {
        Circle::<X> {}
    }

    fn to_dynamic() -> Dynamic {
        Dynamic::Plane(Plane::Circle(X as f32))
    }
}

impl<const X: i8> SPlane for Circle<{ X }> {}

impl<const X: i8, const Y:i8, T: SItem> SItem for Tran<X,Y,T> {
    fn print() -> String {
        format!("Translate() {}", T::print())
    }

    fn new() -> Self {
        Tran::<X,Y,T> {
            phantom_data1: PhantomData,
        }
    }

    fn to_dynamic() -> Dynamic {
        Dynamic::Plane(Plane::Transform(Box::new(T::to_dynamic().unwrap_plane()), Vec2::new(X as f32,Y as f32)))
    }
}

impl<const X: i8, const Y:i8, T: SItem> SPlane for Tran<X,Y,T> {}

impl<T: SItem> SItem for Rot<T> {
    fn print() -> String {
        format!("Rotate() {}", T::print())
    }

    fn new() -> Self {
        Rot::<T> {
            phantom_data: PhantomData,
        }
    }
    fn to_dynamic() -> Dynamic {
        Dynamic::Solid(Solid::Rotate(Box::new(T::to_dynamic().unwrap_solid()), 0.0))
    }
}

fn test() -> String {
    let a = Test::<3, 4> {};
    let b = a.tran::<1,3>().rot().tran::<4,3>().rot();
    let c = (Circle::<3> {}).rot().tran::<2,2>().rot();
    b.print2() + "\n" + &c.print2()
}

impl<const X: u8, const Y: u8> SItem for Test<{ X }, Y> {
    fn print() -> String {
        "test();".to_string()
    }

    fn new() -> Self {
        Self {}
    }

    fn to_dynamic() -> Dynamic {
        Dynamic::Plane(Plane::Circle(4.0))
    }

}
impl<const X: u8, const Y: u8> SSolid for Test<{ X }, Y> {

}

#[inline(never)]
#[unsafe(no_mangle)]
fn add(a: u8, b: u8) -> u8 {
    a + b
}

//
//
//

type Vec1 = f32;

pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

trait Scad {
    fn to_scad(&self) -> String;
}

pub enum Dynamic {
    Solid(Solid),
    Plane(Plane),
}

impl Dynamic {
    fn unwrap_solid(self) -> Solid {
        match self {
            Dynamic::Solid(s) => s,
            Dynamic::Plane(_) => panic!(),
        }
    }

    fn unwrap_plane(self) -> Plane {
        match self {
            Dynamic::Plane(p) => p,
            Dynamic::Solid(_) => panic!(),
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
        Plane::Transform(Box::new(self), Vec2::new(x, y))
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

            Self::Nest(inner) => inner.to_scad()
                
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
    fn make_square_2() {
        let b = Circle::<5>::new();
        let a = Test::<3, 4>::new().tran::<1,3>().rot().add(b);

        assert_eq!(
            a.print2(),
            "Translate() union() {\nRotate() Translate() test(); \ncircle(5);}"
        );
    }

    #[test]
    fn transform_square() {
        let square = Plane::square(Vec2::new(4.0, 4.0)).transform(3.0, 3.0);
        assert_eq!(square.to_scad(), "transform([3,3]) square([4,4]);");
    }

    #[test]
    fn recursion() {
        fn donut<const N: i8, const X:i8, const Y:i8>() -> Tran<X,Y, Circle<N>> {
            Circle::<N>::new().tran::<X,Y>()
        }

    }
    #[test]
    fn zero_sized() {
        assert_eq!(0, size_of::<Rot<Tran<3,4, Circle<4>>>>());
    }

    #[test]
    fn basic_print() {
        let a = Circle::<2>::new().tran::<3,4>();
        assert_eq!("Translate() circle(2);", a.print2());
    }

    #[test]
    fn reduce() {
        let _: Tran<5,5, Circle<2>> = Circle::<2>::new().tran::<3,4>().tran::<2,1>().reduce();
    }

}
