pub mod sprite;
pub mod object;

use std::io::Stdout;
use termion::raw::RawTerminal;

pub use sprite::Sprite;
pub use object::Obj;

pub type TermOut = RawTerminal<Stdout>;

/// Defines behavior of objects that can be rendered.
pub trait Render {
    /// Renders the object on the screen.
    fn render(&self, out: &mut TermOut);
    /// Erase the object from the screen.
    fn erase(&self, out: &mut TermOut);
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
