use std::fmt::Debug;

pub trait WindowDescriber {
    fn resolution(&self) -> &WindowResolution;
    fn resizeable(&self) -> bool;
    fn title(&self) -> String;
    fn decorations(&self) -> bool;
    fn transparent(&self) -> bool;
    fn focused(&self) -> bool;
    fn visible(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultWindowDescription {
    resolution: WindowResolution,
    resizeable: bool,
    title: &'static str,
    decorations: bool,
    transparent: bool,
    focused: bool,
    visible: bool,
}

impl Default for DefaultWindowDescription {
    fn default() -> Self {
        Self {
            title: "App",
            resizeable: true,
            decorations: true,
            transparent: true,
            focused: true,
            visible: true,
            resolution: WindowResolution::default(),
        }
    }
}

impl WindowDescriber for DefaultWindowDescription {
    fn resolution(&self) -> &WindowResolution {
        &self.resolution
    }

    fn resizeable(&self) -> bool {
        self.resizeable
    }

    fn title(&self) -> String {
        self.title.to_string()
    }

    fn decorations(&self) -> bool {
        self.decorations
    }

    fn transparent(&self) -> bool {
        self.transparent
    }

    fn focused(&self) -> bool {
        self.focused
    }

    fn visible(&self) -> bool {
        self.visible
    }
}
