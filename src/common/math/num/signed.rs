use super::Number;

use core::ops::Neg;

pub trait Signed
where
    Self: Number +
        Neg<Output = Self>,
{}

impl Signed for f32 {}
impl Signed for f64 {}
impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}
impl Signed for isize {}