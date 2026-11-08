use crate::gfx::Gfx;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    gfx: Option<Gfx>,
    webview: Option<wry::WebView>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("Critter")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let gfx =
            pollster::block_on(Gfx::new(window.clone())).expect("Failed to initialize graphics");
        let webview = wry::WebViewBuilder::new()
            .with_url("https://marlothacker.com")
            .with_bounds(wry::Rect {
                position: winit::dpi::LogicalPosition::new(400, 0).into(),
                size: winit::dpi::LogicalSize::new(400, 600).into(),
            })
            .build_as_child(&window)
            .expect("Faield to create webview");

        window.request_redraw();
        self.window = Some(window);
        self.gfx = Some(gfx);
        self.webview = Some(webview);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(gfx) = &mut self.gfx {
                    gfx.resize(physical_size);
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gfx) = &mut self.gfx {
                    if let Err(err) = gfx.render() {
                        log::error!("Render error: {err:?}");
                    }
                }
            }
            _ => (),
        }
    }
}
