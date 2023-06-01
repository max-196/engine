use super::{super::num::{Float, Number, Signed, Zero, One}, Vec4};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {x, y, z}
    }
}

impl <T: Copy> Vec3<T> {
    pub const fn fill(v: T) -> Self {
        Self {x: v, y: v, z: v}
    }
}


impl <T: Number> Vec3<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Hadamard product
    pub fn hadamard(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    /// Sum of the components
    pub fn cmpt_sum(self) -> T {
        self.x + self.y + self.z
    }
}

impl <T: Float> Vec3<T> {
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    pub fn distance(self, rhs: Self) -> T {
        (self - rhs).magnitude()
    }
}

impl <T: One> Vec3<T> {
    pub fn homogeneous_point(self) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, T::one())
    }
}

impl <T: Zero + One> Vec3<T> {
    pub fn unit_x() -> Self {
        Self { x: T::one(), y: T::zero(), z: T::zero() }
    }

    pub fn unit_y() -> Self {
        Self { x: T::zero(), y: T::one(), z: T::zero() }
    }

    pub fn unit_z() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::one() }
    }
}

super::impl_vec_zero!(Vec3<T>);


impl <T> From<(T, T, T)> for Vec3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2
        }
    }
}

impl <T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2]
        }
    }
}

impl <T> From<Vec3<T>> for [T; 3] {
    fn from(value: Vec3<T>) -> [T; 3] {
        [value.x, value.y, value.z]
    }
}

impl <T> From<Vec3<T>> for (T, T, T) {
    fn from(value: Vec3<T>) -> (T, T, T) {
        (value.x, value.y, value.z)
    }
}

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

impl <T: Number> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl <T: Number> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
impl <T: Signed> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl <T: Number> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}
impl <T: Float> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}



// ASSIGNMENT
use super::super::macros::impl_assignments;
impl_assignments!(Vec3<T>);