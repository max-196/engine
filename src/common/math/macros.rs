macro_rules! impl_assignments {
    ($type:ty) => {
        impl <T, U> AddAssign<U> for $type
        where
            Self: Add<U, Output = Self> + Copy + Clone
        {
            fn add_assign(&mut self, rhs: U) {
                *self = *self + rhs;
            }
        }

        impl <T, U> SubAssign<U> for $type
        where
            Self: Sub<U, Output = Self> + Copy + Clone
        {
            fn sub_assign(&mut self, rhs: U) {
                *self = *self - rhs;
            }
        }

        impl <T, U> MulAssign<U> for $type
        where
            Self: Mul<U, Output = Self> + Copy + Clone
        {
            fn mul_assign(&mut self, rhs: U) {
                *self = *self * rhs;
            }
        }

        impl <T, U> DivAssign<U> for $type
        where
            Self: Div<U, Output = Self> + Copy + Clone
        {
            fn div_assign(&mut self, rhs: U) {
                *self = *self / rhs;
            }
        }
    };
}
pub(super) use impl_assignments;

/// Duplicates the function that already exists in the scope, passing in optional parameters. Allows customizing the return type
macro_rules! duplicate_type_function {
    ($type:ty, $func:ident) => {
        fn $func(self) -> Self {
            <$type>::$func(self)
        }
    };

    ($type:ty, $func:ident, -> ($($res:ident),+)) => {
        fn $func(self) -> ($($res),+) {
            <$type>::$func(self)
        }
    };

    ($type:ty, $func:ident, (self, $($param:ident: $pty:ty),+) -> $($res:ident),+) => {
        fn $func(self, $($param: $pty),+) -> $($res),+ {
            <$type>::$func(self, $($param),+)
        }
    }
}
pub(super) use duplicate_type_function;