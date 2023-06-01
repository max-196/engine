use std::ops::Div;
use std::ops::Neg;

use super::Number;

use super::super::macros::duplicate_type_function;

pub trait Float
where
    Self: Number +
        Div<Self, Output = Self> +
        Neg<Output = Self>
{
    fn sqrt(self) -> Self;
    fn to_radians(self) -> Self;
    fn to_degrees(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn tan(self) -> Self;
    fn cast(c: f64) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_float {
    ($type:ty) => {
        impl Float for $type {
            duplicate_type_function!($type, sqrt);
            duplicate_type_function!($type, to_radians);
            duplicate_type_function!($type, to_degrees);
            duplicate_type_function!($type, sin);
            duplicate_type_function!($type, cos);
            duplicate_type_function!($type, tan);
            duplicate_type_function!($type, sin_cos, -> (Self, Self));
            duplicate_type_function!($type, clamp, (self, min: Self, max: Self) -> Self);

            fn cast(c: f64) -> Self {
                c as Self
            }

        }
    };
}



impl_float!(f32);
impl_float!(f64);