use crate::app::App;

pub trait Service {
    fn build(app: &mut App) -> Self where Self: Sized;
    fn update(&mut self);
    fn cleanup(&mut self);
}