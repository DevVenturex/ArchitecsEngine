use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::app::App;
use crate::platform::winit::winit_windows::WinitWindows;
use crate::service_locator::{Service, ServiceBuilder};
use crate::window::{Window, WindowWrapper};
use crate::window::description::WindowDescriber;

pub struct WindowService {
    windows: Vec<Window>,
    primary_window: Option<Window>,
}

impl Service for WindowService {
    fn init(&self, app: &mut App) {
        
    }

    fn update(&mut self) {
        
    }

    fn cleanup(&mut self) {
        todo!()
    }
}

impl ServiceBuilder for WindowService {
    fn build(app: &mut App) -> Self where Self: Service {
        Self {
            windows: Vec::new(),
            winit_windows: WinitWindows::default(),
            primary_window: None,
        }
    }
}

impl WindowService {
    pub fn create_window<W: HasWindowHandle + HasDisplayHandle + 'static>(
        &mut self,
        desc: Box<dyn WindowDescriber>, 
        window: &WindowWrapper<W>
    ) {
        let window = Window::new(desc, window).unwrap();
        if self.primary_window.is_none() {
            self.primary_window = Some(window.clone());
        }
        self.windows.push(window);
    }
}