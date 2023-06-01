use super::super::{vec::Vec3, num::{Number, One, Zero}};

use crate::math::Quat;


pub type Mat3<T> = Vec3<Vec3<T>>;

impl <T> Mat3<T> {
    pub const fn new_mat(
        c0r0: T, c1r0: T, c2r0: T,
        c0r1: T, c1r1: T, c2r1: T,
        c0r2: T, c1r2: T, c2r2: T,
    ) -> Self {
        Self {
            x: Vec3::new(c0r0, c0r1, c0r2),
            y: Vec3::new(c1r0, c1r1, c1r2),
            z: Vec3::new(c2r0, c2r1, c2r2),
        }
    }
}

impl <T: Number> Mat3<T> {
    pub fn from_unit_quat(q: Quat<T>) -> Self {
        Self::new_mat(
            T::one() - (q.v.y.square() + q.v.z.square()).double(), (q.v.x * q.v.y - q.s * q.v.z).double()               , (q.v.x * q.v.z + q.s * q.v.y).double(),
            (q.v.x * q.v.y + q.s * q.v.z).double()               , T::one() - (q.v.x.square() + q.v.z.square()).double(), (q.v.y * q.v.z + q.s * q.v.x).double(),
            (q.v.x * q.v.z - q.s * q.v.y).double()               , (q.v.y * q.v.z - q.s * q.v.x).double()               , T::one() - (q.v.x.square() + q.v.y.square()).double(),
        )
    }
}

impl <T: Zero + One> Mat3<T> {
    pub fn identity() -> Self {
        Self::new(
            Vec3::unit_x(),
            Vec3::unit_y(),
            Vec3::unit_z(),
        )
    }
}

impl <T> From<Mat3<T>> for [[T; 3]; 3] {
    fn from(mat: Mat3<T>) -> Self {
        [
            mat.x.into(),
            mat.y.into(),
            mat.z.into(),
        ]
    }
}

impl <T: Copy> From<[[T; 3]; 3]> for Mat3<T> {
    fn from(value: [[T; 3]; 3]) -> Self {
        Self::new(
            Vec3::from(value[0]),
            Vec3::from(value[1]),
            Vec3::from(value[2]),
        )
    }
}