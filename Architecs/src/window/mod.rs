pub mod window_service;

pub trait Window {
    fn title(&self) -> String;
}