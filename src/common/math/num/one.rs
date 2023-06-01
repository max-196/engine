pub trait One {
    fn one() -> Self;
    fn is_one(self) -> bool;
}

macro_rules! impl_one {
    ($type:ty, $value:expr) => {
        impl One for $type {
            fn one() -> Self {
                $value
            }

            fn is_one(self) -> bool {
                self == Self::one()
            }
        }
    };
}

impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(u128, 1);
impl_one!(usize, 1);

impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(i128, 1);
impl_one!(isize, 1);

impl_one!(f32, 1.0);
impl_one!(f64, 1.0);