use crate::client::InputManager;

use {
    crate::client::renderer::{
        Renderer, resources::model::{DrawModel, DrawLight}, state::RenderState,
    },
    crate::client::Time
};


impl Renderer {
    pub fn render(&mut self, camera_bg: &wgpu::BindGroup, time: &Time, inp: &InputManager) -> Result<(), wgpu::SurfaceError> {
        let mut render_state = RenderState::new(&self.state)?;

        {
            let mut render_pass = render_state.render_pass(
                Some("Geometry Render Pass"),
                Some(self.framebuffer.target_view()),
                Some(wgpu::Color {r: 0.1, g: 0.2, b: 0.3, a: 1.0}),
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                })
            );


            render_pass.set_bind_group(3, &time.uf.bg.group, &[]);

            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            render_pass.set_pipeline(self.light.pipeline());
            render_pass.draw_light_model(&self.model, camera_bg, self.light.bg());

            render_pass.set_pipeline(&self.pipeline.pipeline);
            render_pass.draw_model_instanced(&self.model, 0..self.instances.len() as u32, camera_bg, self.light.bg());
        }

        {
            let mut render_pass = render_state.render_pass(
                Some("Skybox Render Pass"),
                Some(self.framebuffer.target_view()),
                None,
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    }),
                    stencil_ops: None,
                })
            );
            render_pass.set_bind_group(0, camera_bg, &[]);
            render_pass.set_bind_group(1, &self.cubemap.bg.group, &[]);
            render_pass.set_pipeline(&self.sky_pipeline.pipeline);
            render_pass.draw(0..36, 0..1);
        }

        self.framebuffer.swap_buffers();

        if inp.mouse.button.left {
            for i in 0..self.postfx.len() {
                self.render_fx(&mut render_state, i);
            }
        }

        self.render_framebuffer(&mut render_state);

        render_state.finish(&self.state);

        Ok(())
    }

    fn render_fx(&mut self, render_state: &mut RenderState, fx_index: usize) {
        {
            let fx = &self.postfx[fx_index];

            let mut render_pass = render_state.render_pass(
                Some(&(fx.label().to_owned() + " Render Pass")),
                Some(self.framebuffer.target_view()),
                None,
                None
            );

            render_pass.set_bind_group(0, self.framebuffer.sample_bg(), &[]);
            fx.set_effect(&mut render_pass);
            fx.draw(&mut render_pass);
        }
        self.framebuffer.swap_buffers();
    }

    fn render_framebuffer(&mut self, render_state: &mut RenderState) {
        let mut render_pass = render_state.render_pass(
            Some("Framebuffer and Color Correction Render Pass"),
            None, None, None);

        render_pass.set_pipeline(self.framebuffer.pipeline());
        render_pass.set_bind_group(0, self.framebuffer.sample_bg(), &[]);
        render_pass.draw(0..6, 0..1);
    }
}