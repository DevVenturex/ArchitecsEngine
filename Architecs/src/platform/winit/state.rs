use winit::{application::ApplicationHandler, event_loop::EventLoop};

use crate::{app::App, platform::winit::winit_windows::WinitWindows};

pub struct WinitWindowsState {
    app: App,
}

impl WinitWindowsState {
    pub fn new(app: App) -> Self {
        Self {
            app,
        }
    }
}

impl ApplicationHandler for WinitWindowsState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}

pub fn winit_runner(mut app: App, event_loop: EventLoop<()>) {
    app.create_event_loop_proxy(event_loop.create_proxy());

    let mut winit_state = WinitWindowsState::new(app);

    if let Err(err) = event_loop.run_app(&mut winit_state) {
        panic!("winit event loop returned an error: {err}");
    }
}