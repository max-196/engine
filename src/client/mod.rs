pub mod renderer;
pub mod window;
pub mod time;
pub mod input;
pub mod path;

mod player;
mod render;

use player::Player;
use crate::err::Error;

pub use {
    window::Window,
    renderer::Renderer,
    time::Time,
    input::InputMapping,
    input::InputManager,
    path::PathManager,
};

pub struct Client<'a> {
    pub renderer: Renderer,
    pub player:   Player,
    pub window:   Window,
    pub time:     Time,
    pub input:    InputManager<'a>,
    pub path_m:   PathManager,
}

impl <'a> Client<'a> {
    pub async fn new(window: winit::window::Window) -> Result<Client<'a>, Error> {
        let path_m = PathManager::new(
            "assets/shaders",
            "assets/models",
            "assets/cubemaps",
            "assets/textures"
        );

        let mut input = InputManager::new();

        let window = Window::new(window);

        let renderer = Renderer::new(&window, &path_m).await?;

        let player = Player::new(&renderer.state.device, &renderer.state.queue, renderer.state.size.x, renderer.state.size.y, &mut input);

        let time = Time::new(&renderer.state);

        input.register_mapping("cursor_hide", winit::event::VirtualKeyCode::F1);
        input.register_mapping("cursor_grab", winit::event::VirtualKeyCode::F2);

        Ok(Client {renderer, player, window, time, input, path_m })
    }

    pub fn device_input(&mut self, event: &winit::event::DeviceEvent) {
        self.input.device_input(event);
    }

    pub fn window_input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.input.window_input(event);

        false
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
        self.player.resize(new_size.width, new_size.height);
    }


    pub fn update(&mut self) {
        self.time.update(&self.renderer.state);
        self.renderer.update(&self.time);
        self.player.update(&self.time, &self.renderer.state.queue, &self.input);
        self.time.every(50, || self.window.set_title(&format!("{:.2}", self.time.fps.avg_fps)));

        // Rare update
        self.time.every(3, || {
            if self.input.get_key_once("cursor_hide") {self.window.switch_cursor_visibility()}
            if self.input.get_key_once("cursor_grab") {self.window.switch_cursor_grab()}
        });


        self.input.update();
    }

    pub fn window_id(&self) -> winit::window::WindowId {
        self.window.id()
    }
}