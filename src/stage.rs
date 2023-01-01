use std::{collections::HashSet, io::Write};

use termion::color::{self, Color, Fg};

use crate::{object::RetObj, Pos, Size, Sprite};

pub struct Stage {
    pub size: Size,
    pub layers: Vec<Layer>,
    pub hitmap: HashSet<u16>,
    objs: Vec<RetObj>,
}

impl Stage {
    pub fn new(width: u16) -> Stage {
        Stage {
            size: Size { width, height: 0 },
            layers: Vec::new(),
            objs: Vec::new(),
            hitmap: HashSet::new(),
        }
    }

    pub fn floor(&self) -> Option<u16> {
        match self.objs.last() {
            Some(obj) => Some(obj.pos.row),
            None => None,
        }
    }

    fn push<C: Color>(&mut self, layer: Layer, fg: Fg<C>) {
        let sprite = layer.as_sprite();
        let pos = Pos {
            col: 1,
            row: self.size.height + 1,
        };
        let size = Size {
            width: sprite[0].len() as u16,
            height: sprite.len() as u16,
        };

        let obj = RetObj {
            size,
            pos,
            sprite,
            bg: color::Reset.bg_str().to_string(),
            fg: fg.to_string(),
        };

        self.objs.push(obj);

        self.size.height += layer.size.height;
        self.layers.push(layer);
    }

    pub fn add_layer<C: Color>(&mut self, sprite: Sprite, fg: Fg<C>, bound: bool, shift: usize) {
        let layer = Layer::new(self.size.width, sprite, bound, shift);
        self.push(layer, fg);
    }

    pub fn shift(&mut self) {
        self.layers.iter_mut().enumerate().for_each(|(i, layer)| {
            if !layer.is_static() {
                layer.shift();
                self.objs[i].sprite = layer.as_sprite();
            }
        });
    }

    pub fn fill_hitmap(&mut self) {
        self.hitmap.clear();
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.bound {
                let height = self.objs[i].size.height;
                let row = self.objs[i].pos.row;
                for n in row..(row + height as u16) {
                    self.hitmap.insert(n);
                }
            }
        }
    }

    pub fn add_padding(&mut self, padding: u16) {
        self.objs.iter_mut().for_each(|obj| obj.pos.row += padding);
        self.fill_hitmap();
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        for obj in &self.objs {
            obj.render(out);
        }
    }
}

pub struct Layer {
    pub size: Size,
    pub sprite: Sprite,
    pub bound: bool,
    pub shift: usize,
    pub offset: usize,
}

impl Layer {
    pub fn new(width: u16, mut sprite: Sprite, bound: bool, shift: usize) -> Layer {
        let offset = 0;
        let size = Size {
            width,
            height: sprite.len() as u16,
        };
        RetObj::to_ret(&mut sprite);
        Layer {
            size,
            sprite,
            bound,
            offset,
            shift,
        }
    }

    pub fn shift(&mut self) {
        self.offset = (self.offset + self.shift) % self.size.width as usize;
    }

    pub fn is_static(&self) -> bool {
        self.shift == 0
    }

    fn as_sprite(&self) -> Sprite {
        let width = self.size.width as usize;
        let height = self.size.height as usize;
        let s_width = self.sprite[0].len();
        let mut sprite = vec![Vec::with_capacity(width); height];

        for j in (self.offset)..(self.offset + width) {
            let j = j % s_width;
            sprite
                .iter_mut()
                .enumerate()
                .for_each(|(i, x)| x.push(self.sprite[i][j]))
        }

        sprite
    }
}
