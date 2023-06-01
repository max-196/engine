use super::super::num::{Float, Number, Signed, Zero, One};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl <T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Copy> Vec2<T> {
    pub const fn fill(v: T) -> Self {
        Self {x: v, y: v}
    }
}

impl <T: Number> Vec2<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Hadamard product
    pub fn hadamard(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }

    /// Sum of the components
    pub fn cmpt_sum(self) -> T {
        self.x + self.y
    }
}

impl <T: Float> Vec2<T> {
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn distance(self, rhs: Self) -> T {
        (self - rhs).magnitude()
    }
}

super::impl_vec_zero!(Vec2<T>);

impl <T: Zero + One> Vec2<T> {
    fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero() }
    }

    fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one() }
    }
}


impl <T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

impl <T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1]
        }
    }
}

impl <T> From<Vec2<T>> for [T; 2] {
    fn from(value: Vec2<T>) -> [T; 2] {
        [value.x, value.y]
    }
}

impl <T> From<Vec2<T>> for (T, T) {
    fn from(value: Vec2<T>) -> (T, T) {
        (value.x, value.y)
    }
}


use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

impl <T: Number> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl <T: Number> Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl <T: Signed> Neg for Vec2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}


impl <T: Number> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl <T: Float> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}


// ASSIGNMENT
use super::super::macros::impl_assignments;
impl_assignments!(Vec2<T>);





use winit::dpi::PhysicalSize;

impl <T> From<PhysicalSize<T>> for Vec2<T> {
    fn from(value: PhysicalSize<T>) -> Self {
        Self::new(value.width, value.height)
    }
}

impl <T> From<Vec2<T>> for PhysicalSize<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::new(value.x, value.y)
    }
}