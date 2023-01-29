use std::{collections::VecDeque, io::Write};
use termion::color::{Color, Fg};

use crate::{
    components::DynComp,
    graphics::{Obj, Pos, Render, Sprite},
};

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

/// A player of `Game`.
pub struct Player {
    pub state: PlayerState,
    pub obj: Obj,
    default_pos: Pos,
    moves: VecDeque<Move>,
}

impl Player {
    /// Creates a new instance of `Player`.
    pub fn new<C: Color>(icon: char, color: C, floor: u16) -> Player {
        let sprite = Sprite::new(vec![icon], 1);
        let pos = Pos { col: 8, row: floor - 1 };
        Player {
            state: PlayerState::Running,
            obj: Obj::new(pos.clone(), sprite, &Fg(color)),
            default_pos: pos,
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

    /// Add jump moves to the movement queue.
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

    /// Kills player (change state).
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

impl DynComp for Player {
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
        self.obj.pos = self.default_pos.clone();
        self.moves.clear();
        self.state = PlayerState::Running;
    }
}
