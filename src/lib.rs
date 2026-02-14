#![allow(dead_code)]
// Templating
//
//
struct Union<T, U> {
    phantom_data: T,
    phantom_data2: U,
}

impl<T, U> Union<T, U>
where
    T: Other,
{
    fn new(inner: T, other: U) -> Union<T, U> {
        Union::<T, U> {
            phantom_data: inner,
            phantom_data2: other,
        }
    }
}

impl<X, Y> Other for Union<X, Y>
where
    X: Other,
    Y: Other,
{
    fn print() -> String {
        format!("union() {{\n{} \n{}}}", X::print(), Y::print())
    }
}

struct Test<const X: u8, const Y: u8> {}

impl<const X: u8, const Y: u8> Test<X, Y> {
    fn new() -> Test<X, Y> {
        Test {}
    }
}

trait Other: Sized {
    fn func(self) -> impl Other {
        Func::<Self>::new(self)
    }

    fn func2(self) -> impl Other
    where
        Self: Sized,
    {
        Func2::<Self>::new(self)
    }

    fn print() -> String;

    fn print2(&self) -> String {
        Self::print()
    }

    fn add<T: Other>(self, rhs: T) -> Union<Self, T> {
        Union::<Self, T>::new(self, rhs)
    }
}

struct Func<T> {
    phantom_data: T,
}

impl<T> Func<T>
where
    T: Other,
{
    fn new(inner: T) -> Func<T> {
        Func::<T> {
            phantom_data: inner,
        }
    }
}

struct Func2<T> {
    phantom_data: T,
}

impl<T> Func2<T>
where
    T: Other,
{
    fn new(inner: T) -> Func2<T> {
        Func2::<T> {
            phantom_data: inner,
        }
    }
}

struct Circle<const X: u8> {}
impl<const X: u8> Circle<X> {
    fn new() -> Circle<X> {
        Circle::<X> {}
    }
}

impl<const X: u8> Other for Circle<{ X }> {
    fn print() -> String {
        format!("circle({})", X)
    }
}

impl<T: Other> Other for Func<T> {
    fn print() -> String {
        format!("Translate() {}", T::print())
    }
}

impl<T: Other> Other for Func2<T> {
    fn print() -> String {
        format!("Rotate() {}", T::print())
    }
}

#[unsafe(no_mangle)]
fn test() -> String {
    let a = Test::<3, 4> {};
    let b = a.func().func2().func().func2();
    let c = (Circle::<3> {}).func2().func().func2();
    b.print2() + "\n" + &c.print2()
}

impl<const X: u8, const Y: u8> Other for Test<{ X }, Y> {
    fn print() -> String {
        "test();".to_string()
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
fn add(a: u8, b: u8) -> u8 {
    a + b
}

//
//
//

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

struct Vec3 {
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

enum Solid {
    Extrude(Box<Plane>, f32),
    RotateExtrude(Box<Plane>, f32),
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
        }
    }
}

enum Plane {
    Square(Vec2),
    Circle(Vec2),
    Transform(Box<Plane>, Vec2),
    Scale(Box<Plane>, Vec2),
    Add(Box<Plane>, Box<Plane>),
    Sub(Box<Plane>, Box<Plane>),
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
            Self::Circle(size) => format!("circle([{},0]);", size.x),
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
        }
    }
}

impl std::ops::Add for Plane {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
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
        let a = Test::<3, 4>::new().func().func2().add(b).func();

        assert_eq!(
            a.print2(),
            "linear_extrude(4) transform([3,40]) scale([3,4]) transform([3,4]) scale([3,4]) transform([12,32]) square([4,4]);"
        );
    }

    #[test]
    fn transform_square() {
        let square = Plane::square(Vec2::new(4.0, 4.0)).transform(3.0, 3.0);
        assert_eq!(square.to_scad(), "transform([3,3]) square([4,4]);");
    }
}
