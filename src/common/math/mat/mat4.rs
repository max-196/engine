use super::super::{vec::{Vec4, Vec3}, num::{Number, One, Zero, Float, Signed}, angle::Angle};

use crate::math::Quat;


pub type Mat4<T> = Vec4<Vec4<T>>;

impl <T> Mat4<T> {
    pub const fn new_mat(
        c0r0: T, c1r0: T, c2r0: T, c3r0: T,
        c0r1: T, c1r1: T, c2r1: T, c3r1: T,
        c0r2: T, c1r2: T, c2r2: T, c3r2: T,
        c0r3: T, c1r3: T, c2r3: T, c3r3: T,
    ) -> Self {
        Self {
            x: Vec4::new(c0r0, c0r1, c0r2, c0r3),
            y: Vec4::new(c1r0, c1r1, c1r2, c1r3),
            z: Vec4::new(c2r0, c2r1, c2r2, c2r3),
            w: Vec4::new(c3r0, c3r1, c3r2, c3r3),
        }
    }

    fn transpose(self) -> Self {
        Self::new_mat(
            self.x.x, self.x.y, self.x.z, self.x.w,
            self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w,
            self.w.x, self.w.y, self.w.z, self.w.w,
        )
    }
}

impl <T: Number> Mat4<T> {
    pub fn from_unit_quat(q: Quat<T>) -> Self {
        Self::new_mat(
            T::one() - (q.v.y.square() + q.v.z.square()).double(), (q.v.x * q.v.y - q.s * q.v.z).double()               , (q.v.x * q.v.z + q.s * q.v.y).double()               , T::zero(),
            (q.v.x * q.v.y + q.s * q.v.z).double()               , T::one() - (q.v.x.square() + q.v.z.square()).double(), (q.v.y * q.v.z - q.s * q.v.x).double()               , T::zero(),
            (q.v.x * q.v.z - q.s * q.v.y).double()               , (q.v.y * q.v.z + q.s * q.v.x).double()               , T::one() - (q.v.x.square() + q.v.y.square()).double(), T::zero(),
            T::zero(), T::zero(), T::zero(), T::one(),
        )
    }
}

impl <T: One + Zero> Mat4<T> {
    pub fn from_translation(t: Vec3<T>) -> Self {
        Self::new(
            Vec4::unit_x(),
            Vec4::unit_y(),
            Vec4::unit_z(),
            t.homogeneous_point(),
        )
    }
}

impl <T: Float + One + Zero> Mat4<T> {
    pub fn look_to_rh(eye: Vec3<T>, dir: Vec3<T>, up: Vec3<T>) -> Self {
        let dir = dir.normalize();
        let s = dir.cross(up).normalize();
        let u = s.cross(dir);


        Self::new_mat(
            s.x      , s.y      , s.z      , -eye.dot(s),
            u.x      , u.y      , u.z      ,  -eye.dot(u),
            -dir.x   , -dir.y   , -dir.z   , eye.dot(dir),
            T::zero(), T::zero(), T::zero(), T::one()
        )
    }

    pub fn perspective(fovy: Angle<T>, aspect: T, near: T, far: T) -> Self {
        let f = T::one() / (fovy.rad() * (T::cast(0.5))).tan();
        let d = near - far;
        Self::new_mat(
            f / aspect, T::zero(), T::zero()       , T::zero(),
            T::zero() , f        , T::zero()       , T::zero(),
            T::zero() , T::zero(), (far + near) / d, (far * near) / d,
            T::zero() , T::zero(), -T::one()       , T::zero(),
        )
    }
}

impl <T: Float + One + Zero + Signed> Mat4<T> {
    pub fn look_to_lh(eye: Vec3<T>, dir: Vec3<T>, up: Vec3<T>) -> Self {
        Self::look_to_rh(eye, -dir, up)
    }
}

impl <T: Zero + One> Mat4<T> {
    pub fn identity() -> Self {
        Self::new(
            Vec4::unit_x(),
            Vec4::unit_y(),
            Vec4::unit_z(),
            Vec4::unit_w()
        )
    }
}

use std::ops::Mul;
impl <T: Number> Mul<Mat4<T>> for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        Self::new(
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.x),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.x),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.x),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.x),
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.y),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.y),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.y),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.y),
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.z),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.z),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.z),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.z),
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.w),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.w),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.w),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.w),
            ),
        )
    }
}

impl <T> From<Mat4<T>> for [[T; 4]; 4] {
    fn from(mat: Mat4<T>) -> Self {
        [
            mat.x.into(),
            mat.y.into(),
            mat.z.into(),
            mat.w.into()
        ]
    }
}

impl <T: Copy> From<[[T; 4]; 4]> for Mat4<T> {
    fn from(value: [[T; 4]; 4]) -> Self {
        Self::new(
            Vec4::from(value[0]),
            Vec4::from(value[1]),
            Vec4::from(value[2]),
            Vec4::from(value[3]),
        )
    }
}