use std::{collections::HashMap, marker::PhantomData};

use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::{Window as WinitWindow, WindowId}};

use crate::{platform::winit::winit_windows, window::{Window, WindowWrapper, description::WindowDescriber}};

#[derive(Debug, Default)]
pub struct WinitWindows {
    pub windows: HashMap<WindowId, WindowWrapper<WinitWindow>>,
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
        desc: impl WindowDescriber,
    ) -> &WindowWrapper<WinitWindow> {
        let mut window_attributes = WinitWindow::default_attributes();
        let logical_size = LogicalSize::new(desc.resolution().width(), desc.resolution().height());
        if let Some(sf) = desc.resolution().scale_factor_override() {
            let inner_size = logical_size.to_physical::<f64>(sf.into());
            window_attributes = window_attributes.with_inner_size(inner_size);
        } else {
            window_attributes = window_attributes.with_inner_size(logical_size)
        }

        window_attributes = window_attributes
            .with_resizable(desc.resizeable())
            .with_decorations(desc.decorations())
            .with_transparent(desc.transparent())
            .with_active(desc.focused());

        window_attributes = window_attributes.with_title(desc.title());
        let winit_window = event_loop.create_window(window_attributes).unwrap();
        winit_window.set_visible(desc.visible());
        let id = winit_window.id();
        self.windows.insert(id, WindowWrapper::new(winit_window));
        self.windows.get(&id).unwrap()
    }

    pub fn get_window(&self, window_id: &WindowId) -> Option<&WindowWrapper<WinitWindow>> {
        self.windows.get(window_id)
    }

    pub fn remove_window(&mut self, window_id: &WindowId) -> Option<WindowWrapper<WinitWindow>> {
        self.windows.remove(window_id)
    }
}