use std::{collections::VecDeque, io::Write};
use termion::color::{Bg, Color, Fg, Reset};

use crate::{object::Obj, sprite::Sprite, Pos};

pub struct Player {
    pub score: u32,
    pub state: PlayerState,
    pub moves: VecDeque<Move>,
    obj: Obj,
}

impl Player {
    pub fn new<C: Color>(sprite: Sprite, fg: Fg<C>, pos: Pos) -> Player {
        Player {
            score: 0,
            state: PlayerState::Running,
            moves: VecDeque::new(),
            obj: Obj::new(pos, sprite, Bg(Reset), fg),
        }
    }

    fn up(&mut self, amount: u16) {
        if self.obj.pos.row > 1 {
            self.moves.push_back(Move::Up(amount));
        }
    }

    fn down(&mut self, amount: u16) {
        self.moves.push_back(Move::Down(amount));
    }

    fn stop(&mut self) {
        self.moves.push_back(Move::Stop);
    }

    pub fn jump(&mut self, height: u16) {
        self.state = PlayerState::Jumping;
        for _ in 0..height {
            self.up(1);
        }
        self.stop();
        for _ in 0..height {
            self.down(1);
        }
    }

    pub fn mv(&mut self) {
        if let Some(mv) = self.moves.pop_front() {
            match mv {
                Move::Up(amount) if self.obj.pos.row > amount => self.obj.pos.row -= amount,
                Move::Down(amount) => self.obj.pos.row += amount,
                _ => (),
            }
        }

        if self.moves.is_empty() {
            self.state = PlayerState::Running;
        }
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        self.obj.render(out);
    }
}

pub enum Move {
    Up(u16),
    Down(u16),
    Stop,
}

pub enum PlayerState {
    Jumping,
    Running,
    Killed,
}
