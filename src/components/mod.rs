pub mod player;
pub mod stage;
pub mod enemies;

pub use stage::Stage;

pub trait Comp {
    fn update(&mut self);
    fn reset(&mut self);
}
