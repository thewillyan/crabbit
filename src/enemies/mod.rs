use crate::Pos;

pub mod walls;

pub trait Hitmap {
    fn hits(&self, pos: &Pos) -> bool;
}
