#![allow(dead_code)]

use crate::dynamic::*;
use std::marker::PhantomData;

trait ZeroSized: Sized {}

pub struct SColour<T> {
    phantom_data: PhantomData<T>,
}

impl<T> SColour<T> where T: SItem {}

impl<T> SItem for SColour<T>
where
    T: SItem,
{
    fn new() -> SColour<T> {
        SColour::<T> {
            phantom_data: PhantomData,
        }
    }

    fn print() -> String {
        format!("COLOUR() {}", T::print())
    }
}

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
        format!("union() {{ {} {} }}", X::print(), Y::print())
    }

    fn new() -> Self {
        Union::<X, Y> {
            phantom_data: PhantomData,
            phantom_data2: PhantomData,
        }
    }
}

impl<X, Y> SSolid for Union<X, Y>
where
    X: SSolid,
    Y: SSolid,
{
    fn to_dynamic() -> Solid {
        todo!()
    }
}

impl<X, Y> Union<X, Y>
where
    X: SPlane,
    Y: SPlane,
{
    fn to_dynamic() -> Plane {
        Plane::Add(vec![X::to_dynamic()])
    }
}

pub trait SSolid: SItem {
    fn tran<const X: i8, const Y: i8>(self) -> Tran<X, Y, Self> {
        Tran::<X, Y, Self>::new()
    }

    fn to_dynamic() -> Solid;
}

pub trait SPlane: SItem {
    fn tran<const X: i8, const Y: i8>(self) -> Tran<X, Y, Self> {
        Tran::<X, Y, Self>::new()
    }

    fn to_dynamic() -> Plane;
}

pub trait SItem: Sized {
    fn rot<const X: i8, const Y: i8, const Z: i8>(self) -> Rot<Self, X, Y, Z>
    where
        Self: Sized,
    {
        Rot::<Self, X, Y, Z>::new()
    }

    fn print() -> String;
    fn new() -> Self;

    fn print2(&self) -> String {
        Self::print()
    }

    fn add<T: SItem>(self, _: T) -> Union<Self, T> {
        Union::<Self, T>::new()
    }

    fn into<T: SItem>(self) -> T {
        T::new()
    }

    fn colour<T: SItem>(self) -> SColour<T> {
        SColour::<T>::new()
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

pub struct Rot<T, const X: i8, const Y: i8, const Z: i8> {
    phantom_data: PhantomData<T>,
}

impl<T, const X: i8, const Y: i8, const Z: i8> Rot<T, X, Y, Z>
where
    T: SItem,
{
    fn new() -> Rot<T, X, Y, Z> {
        Rot::<T, X, Y, Z> {
            phantom_data: PhantomData,
        }
    }
}

impl<T, const X: i8, const Y: i8, const Z: i8> SSolid for Rot<T, X, Y, Z>
where
    T: SSolid,
{
    fn to_dynamic() -> Solid {
        Solid::Rotate(
            Box::new(T::to_dynamic()),
            Vec3::new(X as f32, Y as f32, Z as f32),
        )
    }
}

pub struct Circle<const X: i8> {}

impl<const X: i8> SItem for Circle<X> {
    fn print() -> String {
        format!("circle({});", X)
    }

    fn new() -> Circle<X> {
        Circle::<X> {}
    }
}

impl<const X: i8> Circle<X> {}

impl<const X: i8> SPlane for Circle<{ X }> {
    fn to_dynamic() -> Plane {
        Plane::Circle(X as f32)
    }
}

impl<const X: i8, const Y: i8, T: SPlane> SItem for Tran<X, Y, T> {
    fn print() -> String {
        format!("Translate() {}", T::print())
    }

    fn new() -> Self {
        Tran::<X, Y, T> {
            phantom_data1: PhantomData,
        }
    }
}

impl<const X: i8, const Y: i8, T: SPlane> SPlane for Tran<X, Y, T> {
    fn to_dynamic() -> Plane {
        Plane::Transform(Box::new(T::to_dynamic()), Vec2::new(X as f32, Y as f32))
    }
}

impl<T: SItem, const X: i8, const Y: i8, const Z: i8> SItem for Rot<T, X, Y, Z> {
    fn print() -> String {
        format!("Rotate({}, {}, {}) {}", X, Y, Z, T::print())
    }

    fn new() -> Self {
        Rot::<T, X, Y, Z> {
            phantom_data: PhantomData,
        }
    }
}

impl<T: SPlane, const X: i8, const Y: i8, const Z: i8> SPlane for Rot<T, X, Y, Z> {
    fn to_dynamic() -> Plane {
        Plane::Rotate(Box::new(T::to_dynamic()), 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::SPlane;

    #[test]
    fn recursion() {
        fn donut<const N: i8, const X: i8, const Y: i8>() -> Tran<X, Y, Circle<N>> {
            Circle::<N>::new().tran::<X, Y>()
        }
        let _ = donut::<10, 3, 3>().tran::<3, 4>();
    }

    #[test]
    fn zero_sized() {
        assert_eq!(0, size_of::<Rot<Tran<3, 4, Circle<4>>, 2, 3, 4>>());
    }

    #[test]
    fn basic_print() {
        let a = Circle::<2>::new().tran::<3, 4>();
        assert_eq!("Translate() circle(2);", a.print2());
    }

    #[test]
    fn reduce() {
        let _: Tran<5, 5, Circle<2>> = Circle::<2>::new().tran::<3, 4>().tran::<2, 1>().reduce();
    }
}
