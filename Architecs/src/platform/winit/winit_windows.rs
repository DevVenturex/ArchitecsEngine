use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::{Window as WinitWindow, WindowId}};

use crate::window::Window;

#[derive(Debug, Default)]
pub struct WinitWindows {
    pub windows: HashMap<WindowId, Arc<WinitWindow>>,
    _not_send_sync: PhantomData<*const ()>,
}

impl WinitWindows {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            _not_send_sync: PhantomData,
        }
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: &Window,
    ) {
        let mut window_attributes = WinitWindow::default_attributes();
        let logical_size = LogicalSize::new(window.width(), window.height());
        if let Some(sf) = window.resolution.scale_factor_override() {
            let inner_size = logical_size.to_physical::<f64>(sf.into());
            window_attributes = window_attributes.with_inner_size(inner_size);
        } else {
            window_attributes = window_attributes.with_inner_size(logical_size)
        }

        window_attributes = window_attributes
            .with_resizable(window.resizeable)
            .with_decorations(window.decorations)
            .with_transparent(window.transparent)
            .with_active(window.focused)
            .with_title(window.title.clone());

        let winit_window = event_loop.create_window(window_attributes).unwrap();
        winit_window.set_visible(window.visible);
        let id = winit_window.id();
        self.windows.insert(id, Arc::new(winit_window));
    }

    pub fn get_window(&self, window_id: &WindowId) -> Option<&Arc<WinitWindow>> {
        self.windows.get(window_id)
    }

    pub fn remove_window(&mut self, window_id: &WindowId) -> Option<Arc<WinitWindow>> {
        self.windows.remove(window_id)
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }
}