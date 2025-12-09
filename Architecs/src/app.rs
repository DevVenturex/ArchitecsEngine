use crate::{platform::winit::service::WinitService, service_locator::{Service, ServiceLocator}, window::{Window, service::WindowService}};

#[derive(Default)]
pub struct App {
    service_locator: ServiceLocator,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_service<S: Service + 'static>(&mut self) {
        self.service_locator.add_service::<S>();
    }

    pub fn get_service<S: Service + 'static>(&self) -> S {

    }

    pub fn create_winit_window(&mut self, window: Window) {
        self.get_service::<WinitService>().create_window(window);
    }
}