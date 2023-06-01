pub trait Zero {
    fn zero() -> Self;
    fn is_zero(self) -> bool;
}

macro_rules! impl_zero {
    ($type:ty, $value:expr) => {
        impl Zero for $type {
            fn zero() -> Self {
                $value
            }

            fn is_zero(self) -> bool {
                self == Self::zero()
            }
        }
    };
}
pub(crate) use impl_zero;

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);
impl_zero!(usize, 0);

impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);
impl_zero!(isize, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);