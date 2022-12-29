use std::{
    collections::HashSet,
    io::Write
};
use crate::{Size, Sprite, Obj, AsSprite};

pub struct Stage {
    pub size: Size,
    pub layers: Vec<Layer>,
    pub hitmap: HashSet<u16>,
    objs: Vec<Obj>
}

impl Stage {
    pub fn new(width: usize) -> Stage {
        Stage {
            size: (width, 0),
            layers: Vec::new(),
            objs: Vec::new(),
            hitmap: HashSet::new(),
        }
    }

    fn push(&mut self, layer: Layer) {
        let (_, height) = &mut self.size;
        let (_, layer_height) = layer.size;
        let layer_y = *height as u16 + 1;

        if layer.bound {
            for y in layer_y..(layer_y + layer_height as u16) {
                self.hitmap.insert(y);
            }
        }

        let obj = Obj {
            pos: (1, *height as u16 + 1),
            sprite: layer.as_sprite()
        };
        self.objs.push(obj);

        *height += layer_height;
        self.layers.push(layer);
    }

    pub fn add_layer(&mut self, sprite: Sprite, bound: bool, shift: usize) {
        let (width, _) = self.size;
        let layer = Layer::new(width, sprite, bound, shift);
        self.push(layer);
    }

    pub fn shift(&mut self) {
        self.layers.iter_mut().enumerate().for_each(|(i, layer)| {
            if !layer.is_static() {
                layer.shift();
                self.objs[i].sprite = layer.as_sprite();
            }
        });
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
    pub fn new(width: usize, sprite: Sprite, bound: bool, shift: usize) -> Layer {
        let offset = 0;
        let size = (width, sprite.len());
        Layer { size, sprite, bound, offset, shift }
    }

    pub fn shift(&mut self) {
        let (width, _) = self.size;
        self.offset = (self.offset + self.shift) % width;
    }

    pub fn is_static(&self) -> bool {
        self.shift == 0
    }
}

impl AsSprite for Layer {
    fn as_sprite(&self) -> Sprite {
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

        sprite
    }
}
