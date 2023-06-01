pub struct Window {
    win: winit::window::Window,
    cursor_visibility: bool,
    cursor_grabbed: bool,
}

impl Window {
    pub fn new(win: winit::window::Window) -> Self {
        Self { win, cursor_visibility: true, cursor_grabbed: false }
    }

    pub fn get_raw(&self) -> &winit::window::Window {&self.win}
    pub fn id(&self) -> winit::window::WindowId {self.win.id()}
    pub fn inner_size(&self) -> winit::dpi::PhysicalSize<u32> {self.win.inner_size()}

    pub fn switch_cursor_visibility(&mut self) {
        self.win.set_cursor_visible(!self.cursor_visibility);
        self.cursor_visibility = !self.cursor_visibility;
    }

    pub fn switch_cursor_grab(&mut self) {
        self.cursor_grabbed = !self.cursor_grabbed;
        let mode = if self.cursor_grabbed { winit::window::CursorGrabMode::Confined } else { winit::window::CursorGrabMode::None };
        if let Err(e) = self.win.set_cursor_grab(mode) {
            log::error!("{e}");
        }
    }

    pub fn set_title(&mut self, t: &str) {
        self.win.set_title(t);
    }

    pub fn request_redraw(&self) {self.win.request_redraw();}
}