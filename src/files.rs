use std::{
    path::{Path, PathBuf},
    io::Read,
};

pub fn read_file(path: &Path) -> Result<(String, usize), FileError> {
    let mut contents = String::new();
    match std::fs::File::open(path) {
        Ok(mut v) => match v.read_to_string(&mut contents) {
            Ok(v) => Ok((contents, v)),
            Err(e) => Err(FileReadIO(e, path.to_owned())),
        }
        Err(e) => Err(FileReadIO(e, path.to_owned())),
    }
}

pub fn read_texture<T: AsRef<std::path::Path>>(path: T) -> Result<(Vec<u8>, png::OutputInfo), FileError> {
    let path = path.as_ref();
    let decoder = png::Decoder::new(
        match std::fs::File::open(path) {
            Ok(v) => v,
            Err(e) => return Err(TextureReadIO(e, path.to_path_buf()))
        }
    );
    let mut reader = match decoder.read_info() {
        Ok(v) => v,
        Err(e) => return Err(TextureReadDecoding(e, path.to_path_buf()))
    };
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = match reader.next_frame(&mut img_data) {
        Ok(v) => v,
        Err(e) => return Err(TextureReadDecoding(e, path.to_path_buf()))
    };

    Ok((img_data, info))
}

// ERROR HANDLING

use crate::err::macros::*;

use png::DecodingError;
pub enum FileError {
    FileReadIO(std::io::Error, PathBuf),
    TextureReadIO(std::io::Error, PathBuf),
    TextureReadDecoding(DecodingError, PathBuf),
}

impl_error!(FileError,
    FileReadIO(e, p) => "Couldn't read text file at path '{}': {}", p.display(), e;
    TextureReadIO(e, p) => "Couldn't read texture at path '{}': {}", p.display(), e;
    TextureReadDecoding(e, p) => "Couldn't decode texture at path '{}': {}", p.display(), e
);
