use crate::graphics::Pos;

pub mod walls;

pub use walls::Walls;

pub trait Hitmap {
    fn hits(&self, pos: &Pos) -> bool;
}
