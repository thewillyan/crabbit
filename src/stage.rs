use std::{collections::VecDeque, io::Write};

use termion::color::{self, Color, Fg};

use crate::{
    object::RetObj,
    sprite::{self, Sprite},
    Pos, Size,
};

pub struct Stage {
    pub size: Size,
    pub layers: Vec<Layer>,
    objs: Vec<RetObj>,
    hitmap: VecDeque<u16>,
}

impl Stage {
    pub fn new(width: u16) -> Stage {
        Stage {
            size: Size { width, height: 0 },
            layers: Vec::new(),
            objs: Vec::new(),
            hitmap: VecDeque::new(),
        }
    }

    pub fn floor(&self) -> Option<&u16> {
        self.hitmap.front()
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

    pub fn add_layer<C: Color>(
        &mut self,
        sprite: Sprite,
        fg: Fg<C>,
        gap: usize,
        barrier: bool,
        shift: usize,
    ) {
        let layer = Layer::new(self.size.width, sprite, gap, barrier, shift);
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
            if layer.barrier {
                let height = self.objs[i].size.height;
                let row = self.objs[i].pos.row;
                for n in row..(row + height as u16) {
                    self.hitmap.push_back(n);
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
    pub sprite_width: usize,
    pub barrier: bool,
    pub shift: usize,
    pub offset: usize,
}

impl Layer {
    pub fn new(width: u16, mut sprite: Sprite, gap: usize, barrier: bool, shift: usize) -> Layer {
        // format sprite
        sprite::to_ret(&mut sprite);
        sprite::stretch(&mut sprite, gap, ' ');

        let sprite_width = sprite[0].len();
        let offset = 0;
        let size = Size {
            width,
            height: sprite.len() as u16,
        };
        Layer {
            size,
            sprite,
            sprite_width,
            barrier,
            offset,
            shift,
        }
    }

    pub fn is_static(&self) -> bool {
        self.shift == 0
    }

    pub fn shift(&mut self) {
        self.offset = (self.offset + self.shift) % self.sprite_width;
    }

    fn as_sprite(&self) -> Sprite {
        let width = self.size.width as usize;
        let height = self.size.height as usize;

        let mut ascii_matrix = vec![Vec::with_capacity(width); height];

        for j in (self.offset)..(self.offset + width) {
            let j = j % self.sprite_width;
            ascii_matrix
                .iter_mut()
                .enumerate()
                .for_each(|(i, x)| x.push(self.sprite[i][j]))
        }

        ascii_matrix
    }
}
