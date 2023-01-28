use std::{slice::Chunks, fs::File, io::{prelude::*, BufReader}};

use crate::graphics::Size;

pub struct Sprite {
    size: Size,
    chars: Vec<char>,
}

impl Sprite {
    /// return a new sprite with the given width and charaters
    pub fn new(chars: Vec<char>, width: u16) -> Sprite {
        let n = chars.len() as u16;

        if width > n || n % width != 0 {
            panic!("Invalid width for a char matrix with {} values.", n);
        }

        let height = n / width;
        let size = Size { width, height };
        Sprite { size, chars }
    }

    /// return a sprite extracting its charaters from a text file
    pub fn from_file(fpath: &str) -> Sprite {
        let file = File::open(fpath).expect("Failed to read the sprite file!");
        let reader = BufReader::new(file);
        let mut ascii_matrix = Vec::new();
        let mut width = 0;

        let lines: Vec<_> = reader.lines().map(|line| {
            let line = line.expect("Failed to read file line!");
            width = width.max(line.len());
            line
        }).collect();

        lines.into_iter().for_each(|mut line| {
            let padding = " ".repeat(width - line.len());
            line.push_str(&padding);
            ascii_matrix.extend(line.chars());
        });

        Self::new(ascii_matrix, width as u16)
    }

    /// returns the sprite size as a tuple (width, height)
    pub fn size(&self) -> (u16, u16) {
        (self.size.width, self.size.height)
    }

    /// returns the number of characters that the sprite has
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// returns the element at the i line and j column
    pub fn get(&self, i: u16, j: u16) -> Option<&char> {
        let (i, j) = (i as usize, j as usize);
        let idx = (i * self.size.width as usize) + j;
        self.chars.get(idx)
    }

    /// update the visual representation of the sprite
    pub fn set_ascii(&mut self, ascii_matrix: Vec<char>) {
        if ascii_matrix.len() != self.chars.len() {
            panic!("Failed to update sprite: new ascii matrix has different size.");
        }
        self.chars = ascii_matrix;
    }

    /// stretch sprite "size" times with a given char
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

    /// returns a iterator over the sprite rows
    pub fn rows(&self) -> Chunks<char> {
        self.chars.chunks(self.size.width as usize)
    }
}
