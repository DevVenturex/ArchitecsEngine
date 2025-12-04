pub mod window_service;

pub trait Window {
    pub fn title(&self) -> String;
    
}