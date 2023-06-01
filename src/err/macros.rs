macro_rules! impl_error_conversion {
    ($for:ty, $from:ty => $type:ident) => {
        impl From<$from> for $for {
            fn from(value: $from) -> Self {
                <$for>::$type(value.into())
            }
        }
    }
}

macro_rules! impl_error {
    ($type:ty, $( $variant:ident$(($($e:ident),+))?  => $fmt:expr $(, $($into:expr),+)?  );+ ) => {
        use $type::*;

        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        $variant$(($($e),+))? => write!(f, concat!("\n        ", $fmt) $(, $($into),+)?),
                    )+
                }
            }
        }

        impl ::std::fmt::Debug for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(self, f)
            }
        } impl ::std::error::Error for $type {}
    }
}

macro_rules! impl_error_conversions {
    ($for:ty, $($from:ty => $end:ident),+) => {
        $(
            impl_error_conversion!($for, $from => $end);
        )+
    }
}

pub(crate) use impl_error;
pub(crate) use impl_error_conversion;
pub(crate) use impl_error_conversions;