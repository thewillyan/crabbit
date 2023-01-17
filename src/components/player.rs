use crate::{
    components::Comp,
    graphics::{Obj, Render, Sprite},
    Pos,
};
use std::{collections::VecDeque, io::Write};
use termion::color::{Color, Fg};

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

pub struct Player {
    pub score: u32,
    pub state: PlayerState,
    pub obj: Obj,
    default_pos: (u16, u16),
    moves: VecDeque<Move>,
}

impl Player {
    pub fn new<C: Color>(sprite: Sprite, color: Fg<C>, spawn: Pos) -> Player {
        let (_, sp_height) = sprite.size();
        let pos = Pos {
            col: spawn.col,
            row: spawn.row - sp_height,
        };
        let default_pos = (pos.col, pos.row);
        Player {
            score: 0,
            state: PlayerState::Running,
            obj: Obj::new(pos.clone(), sprite, color),
            default_pos,
            moves: VecDeque::new(),
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
        if let PlayerState::Running = self.state {
            self.state = PlayerState::Jumping;
            for _ in 0..height {
                self.up(1);
            }
            self.stop();
            for _ in 0..height {
                self.down(1);
            }
        }
    }

    pub fn kill(&mut self) {
        self.state = PlayerState::Killed;
    }
}

impl Render for Player {
    fn render<O: Write>(&self, out: &mut O) {
        self.obj.render(out);
    }

    fn erase<O: Write>(&self, out: &mut O) {
        self.obj.render(out);
    }
}

impl Comp for Player {
    fn update(&mut self) {
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

    fn reset(&mut self) {
        let (col, row) = self.default_pos;
        self.obj.pos.col = col;
        self.obj.pos.row = row;
        self.moves.clear();
        self.state = PlayerState::Running;
        self.score = 0;
    }
}
