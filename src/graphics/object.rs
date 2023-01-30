use std::io::Write;
use termion::{
    color::{Bg, Color, Fg, Reset},
    cursor,
};

use crate::graphics::{Pos, Render, Sprite, TermOut};

pub struct Obj {
    pub pos: Pos,
    pub sprite: Sprite,
    pub color: String,
}

impl Obj {
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
