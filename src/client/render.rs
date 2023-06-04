use crate::client::Client;

impl <'a> Client<'a> {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        // {
        //     unreachable!();
        //     let mut frame = self.renderer.get_frame();
        //     {
        //         let geometry_pass = frame.depth_pass();
        //         geometry_pass.set_bg(3, self.renderer.time.uf.bg.group);
        //         geometry_pass.set_vbuffer(1, self.renderer.instance_buffer.slice());
        //         geometry_pass.set_pipeline(self.renderer.light.pipeline());
        //         geometry_pass.draw_light_model(self.renderer.model)
        //     }
        //     self.renderer.render_effects(frame);
        // }


        self.renderer.render(self.player.camera_bg(), &self.time, &self.input, &self.gui)
    }
}