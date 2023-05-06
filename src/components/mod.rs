/// Player's enemies.
pub mod enemies;
/// Head-up display components.
pub mod hud;
/// Player of the game.
pub mod player;
/// Game stage.
pub mod stage;

pub use player::Player;
pub use stage::Stage;

/// Define behavior of *"Dynamic Components"* (components that moves).
pub trait DynComp {
    /// Update component (go to the next frame).
    fn update(&mut self);
    /// Reset component to the original state.
    fn reset(&mut self);
}
