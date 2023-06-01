pub struct PathManager {
    shaders : std::path::PathBuf,
    models  : std::path::PathBuf,
    cubemaps: std::path::PathBuf,
    textures: std::path::PathBuf,
}

impl PathManager{
    pub fn new<T: Into<std::path::PathBuf>>(
        shaders: T,
        models: T,
        cubemaps: T,
        textures: T,
    ) -> Self {
        Self {
            shaders : shaders.into(),
            models  : models.into(),
            cubemaps: cubemaps.into(),
            textures: textures.into(),
        }
    }

    pub fn shader<T: AsRef<std::path::Path>>(&self, src: T) -> std::path::PathBuf {
        self.shaders.join(src.as_ref())
    }

    pub fn model<T: AsRef<std::path::Path>>(&self, src: T) -> std::path::PathBuf {
        self.models.join(src.as_ref())
    }

    pub fn cubemap<T: AsRef<std::path::Path>>(&self, src: T) -> std::path::PathBuf {
        self.cubemaps.join(src.as_ref())
    }

    pub fn texture<T: AsRef<std::path::Path>>(&self, src: T) -> std::path::PathBuf {
        self.textures.join(src.as_ref())
    }
}