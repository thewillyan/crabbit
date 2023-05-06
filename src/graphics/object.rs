//! Provides structs that represents visual objects as characters in a terminal
//! screen.

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    slice::Chunks,
};
use termion::{
    color::{Bg, Color, Fg, Reset},
    cursor,
};

use super::{Pos, Render, Size, TermOut};

/// Graphical representation of an [`Obj`]. An ANSII Matrix.
#[derive(Debug)]
pub struct Sprite {
    size: Size,
    chars: Vec<char>,
}

impl Sprite {
    /// Return a new sprite with the given width and charaters.
    pub fn new(chars: Vec<char>, width: u16) -> Sprite {
        let n = chars.len() as u16;

        if width > n || n % width != 0 {
            panic!("Invalid width for a char matrix with {} values.", n);
        }

        let height = n / width;
        let size = Size { width, height };
        Sprite { size, chars }
    }

    /// Return a sprite extracting its charaters from a text file.
    pub fn from_file(fpath: &str) -> Sprite {
        let file = File::open(fpath).expect("Failed to read the sprite file!");
        let reader = BufReader::new(file);
        let mut ascii_matrix = Vec::new();
        let mut width = 0;

        let lines: Vec<_> = reader
            .lines()
            .map(|line| {
                let line = line.expect("Failed to read file line!");
                width = width.max(line.len());
                line
            })
            .collect();

        lines.into_iter().for_each(|mut line| {
            let padding = " ".repeat(width - line.len());
            line.push_str(&padding);
            ascii_matrix.extend(line.chars());
        });

        Self::new(ascii_matrix, width as u16)
    }

    /// Returns the sprite size as a tuple (width, height).
    pub fn size(&self) -> (u16, u16) {
        (self.size.width, self.size.height)
    }

    /// Returns the element at the `i` line and `j` column.
    pub fn get(&self, i: u16, j: u16) -> Option<&char> {
        let (i, j) = (i as usize, j as usize);
        let idx = (i * self.size.width as usize) + j;
        self.chars.get(idx)
    }

    /// Update the visual representation of the sprite.
    pub fn set_ascii(&mut self, ascii_matrix: Vec<char>) {
        if ascii_matrix.len() != self.chars.len() {
            panic!("Failed to update sprite: new ascii matrix has different size.");
        }
        self.chars = ascii_matrix;
    }

    /// Stretch sprite "size" times with a given char
    pub fn stretch(&mut self, size: usize, c: char) {
        let padding = vec![c; size];
        let width = self.size.width as usize;
        let height = self.size.height as usize;

        let mut idx = width;
        for _ in 0..height {
            self.chars.splice(idx..idx, padding.iter().cloned());
            idx += width + size;
        }

        self.size.width += size as u16;
    }

    /// Returns a iterator over the sprite rows.
    pub fn rows(&self) -> Chunks<char> {
        self.chars.chunks(self.size.width as usize)
    }
}

/// A object that can be rendered in the screen.
#[derive(Debug)]
pub struct Obj {
    /// Position.
    pub pos: Pos,
    /// Graphical representation.
    pub sprite: Sprite,
    /// Foreground color.
    pub color: String,
}

impl Obj {
    /// Returns a new instance of `Obj`.
    pub fn new<C: Color>(pos: Pos, sprite: Sprite, color: &Fg<C>) -> Obj {
        Obj {
            pos,
            sprite,
            color: color.to_string(),
        }
    }
}

impl Render for Obj {
    fn render(&self, out: &mut TermOut) {
        let mut row = self.pos.row;
        for line in self.sprite.rows() {
            let line: String = line.iter().collect();
            write!(
                out,
                "{}{}{}{}{}",
                cursor::Goto(self.pos.col, row),
                self.color,
                line,
                Fg(Reset),
                Bg(Reset)
            )
            .unwrap();
            row += 1;
        }
    }

    fn erase(&self, out: &mut TermOut) {
        let row = self.pos.row;
        let (width, height) = self.sprite.size();
        let overwrite = " ".repeat(width as usize);
        for r in row..(row + height) {
            write!(out, "{}{}", cursor::Goto(self.pos.col, r), overwrite).unwrap();
        }
    }
}
