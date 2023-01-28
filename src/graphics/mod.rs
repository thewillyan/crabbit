pub mod sprite;
pub mod object;

pub use sprite::Sprite;
pub use object::Obj;

/// Defines behavior of objects that can be rendered.
pub trait Render {
    /// Renders the object on the screen.
    fn render<O: std::io::Write>(&self, out: &mut O);
    /// Erase the object from the screen.
    fn erase<O: std::io::Write>(&self, out: &mut O);
}

/// Terminal position (row, col).
#[derive(Clone)]
pub struct Pos {
    pub col: u16,
    pub row: u16,
}

/// Object size (width, height).
#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
