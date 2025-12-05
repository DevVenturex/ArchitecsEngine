use std::{any::Any, marker::PhantomData, ops::Deref, sync::Arc};

use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::window::description::WindowDescriber;

#[derive(Debug, Clone)]
pub struct Window {
    _window: Arc<dyn Any + Send + Sync>,
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
    
}

impl Window {
    pub fn new<W: HasWindowHandle + HasDisplayHandle + 'static>(_desc: Box<dyn WindowDescriber>, window: &WindowWrapper<W>) -> Result<Window, HandleError> {
        Ok(Self {
            _window: window.reference.clone(),
            window_handle: window.window_handle()?.as_raw(),
            display_handle: window.display_handle()?.as_raw(),
        })
    }

    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }

    pub fn get_display_handle(&self) -> RawDisplayHandle {
        self.display_handle
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