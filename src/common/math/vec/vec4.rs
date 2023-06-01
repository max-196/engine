use super::super::num::{Float, Number, Signed, Zero, One};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl <T> Vec4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {x, y, z, w}
    }
}

impl <T: Copy> Vec4<T> {
    pub const fn fill(v: T) -> Self {
        Self {x: v, y: v, z: v, w: v}
    }
}

impl <T: Number> Vec4<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.w * rhs.w
    }

    /// Hadamard product
    pub fn hadamard(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }

    /// Sum of the components
    pub fn cmpt_sum(self) -> T {
        self.x + self.y + self.z + self.w
    }
}

impl <T: Float> Vec4<T> {
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn distance(self, rhs: Self) -> T {
        (self - rhs).magnitude()
    }
}

super::impl_vec_zero!(Vec4<T>);

impl <T: Zero + One> Vec4<T> {
    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero(), z: T::zero(), w: T::zero() }
    }

    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one(), z: T::zero(), w: T::zero() }
    }

    pub fn unit_z() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::one(), w: T::zero() }
    }

    pub fn unit_w() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::one() }
    }
}


impl <T> From<(T, T, T, T)> for Vec4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
            w: value.3
        }
    }
}

impl <T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(value: [T; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3]
        }
    }
}

impl <T> From<Vec4<T>> for [T; 4] {
    fn from(value: Vec4<T>) -> [T; 4] {
        [value.x, value.y, value.z, value.w]
    }
}

impl <T> From<Vec4<T>> for (T, T, T, T) {
    fn from(value: Vec4<T>) -> (T, T, T, T) {
        (value.x, value.y, value.z, value.w)
    }
}

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

impl <T: Number> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}
impl <T: Number> Sub<Vec4<T>> for Vec4<T> {
    type Output = Self;
    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}
impl <T: Signed> Neg for Vec4<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl <T: Number> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl <T: Float> Div<T> for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}



// ASSIGNMENT
use super::super::macros::impl_assignments;
impl_assignments!(Vec4<T>);