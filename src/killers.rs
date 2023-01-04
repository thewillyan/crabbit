use std::{
    collections::{HashSet, LinkedList},
    io::Write, borrow::BorrowMut,
};

use rand::{distributions::Bernoulli, prelude::Distribution};
use termion::color::{Bg, Fg, Red, Reset};

use crate::{object::RetObj, Pos};

// each wall is, at most, 2 rows high.
pub struct Walls {
    pub pos: Pos,
    pub sprite_char: char,
    pub gap: u8,
    pub speed: u16,
    pub hitmap: HashSet<(u16, u16)>,
    queue: LinkedList<Wall>,
    objs: LinkedList<RetObj>,
    wall_prob: Bernoulli,
}

impl Walls {
    pub fn new(pos: Pos, sprite_char: char, gap: u8, speed: u16) -> Walls {
        Walls {
            pos,
            sprite_char,
            gap,
            speed,
            queue: LinkedList::new(),
            objs: LinkedList::new(),
            hitmap: HashSet::new(),
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
            obj.pos.col = match obj.pos.col.checked_sub(self.speed) {
                Some(n) => n,
                None => 0,
            };
        });
    }

    pub fn clean_objs(&mut self) {
        if let Some(obj) = self.objs.front() {
            if obj.pos.col == 0 {
                self.objs.pop_front();
            }
        }
    }

    pub fn fill_hitmap(&mut self) {
        self.hitmap.clear();
        for obj in &self.objs {
            self.hitmap.insert((obj.pos.col, obj.pos.row));
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
        self.fill_hitmap();
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        for obj in &self.objs {
            obj.render(out);
        }
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
        pos.row -= h;

        let sprite = vec![vec![sprite_char]; h as usize];
        let obj = RetObj::new(pos, sprite, Bg(Reset), Fg(Red));
        Some(obj)
    }
}
