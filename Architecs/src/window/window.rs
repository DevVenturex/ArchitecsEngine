use std::{any::Any, marker::PhantomData, ops::Deref, sync::Arc};

use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::window::description::{WindowDescriber, DefaultWindowDescription};

pub type DefaultWindow = Window<DefaultWindowDescription>;

#[derive(Debug, Clone)]
pub struct Window<T: WindowDescriber> {
    desc: T,
    _window: Arc<dyn Any + Send + Sync>,
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

impl<T: WindowDescriber + 'static> Window<T> {
    pub fn new<W: HasWindowHandle + HasDisplayHandle + 'static>(desc: T, window: &WindowWrapper<W>) -> Result<Window<T>, HandleError> {
        Ok(Self {
            _window: window.reference.clone(),
            window_handle: window.window_handle()?.as_raw(),
            display_handle: window.display_handle()?.as_raw(),
            desc,  
        })
    }

    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }

    pub fn get_display_handle(&self) -> RawDisplayHandle {
        self.display_handle
    }

    // TODO: implement functions from describer here
    pub fn description(&self) -> &T {
        &self.desc
    }
}

#[derive(Debug)]
pub struct WindowWrapper<W> {
    reference: Arc<dyn Any + Send + Sync>,
    ty: PhantomData<W>,
}

impl<W: Send + Sync + 'static> WindowWrapper<W> {
    pub fn new(window: W) -> WindowWrapper<W> {
        WindowWrapper {
            reference: Arc::new(window),
            ty: PhantomData,
        }
    }
}

impl<W: 'static> Deref for WindowWrapper<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.reference.downcast_ref::<W>().unwrap()
    }
}