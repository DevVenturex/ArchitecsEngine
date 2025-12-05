use crate::app::App;

pub trait Service {
    fn init(&self, app: &mut App);
    fn update(&mut self);
    fn cleanup(&mut self);
}

pub trait ServiceBuilder {
    fn build(app: &mut App) -> Self where Self: Service;
}

pub struct DumpService;

impl Service for DumpService {
    fn init(&self, app: &mut App) {        
    }

    fn update(&mut self) {
    }

    fn cleanup(&mut self) {
    }
}

impl ServiceBuilder for DumpService {
    fn build(app: &mut App) -> Self where Self: Service {
        Self
    }
}