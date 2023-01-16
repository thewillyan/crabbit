use std::io::Write;

pub mod sprite;
pub mod object;

pub use sprite::Sprite;
pub use object::Obj;

pub trait Render {
    fn render<O: Write>(&self, out: &mut O);
    fn erase<O: Write>(&self, out: &mut O);
}

#[derive(Clone)]
pub struct Pos {
    pub col: u16,
    pub row: u16,
}

#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
