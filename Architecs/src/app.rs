use std::sync::Arc;

use winit::event_loop::EventLoopProxy;

use crate::{platform::winit::EventLoopProxyWrapper, service_locator::{Service, ServiceBuilder, ServiceLocator}};

#[derive(Default)]
pub struct App {
    pub service_locator: ServiceLocator,
    pub event_loop_proxy: Option<EventLoopProxy<()>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_service<S: Service + ServiceBuilder + 'static>(&mut self) {
        self.service_locator.add_service::<S>();
    }

    pub fn get_service<S: Service + 'static>(&self) -> &S {

    }

    pub fn create_event_loop_proxy(&mut self, proxy: EventLoopProxy<()>) {
        self.event_loop_proxy = Some(proxy);
    }
}