use super::num::{Float, Zero};

/// Angle type with internal representation in radians
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Angle<T>(T);

impl <T: Float> Angle<T> {
    pub fn rad(self) -> T { self.0 }
    pub fn deg(self) -> T { self.0.to_degrees() }
    pub fn from_rad(v: T) -> Self { Self(v) }
    pub fn from_deg(v: T) -> Self { Self(v.to_radians()) }
    pub fn sin(self) -> T { self.0.sin() }
    pub fn cos(self) -> T { self.0.cos() }
    pub fn sin_cos(self) -> (T, T) { self.0.sin_cos() }
    pub fn clamp(self, min: Angle<T>, max: Angle<T>) -> Self { Angle(self.0.clamp(min.0, max.0)) }
}

impl <T: Zero + PartialEq> Zero for Angle<T> {
    fn zero() -> Self {Self(T::zero())}
    fn is_zero(self) -> bool {self == Self::zero()}
}

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
impl <T: Float> Add<Angle<T>> for Angle<T> {
    type Output = Self;
    fn add(self, rhs: Angle<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl <T: Float> Sub<Angle<T>> for Angle<T> {
    type Output = Self;
    fn sub(self, rhs: Angle<T>) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl <T: Float> Neg for Angle<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl <T: Float> Mul<T> for Angle<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl <T: Float> Div<T> for Angle<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs)
    }
}

use super::macros::impl_assignments;
impl_assignments!(Angle<T>);