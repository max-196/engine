pub mod renderer;
pub mod window;
pub mod time;
pub mod input;
pub mod path;
pub mod gui;

mod player;
mod render;

use player::Player;
use crate::err::Error;

use self::gui::GuiElement;

pub use {
    window::Window,
    renderer::Renderer,
    time::Time,
    input::InputMapping,
    input::InputManager,
    path::PathManager,
    gui::Gui,
};

pub struct Client<'a> {
    pub renderer: Renderer,
    pub player:   Player,
    pub window:   Window,
    pub time:     Time,
    pub input:    InputManager<'a>,
    pub path_m:   PathManager,
    pub gui:      Gui,
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

        let mut gui = gui::Gui::new(&renderer.state, &path_m, 8)?;

        gui.add(
            GuiElement::new(0)
                .set_pos(gui::Pos::Pixel2D((50, 50).into()))
                .set_size(gui::Size::PixelSquare(50))
                .set_color((1.0, 0.0, 0.0, 1.0))
                .set_min_size(gui::Size::Pixel2D((100, 100).into())),
            Some(&renderer.state)
        );

        gui.add(
            GuiElement::new(0)
                .set_size(gui::Size::RelYSquare(0.5))
                .set_color((0.0, 1.0, 0.0, 1.0))
                .set_circle(true)
                .set_origin(gui::element::Origin::BottomRight)
                .set_pos(gui::Pos::Rel2D((1., 1.).into())),
            Some(&renderer.state)
        );

        Ok(Client {renderer, player, window, time, input, path_m, gui })
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
        self.gui.resize(&self.renderer.state);
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