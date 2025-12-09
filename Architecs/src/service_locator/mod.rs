mod service;

pub use service::*;

use crate::app::App;

pub struct ServiceLocator {
    services: Vec<Box<dyn Service>>,
}

impl Default for ServiceLocator {
    fn default() -> Self {
        Self::empty()
    }
}

impl ServiceLocator {
    pub fn empty() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn add_service<S: Service + 'static>(&mut self, app: &mut App) {
        self.services.push(Box::new(S::build(app)));
    }
}   
