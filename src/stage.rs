use std::collections::HashSet;
use crate::{Size, Pos, Sprite, Obj, AsObj};

pub struct Stage {
    pub size: Size,
    pub layers: Vec<Layer>,
    pub hitmap: HashSet<u16>
}

impl Stage {
    pub fn new(width: usize, height: usize) -> Stage {
        Stage {
            size: (width, height),
            layers: Vec::with_capacity(height),
            hitmap: HashSet::new(),
        }
    }

    pub fn add_layer(&mut self, mut layer: Layer) {
        let layer_y = match self.layers.last() {
            Some(l) => l.pos.1 + l.size.1 as u16,
            None => 1
        };

        if layer.bound {
            let (_, height) = layer.size;
            for y in layer_y..(layer_y + height as u16) {
                self.hitmap.insert(y);
            }
        }

        layer.pos = (1, layer_y);
        self.layers.push(layer);
    }

    pub fn shift(&mut self) {
        self.layers.iter_mut().for_each(|layer| layer.shift());
    }
}

pub struct Layer {
    pub pos: Pos,
    pub size: Size,
    pub sprite: Sprite,
    pub bound: bool,
    pub shift: usize,
    pub offset: usize,
}

impl Layer {
    pub fn new(width: usize, sprite: Sprite, bound: bool, shift: usize) -> Layer {
        let offset = 0;
        let pos = (1,1);
        let size = (width, sprite.len());
        Layer { pos, size, sprite, bound, offset, shift }
    }

    pub fn shift(&mut self) {
        let (width, _) = self.size;
        self.offset = (self.offset + self.shift) % width;
    }

    pub fn is_static(&self) -> bool {
        self.shift == 0
    }
}

impl AsObj for Layer {
    fn as_obj(&self) -> Obj {
        let (width, height) = self.size;
        let s_width = self.sprite[0].len();
        let mut sprite = vec![Vec::with_capacity(width); height];

        for j in (self.offset)..(self.offset + width) {
            let j = j % s_width;
            sprite
                .iter_mut()
                .enumerate()
                .for_each(|(i, x)| x.push(self.sprite[i][j]))
        }
        Obj { pos: self.pos, sprite }
    }
}
