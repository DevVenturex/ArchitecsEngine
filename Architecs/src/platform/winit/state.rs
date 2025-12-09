use winit::application::ApplicationHandler;

use crate::app::App;

pub struct WinitWindowsState {
}

impl WinitWindowsState {
    pub fn new(app: &mut App) -> Self {
        Self {
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