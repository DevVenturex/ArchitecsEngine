mod service;

pub use service::*;

pub struct ServiceLocator {
    services: Vec<Box<dyn Service>>,
}

impl ServiceLocator {
    pub fn empty() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn add_service<S: Service + ServiceBuilder + 'static>(&mut self) {
        let service = S::build();
        self.services.push(Box::new(service));
    }
}
