use termion::color::{Color, Fg};

use crate::graphics::{Obj, Pos, Size, Sprite};

pub fn splash_screen<C: Color>(msg: &str, color: &Fg<C>, size: &Size) -> Obj {
    let h_center = (size.width / 2) + 1;
    let v_center = size.height / 2;
    let col = h_center.checked_sub((msg.len() / 2) as u16).unwrap_or(1);
    let pos = Pos { col, row: v_center };

    let chars = msg.chars().collect();
    let sprite = Sprite::new(chars, msg.len() as u16);
    Obj::new(pos, sprite, color)
}
