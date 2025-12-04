use crate::service_locator::{Service, ServiceBuilder};

pub struct WindowService {

}

impl Service for WindowService {
    fn init(&self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn cleanup(&mut self) {
        todo!()
    }
}

impl ServiceBuilder for WindowService {
    fn build() -> Self where Self: Service {
        Self {}
    }
}