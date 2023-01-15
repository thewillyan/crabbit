use crate::Size;
use std::slice::Chunks;

pub struct Sprite {
    size: Size,
    chars: Vec<char>,
}

impl Sprite {
    pub fn new(chars: Vec<char>, width: u16) -> Sprite {
        let n = chars.len() as u16;

        if width > n || n % width != 0 {
            panic!("Invalid width for a char matrix with {} values.", n);
        }

        let height = n / width;
        let size = Size { width, height };
        Sprite { size, chars }
    }

    // returns the sprite size as a tuple (width, height)
    pub fn size(&self) -> (u16, u16) {
        (self.size.width, self.size.height)
    }

    // returns the number of characters that the sprite has
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    // returns the element at the i line and j column
    pub fn get(&self, i: u16, j: u16) -> Option<&char> {
        let (i, j) = (i as usize, j as usize);
        let idx = (i * self.size.width as usize) + j;
        self.chars.get(idx)
    }

    // stretch sprite "size" times with a given char
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

    // returns a iterator over the sprite rows
    pub fn rows(&self) -> Chunks<char> {
        self.chars.chunks(self.size.width as usize)
    }
}
