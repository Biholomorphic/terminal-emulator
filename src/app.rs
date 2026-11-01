use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

pub struct App {
    window: Option<Window>,
}

impl App {
    pub fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("Critter")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
        self.window = Some(event_loop.create_window(attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}
