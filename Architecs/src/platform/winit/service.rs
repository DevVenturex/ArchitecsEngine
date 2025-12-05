use crate::{app::App, platform::winit::winit_windows::WinitWindows, service_locator::{Service, ServiceBuilder}};

pub struct WinitService {
}

impl Service for WinitService {
    fn init(&self, app: &mut App) {

    }

    fn update(&mut self) {
        todo!()
    }

    fn cleanup(&mut self) {
        todo!()
    }
}

impl ServiceBuilder for WinitService {
    fn build(app: &mut App) -> Self where Self: Service {
        Self {
        }
    }
}