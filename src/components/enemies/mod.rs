use crate::graphics::Pos;

pub mod walls;

pub use walls::Walls;

/// Defines behavior of components that can hit the player.
pub trait Hitmap {
    /// Verify if the component hit the given position.
    fn hits(&self, pos: &Pos) -> bool;
}
