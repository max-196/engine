use crate::files::FileError;
use super::{resources::{err::ResourceError, image::ImageError}, gpu::{err::GpuResourceError, shader::ShaderError}};

use crate::err::macros::*;

// MAIN TYPE
pub enum RendererError {
    GpuResource(GpuResourceError),
    File(FileError),
    Init(RendererInitError),
    Resource(ResourceError),
}

impl_error!(
    RendererError,
    GpuResource(e) => "In GPU resources: {}", e;
    File(e)        => "With file: {}", e;
    Init(e)        => "While Initializing renderer: {}", e;
    Resource(e)    => "In resources: {}", e
);
impl_error_conversions!(RendererError,
    FileError => File,
    GpuResourceError => GpuResource,
    RendererInitError => Init,
    ResourceError => Resource,
    ImageError => Resource,
    ShaderError => GpuResource
);

// INIT TYPE
pub enum RendererInitError {
    Surface(wgpu::CreateSurfaceError),
    Adapter,
    Device(wgpu::RequestDeviceError),
}

impl_error!(
    RendererInitError,
    Surface(e) => "While initializing render surface: {}", e;
    Adapter    => "No adapter available";
    Device(e)  => "While initializing device: {}", e
);
impl_error_conversions!(RendererInitError,
    wgpu::CreateSurfaceError => Surface,
    wgpu::RequestDeviceError => Device
);