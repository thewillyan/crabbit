/// Player of the game.
pub mod player;
/// Game stage.
pub mod stage;
/// Player's enemies.
pub mod enemies;
/// Game HUD.
pub mod hud;

pub use stage::Stage;
pub use hud::Hud;
pub use player::Player;

/// Define behavior of Dynamic Components (components that moves).
pub trait DynComp {
    /// Update component (go to the next frame).
    fn update(&mut self);
    /// Reset component to the original state.
    fn reset(&mut self);
}
