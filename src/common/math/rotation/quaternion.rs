use crate::math::{Vec3, Float, Number, One, Angle, Signed};


/// A quaternion type
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Quat<T> {
    /// Scalar component
    pub s: T,
    /// Vector component
    pub v: Vec3<T>,
}

impl <T> Quat<T> {
    pub fn from_sv(s: T, v: Vec3<T>) -> Self { Self  {s, v} }
}

impl <T: Signed> Quat<T> {
    pub fn conj(self) -> Self {
        Self::from_sv(self.s, -self.v)
    }
}

impl <T: Float + Signed> Quat<T> {
    pub fn magnitude(self) -> T {
        (self.s.square() + self.v.dot(self.v)).sqrt()
    }

    pub fn norm(self) -> Self {
        self / self.magnitude()
    }

    pub fn from_axis_angle(axis: Vec3<T>, angle: Angle<T>) -> Self {
        let (sin, cos) = (angle * T::cast(0.5)).sin_cos();
        Self::from_sv(
            cos,
            Vec3::new(axis.x * sin, axis.y * sin, axis.z * sin)
        )
    }

    pub fn inv(self) -> Self {
        self.conj() / self.magnitude().square()
    }
}

impl <T: Number + One> Quat<T> {
    pub fn is_unit(self) -> bool {
        (self.s.square() + self.v.dot(self.v)).is_one()
    }
}

use std::ops::{Mul, Div};

impl <T: Number> Mul<Quat<T>> for Quat<T> {
    type Output = Self;
    fn mul(self, rhs: Quat<T>) -> Self::Output {
        Self::from_sv(
            self.s * rhs.s - (self.v.dot(rhs.v)),
            Vec3::new(
                self.s * rhs.v.x + self.v.x * rhs.s + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
                self.s * rhs.v.y - self.v.x * rhs.v.z + self.v.y * rhs.s + self.v.z * rhs.v.x,
                self.s * rhs.v.z + self.v.x * rhs.v.y - self.v.y * rhs.v.x + self.v.z * rhs.s,
            )
        )
    }
}

impl <T: Number> Mul<Vec3<T>> for Quat<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let t = self.v.cross(rhs) + rhs * self.s;
        (self.v.cross(t) * T::one().double()) + rhs
    }
}

impl <T: Number> Mul<T> for Quat<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::from_sv(self.s * rhs, self.v * rhs)
    }
}

impl <T: Float> Div<T> for Quat<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::from_sv(self.s / rhs, self.v / rhs)
    }
}