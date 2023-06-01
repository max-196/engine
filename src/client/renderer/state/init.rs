use super::State;

use crate::{
    common::math::vec::Vec2,
    client::{
        Window,
        renderer::err::RendererInitError,
    }
};

impl State {
    pub async fn new(window: &Window) -> Result<Self, RendererInitError> {
        let size = window.inner_size();
        let size = (size.width, size.height).into();

        let instance = Self::init_instance();

        let surface = Self::init_surface(&instance, window)?;

        let adapter = Self::init_adapter(&instance, &surface).await?;

        let (device, queue) = Self::init_device_q(&adapter).await?;

        let config = Self::init_config(&surface, &adapter, size);
        surface.configure(&device, &config);

        Ok(Self { device, queue, size, surface, config })
    }

    fn init_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        })
    }

    fn init_surface(instance: &wgpu::Instance, window: &Window) -> Result<wgpu::Surface, RendererInitError> {
        Ok(unsafe { instance.create_surface(window.get_raw()) }?)
    }

    async fn init_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> Result<wgpu::Adapter, RendererInitError> {
        instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            },
        ).await.map_or_else(
            || instance.enumerate_adapters(wgpu::Backends::all())
                .find(|adapter| adapter.is_surface_supported(surface))
                .ok_or(RendererInitError::Adapter),
        Ok)
    }

    async fn init_device_q(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), RendererInitError> {
        Ok(adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: adapter.limits(),
                        label: Some("Renderer Device"),
                    },
                    None,
                ).await?)
    }

    fn init_config(surface: &wgpu::Surface, adapter: &wgpu::Adapter, size: Vec2<u32>) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::AutoNoVsync, // change this
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        }
    }
}