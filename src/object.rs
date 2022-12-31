use std::io::Write;
use super::{ Pos, Size, Sprite };
use termion::cursor;

pub struct Obj {
    pub pos: Pos,
    pub sprite: Sprite
}

impl Obj {
    pub fn render<O: Write>(&self, out: &mut O) {
        let mut row = self.pos.row;
        for line in &self.sprite {
            let line: String = line.iter().collect();
            write!(out, "{}{}", cursor::Goto(self.pos.col, row), line)
                .unwrap();
            row += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let mut row = self.pos.row;
        for line in &self.sprite {
            let overwrite = " ".repeat(line.len());
            write!(out, "{}{}", cursor::Goto(self.pos.col, row), overwrite)
                .unwrap();
            row += 1;
        }
    }
}

// Retangular object
pub struct RetObj {
    pub size: Size,
    pub pos: Pos,
    pub sprite: Sprite,
}

impl RetObj {
    pub fn new(col: u16, row: u16, mut sprite: Sprite) -> RetObj {
        let pos = Pos { col, row };
        Self::to_ret(&mut sprite);
        let size = Size {
            width: sprite[0].len() as u16,
            height: sprite.len() as u16,
        };

        RetObj { size, pos, sprite }
    }

    pub fn to_ret(sprite: &mut Sprite) {
        let width = sprite
            .iter()
            .map(|line| line.len())
            .max()
            .expect("Empty sprite!");
        sprite.iter_mut().for_each(|line| {
            // add padding right
            while line.len() < width {
                line.push(' ');
            }
        });
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        let mut row = self.pos.row;
        for line in &self.sprite {
            let line: String = line.iter().collect();
            write!(out, "{}{}", cursor::Goto(self.pos.col, row), line)
                .unwrap();
            row += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let row = self.pos.row;
        let overwrite = " ".repeat(self.size.width as usize);
        for r in row..(row + self.size.height) {
            write!(out, "{}{}", cursor::Goto(self.pos.col, r), overwrite)
                .unwrap();
        }
    }
}


