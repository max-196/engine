pub mod chromatic_aberration;
pub mod box_blur;

pub trait PostFx {
    fn set_effect<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.draw(0..6, 0..1);
    }
    fn label(&self) -> &'static str {"Post-Processing effect"}
}