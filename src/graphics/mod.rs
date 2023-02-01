use std::io::Stdout;
use termion::raw::RawTerminal;

pub mod object;

/// Raw terminal output.
pub type TermOut = RawTerminal<Stdout>;

/// Defines behavior of objects that can be rendered.
pub trait Render {
    /// Renders the object on the screen.
    fn render(&self, out: &mut TermOut);
    /// Erase the object from the screen.
    fn erase(&self, out: &mut TermOut);
}

/// Terminal position (row, col).
#[derive(Debug, Clone)]
pub struct Pos {
    /// Column
    pub col: u16,
    /// Row
    pub row: u16,
}

/// Object size (width, height).
#[derive(Debug, Clone)]
pub struct Size {
    /// Horizontal size.
    pub width: u16,
    /// Vertical size.
    pub height: u16,
}

