pub mod player;
pub mod stage;
pub mod enemies;
pub mod hud;

pub use stage::Stage;
pub use hud::Hud;

/// Define behavior of Dynamic Components (components that moves).
pub trait DynComp {
    /// Update component (go to the next frame).
    fn update(&mut self);
    /// Reset component to the original state.
    fn reset(&mut self);
}
