use std::ops::Deref;

use winit::event_loop::EventLoopProxy;

pub mod service;
pub mod winit_windows;
pub mod state;

pub struct EventLoopProxyWrapper<T: 'static>(EventLoopProxy<T>);

impl<T: 'static> Deref for EventLoopProxyWrapper<T> {
    type Target = EventLoopProxy<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} 

pub struct DisplayHandleWrapper(pub winit::event_loop::OwnedDisplayHandle);

impl Deref for DisplayHandleWrapper {
    type Target = winit::event_loop::OwnedDisplayHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}