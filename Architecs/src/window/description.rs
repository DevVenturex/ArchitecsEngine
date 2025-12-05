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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowResolution {
    physical_width: u32,
    physical_height: u32,
    scale_factor_override: Option<f32>,
    scale_factor: f32,
}

impl Default for WindowResolution {
    fn default() -> Self {
        Self {
            physical_width: 1280,
            physical_height: 720,
            scale_factor_override: None,
            scale_factor: 1.0,
        }
    }
}

impl WindowResolution {
    pub fn new(physical_width: u32, physical_height: u32) -> Self {
        Self {
            physical_width,
            physical_height,
            ..Default::default()
        }
    }

    pub fn with_scale_factor_override(mut self, scale_factor_override: f32) -> Self {
        self.set_scale_factor_override(Some(scale_factor_override));
        self
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.physical_width() as f32 / self.scale_factor()
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.physical_height() as f32 / self.scale_factor()
    }

    #[inline]
    pub fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }

    #[inline]
    pub fn physical_width(&self) -> u32 {
        self.physical_width
    }

    #[inline]
    pub fn physical_height(&self) -> u32 {
        self.physical_height
    }

    #[inline]
    pub fn physical_size(&self) -> (u32, u32) {
        (self.physical_width, self.physical_height)
    }

    
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor_override
            .unwrap_or_else(|| self.base_scale_factor())
    }

    #[inline]
    pub fn base_scale_factor(&self) -> f32 {
        self.scale_factor
    }

    #[inline]
    pub fn scale_factor_override(&self) -> Option<f32> {
        self.scale_factor_override
    }

    #[inline]
    pub fn set(&mut self, width: f32, height: f32) {
        self.set_physical_resolution(
            (width * self.scale_factor()) as u32,
            (height * self.scale_factor()) as u32,
        );
    }

    #[inline]
    pub fn set_physical_resolution(&mut self, width: u32, height: u32) {
        self.physical_width = width;
        self.physical_height = height;
    }
    
        #[inline]
        pub fn set_scale_factor(&mut self, scale_factor: f32) {
            self.scale_factor = scale_factor;
        }

    #[inline]
    pub fn set_scale_factor_and_apply_to_physical_size(&mut self, scale_factor: f32) {
        self.scale_factor = scale_factor;
        self.physical_width = (self.physical_width as f32 * scale_factor) as u32;
        self.physical_height = (self.physical_height as f32 * scale_factor) as u32;
    }

    #[inline]
    pub fn set_scale_factor_override(&mut self, scale_factor_override: Option<f32>) {
        self.scale_factor_override = scale_factor_override; 
    }
}

impl From<(u32, u32)> for WindowResolution {
    fn from((width, height): (u32, u32)) -> Self {
        WindowResolution::new(width, height)
    }
}

impl From<[u32; 2]> for WindowResolution {
    fn from([width, height]: [u32; 2]) -> Self {
        WindowResolution::new(width, height)
    }
}