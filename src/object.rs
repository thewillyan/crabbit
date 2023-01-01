use super::{Pos, Size, Sprite};
use std::io::Write;
use termion::{
    color::{Bg, Color, Fg, Reset},
    cursor,
};

pub struct Obj {
    pub pos: Pos,
    pub sprite: Sprite,
    pub bg: String,
    pub fg: String,
}

impl Obj {
    pub fn new<C: Color, D: Color>(pos: Pos, sprite: Sprite, bg: Bg<C>, fg: Fg<D>) -> Obj {
        Obj {
            pos,
            sprite,
            bg: bg.to_string(),
            fg: fg.to_string(),
        }
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        let mut row = self.pos.row;
        for line in &self.sprite {
            let line: String = line.iter().collect();
            write!(
                out,
                "{}{}{}{}{}{}",
                cursor::Goto(self.pos.col, row),
                self.bg,
                self.fg,
                line,
                Fg(Reset),
                Bg(Reset)
            )
            .unwrap();
            row += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let mut row = self.pos.row;
        for line in &self.sprite {
            let overwrite = " ".repeat(line.len());
            write!(out, "{}{}", cursor::Goto(self.pos.col, row), overwrite).unwrap();
            row += 1;
        }
    }
}

// Retangular object
pub struct RetObj {
    pub size: Size,
    pub pos: Pos,
    pub sprite: Sprite,
    pub bg: String,
    pub fg: String,
}

impl RetObj {
    pub fn new<C: Color, D: Color>(pos: Pos, mut sprite: Sprite, bg: Bg<C>, fg: Fg<D>) -> RetObj {
        Self::to_ret(&mut sprite);
        let size = Size {
            width: sprite[0].len() as u16,
            height: sprite.len() as u16,
        };

        RetObj {
            size,
            pos,
            sprite,
            bg: bg.to_string(),
            fg: fg.to_string(),
        }
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
            write!(
                out,
                "{}{}{}{}{}{}",
                cursor::Goto(self.pos.col, row),
                self.bg,
                self.fg,
                line,
                Fg(Reset),
                Bg(Reset)
            )
            .unwrap();
            row += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let row = self.pos.row;
        let overwrite = " ".repeat(self.size.width as usize);
        for r in row..(row + self.size.height) {
            write!(out, "{}{}", cursor::Goto(self.pos.col, r), overwrite).unwrap();
        }
    }
}
