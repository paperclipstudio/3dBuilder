#![allow(dead_code)]

use crate::dynamic::*;
use std::marker::PhantomData;

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
        Dynamic::Solid(Solid::Add(vec![X::to_dynamic().unwrap_solid().unwrap()]))
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

impl<T> SSolid for Rot<T> where T: SItem {}

pub struct Circle<const X: i8> {}

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

impl<const X: i8, const Y: i8, T: SItem> SItem for Tran<X, Y, T> {
    fn print() -> String {
        format!("Translate() {}", T::print())
    }

    fn new() -> Self {
        Tran::<X, Y, T> {
            phantom_data1: PhantomData,
        }
    }

    fn to_dynamic() -> Dynamic {
        Dynamic::Plane(Plane::Transform(
            Box::new(T::to_dynamic().unwrap_plane().unwrap()),
            Vec2::new(X as f32, Y as f32),
        ))
    }
}

impl<const X: i8, const Y: i8, T: SItem> SPlane for Tran<X, Y, T> {}

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
        Dynamic::Solid(Solid::Rotate(
            Box::new(T::to_dynamic().unwrap_solid().unwrap()),
            0.0,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursion() {
        fn donut<const N: i8, const X: i8, const Y: i8>() -> Tran<X, Y, Circle<N>> {
            Circle::<N>::new().tran::<X, Y>()
        }
        let _ = donut::<10, 3, 3>().tran::<3, 4>();
    }

    #[test]
    fn zero_sized() {
        assert_eq!(0, size_of::<Rot<Tran<3, 4, Circle<4>>>>());
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
