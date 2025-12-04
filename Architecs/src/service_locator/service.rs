pub trait Service {
    fn init(&self);
    fn update(&mut self);
    fn cleanup(&mut self);
}

pub trait ServiceBuilder {
    fn build() -> Self where Self: Service;
}

pub struct DumpService;

impl Service for DumpService {
    fn init(&self) {        
    }

    fn update(&mut self) {
    }

    fn cleanup(&mut self) {
    }
}

impl ServiceBuilder for DumpService {
    fn build() -> Self where Self: Service {
        Self
    }
}