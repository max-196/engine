use crate::files::FileError;

pub enum ShaderError {
    File(FileError)
}

impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ShaderError::*;
        match self {
            File(e) => write!(f,
                "\n        With file: {e}"
            ),
        }
    }
}
impl std::fmt::Debug for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
} impl std::error::Error for ShaderError {}

impl From<FileError> for ShaderError {
    fn from(value: FileError) -> Self {
        Self::File(value)
    }
}