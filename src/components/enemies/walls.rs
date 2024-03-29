use rand::{distributions::Bernoulli, prelude::Distribution};
use std::collections::VecDeque;
use termion::color::{Fg, Red};

use crate::{
    components::{enemies::Enemy, DynComp},
    graphics::{
        object::{Obj, Sprite},
        Pos, Render,
    },
};

/// Kinds of wall.
enum Wall {
    Big,
    Small,
    Void,
}

impl Wall {
    pub fn to_obj(&self, sprite_char: char, mut pos: Pos) -> Option<Obj> {
        let h = match self {
            Self::Big => 2,
            Self::Small => 1,
            Self::Void => return None,
        };
        pos.row = pos.row.checked_sub(h).unwrap_or(1);

        let ascii_matrix = vec![sprite_char; h as usize];
        let sprite = Sprite::new(ascii_matrix, 1);
        let obj = Obj::new(pos, sprite, &Fg(Red));
        Some(obj)
    }
}

/// Obstacle walls.
pub struct Walls {
    pos: Pos,
    icon: char,
    shift: u16,
    queue: VecDeque<Wall>,
    objs: VecDeque<Obj>,
    wall_prob: Bernoulli,
}

impl Walls {
    pub fn new(icon: char, pos: Pos, shift: u16) -> Self {
        Walls {
            pos,
            icon,
            shift,
            queue: VecDeque::with_capacity(8),
            objs: VecDeque::new(),
            // chance of having a wall: 16% per chunk
            wall_prob: Bernoulli::from_ratio(16, 100).expect("Failed to create Bernoulli."),
        }
    }

    /// Generate a chunk of walls (4 walls) plus a gap (4 spaces).
    fn gen_walls(&mut self) {
        // walls
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

        // gap
        for _ in 0..4 {
            self.queue.push_back(Wall::Void);
        }
    }

    /// Moves each wall object.
    fn shift_objs(&mut self) {
        self.objs.iter_mut().for_each(|obj| {
            obj.pos.col = obj.pos.col.saturating_sub(self.shift);
        });
    }

    /// Remove objects that are not on the screen.
    fn clean_objs(&mut self) {
        if let Some(obj) = self.objs.front() {
            if obj.pos.col == 0 {
                self.objs.pop_front();
            }
        }
    }
}

impl Render for Walls {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        for obj in &self.objs {
            obj.render(out);
        }
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        for obj in &self.objs {
            obj.erase(out);
        }
    }
}

impl DynComp for Walls {
    fn update(&mut self) {
        self.shift_objs();
        self.clean_objs();

        match self.queue.pop_front() {
            Some(w) => {
                if let Some(obj) = w.to_obj(self.icon, self.pos.clone()) {
                    self.objs.push_back(obj);
                }
            }
            None => self.gen_walls(),
        }
    }

    fn reset(&mut self) {
        self.objs.clear();
    }
}

impl Enemy for Walls {
    fn hits(&self, pos: &Pos) -> bool {
        for obj in &self.objs {
            let col = obj.pos.col + (obj.pos.col % self.shift);
            let (_, obj_height) = obj.sprite.size();
            let row_range = obj.pos.row..(obj.pos.row + obj_height);

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
