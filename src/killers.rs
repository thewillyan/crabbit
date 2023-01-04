use std::{collections::VecDeque, io::Write};

use rand::{distributions::Bernoulli, prelude::Distribution};
use termion::color::{Bg, Fg, Red, Reset};

use crate::{object::RetObj, Pos};

pub trait Hitmap {
    fn hits(&self, pos: &Pos) -> bool;
}

// each wall is, at most, 2 rows high.
pub struct Walls {
    pub pos: Pos,
    pub sprite_char: char,
    pub gap: u8,
    pub speed: u16,
    queue: VecDeque<Wall>,
    objs: VecDeque<RetObj>,
    wall_prob: Bernoulli,
}

impl Walls {
    pub fn new(pos: Pos, sprite_char: char, gap: u8, speed: u16) -> Walls {
        Walls {
            pos,
            sprite_char,
            gap,
            speed,
            queue: VecDeque::new(),
            objs: VecDeque::new(),
            // chance of having a wall: 16% per chunk
            wall_prob: Bernoulli::from_ratio(16, 100).unwrap(),
        }
    }

    // generate a chunk of walls (4 walls)
    pub fn gen_walls(&mut self) {
        for i in 0..4 {
            let has_wall = self.wall_prob.sample(&mut rand::thread_rng());
            if has_wall && (i == 0 || i == 3) {
                self.queue.push_back(Wall::Small);
            } else if has_wall {
                let is_big: bool = rand::random();
                let wall = if is_big { Wall::Big } else { Wall::Small };
                self.queue.push_back(wall);
            } else {
                self.queue.push_back(Wall::Void);
            }
        }

        for _ in 0..self.gap {
            self.queue.push_back(Wall::Void);
        }
    }

    pub fn shift_objs(&mut self) {
        self.objs.iter_mut().for_each(|obj| {
            obj.pos.col = obj.pos.col.checked_sub(self.speed).unwrap_or(0);
        });
    }

    pub fn clean_objs(&mut self) {
        if let Some(obj) = self.objs.front() {
            if obj.pos.col == 0 {
                self.objs.pop_front();
            }
        }
    }

    pub fn update(&mut self) {
        self.shift_objs();
        self.clean_objs();

        match self.queue.pop_front() {
            Some(w) => {
                if let Some(obj) = w.to_obj(self.sprite_char, self.pos.clone()) {
                    self.objs.push_back(obj);
                }
            }
            None => self.gen_walls(),
        }
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        for obj in &self.objs {
            obj.render(out);
        }
    }
}

impl Hitmap for Walls {
    fn hits(&self, pos: &Pos) -> bool {
        for obj in &self.objs {
            let col = obj.pos.col + (obj.pos.col % self.speed);
            let row_range = obj.pos.row..(obj.pos.row + obj.size.height);

            if col > pos.col {
                return false;
            }

            if col == pos.col && row_range.contains(&pos.row) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone)]
enum Wall {
    Big,
    Small,
    Void,
}

impl Wall {
    pub fn to_obj(&self, sprite_char: char, mut pos: Pos) -> Option<RetObj> {
        let h = match self {
            Self::Big => 2,
            Self::Small => 1,
            Self::Void => return None,
        };
        pos.row = pos.row.checked_sub(h).unwrap_or(1);

        let sprite = vec![vec![sprite_char]; h as usize];
        let obj = RetObj::new(pos, sprite, Bg(Reset), Fg(Red));
        Some(obj)
    }
}
