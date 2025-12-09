use crate::app::App;
use crate::platform::winit::service::WinitService;
use crate::service_locator::Service;
use crate::window::Window;

pub struct WindowService {
    windows: Vec<Window>,
    primary_window: Option<Window>,
}

impl Service for WindowService {
    fn build(app: &mut App) -> Self where Self: Sized {
        app.add_service::<WinitService>();
        Self {
            windows: Vec::new(),
            primary_window: None,
        }
    }

    fn update(&mut self) {
    }

    fn cleanup(&mut self) {
    }
}

impl WindowService {
    pub fn create_window(
        &mut self,
        app: &mut App,
        window: Window,
    ) {
        app.create_winit_window(window);
    }
}