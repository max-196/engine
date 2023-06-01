pub mod vec2;
pub mod vec3;
pub mod vec4;

pub use {
    vec2::Vec2,
    vec3::Vec3,
    vec4::Vec4,
};

use super::num::Zero;
macro_rules! impl_vec_zero(
    ($type:ty) => {
        impl <T: Zero + PartialEq + Copy> Zero for $type {
            fn zero() -> Self {
                Self::fill(T::zero())
            }

            fn is_zero(self) -> bool {
                self == Self::zero()
            }
        }
    }
);
pub(self) use impl_vec_zero;