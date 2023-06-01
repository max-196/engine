use {
    crate::files::FileError,
    super::objfile::ObjFileError,
    super::super::image::ImageError,
};

use crate::err::macros::*;


// MAIN TYPE
pub enum ModelError {
    File(FileError),
    Image(ImageError),
    ObjFile(ObjFileError),
}

impl_error!(ModelError,
    File(e) => "With file: {}", e;
    Image(e) => "With texture: {}", e;
    ObjFile(e) => "While importing Wavefront OBJ: {}", e
);

impl_error_conversions!(ModelError,
    FileError => File,
    ObjFileError => ObjFile,
    ImageError => Image
);