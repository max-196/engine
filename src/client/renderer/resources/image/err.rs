use crate::files::FileError;

use crate::err::macros::*;

pub enum ImageError {
    File(FileError)
}

impl_error!(ImageError,
    File(e) => "With file: {}", e
);

impl_error_conversion!(ImageError,
    FileError => File
);