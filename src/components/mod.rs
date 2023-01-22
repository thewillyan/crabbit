pub mod player;
pub mod stage;
pub mod enemies;
pub mod hud;

pub use stage::Stage;
pub use hud::Hud;

pub trait DynComp {
    fn update(&mut self);
    fn reset(&mut self);
}
