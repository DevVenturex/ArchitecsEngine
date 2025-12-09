use winit::event_loop::{self, EventLoop, EventLoopProxy};

use crate::{app::App, platform::winit::{state::WinitWindowsState, window_handler::UserEvent, winit_windows::WinitWindows}, service_locator::Service, window::Window};

pub struct WinitService {
    winit_window_state: WinitWindowsState,
    proxy: EventLoopProxy<UserEvent>,
}

impl Service for WinitService {
    fn build(app: &mut App) -> Self where Self: Sized {
        let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
        Self {
            winit_window_state: WinitWindowsState::new(app),
            proxy: event_loop.create_proxy(),
        }
    }

    fn update(&mut self) {
    }

    fn cleanup(&mut self) {
    }
}

impl WinitService {
    pub fn run_winit_window_state() {
        
    }

    pub fn create_window(&mut self, window: Window) {
        self.proxy.send_event(UserEvent::CreateWindow(window));
    }
}
