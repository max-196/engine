use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::cmp::PartialEq;

use super::{One, Zero};

pub trait Number
where
    Self: Sized + Clone + Copy +
        Add<Self, Output = Self> +
        Mul<Self, Output = Self> +
        Sub<Self, Output = Self> +
        PartialEq + One + Zero
{
    fn square(self) -> Self {self * self}
    fn double(self) -> Self {self + self}
}

impl Number for f32 {}
impl Number for f64 {}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}