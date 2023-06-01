use super::State;

pub struct RenderState {
    out: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
}

impl RenderState {
    pub fn new(state: &State) -> Result<Self, wgpu::SurfaceError> {
        let out = state.surface.get_current_texture()?;
        let view = out.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        Ok(Self {out, view, encoder})
    }

    pub fn render_pass<'a>(
        &'a mut self,
        label: Option<&str>,
        view: Option<&'a wgpu::TextureView>,
        clear_color: Option<wgpu::Color>,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>) -> wgpu::RenderPass<'a>
    {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: if let Some(v) = view {v} else {&self.view},
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if let Some(c) = clear_color {wgpu::LoadOp::Clear(c)} else {wgpu::LoadOp::Load},
                    store: true,
                },
            })],
            depth_stencil_attachment,
        })
    }

    pub fn finish(self, state: &State) {
        state.queue.submit(std::iter::once(self.encoder.finish()));
        self.out.present();
    }
}