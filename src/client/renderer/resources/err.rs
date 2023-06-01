use super::image::ImageError;
use super::model::{err::ModelError, objfile::ObjFileError};

use crate::err::macros::*;


pub enum ResourceError {
    Image(ImageError),
    Model(ModelError),
}

impl_error!(
    ResourceError,
    Image(e) => "With texture: {}", e;
    Model(e)   => "With model: {}", e
);
impl_error_conversions!(ResourceError,
    ImageError => Image,
    ModelError => Model,
    ObjFileError => Model
);