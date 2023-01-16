use std::{collections::VecDeque, io::Write};

use termion::color::{Color, Fg};

use crate::{
    graphics::{object::Obj, sprite::Sprite},
    Pos, Size,
};

pub struct Stage {
    pub size: Size,
    pub layers: Vec<Layer>,
    objs: Vec<Obj>,
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

    fn push<C: Color>(&mut self, layer: Layer, color: Fg<C>) {
        let sprite = layer.as_sprite();
        let pos = Pos {
            col: 1,
            row: self.size.height + 1,
        };

        let obj = Obj {
            pos,
            sprite,
            color: color.to_string(),
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
        shift: u16,
    ) {
        let layer = Layer::new(self.size.width, sprite, gap, barrier, shift);
        self.push(layer, fg);
    }

    pub fn shift(&mut self) {
        self.layers.iter_mut().enumerate().for_each(|(i, layer)| {
            if !layer.is_static() {
                layer.shift();
                self.objs[i].sprite.update(layer.ascii_matrix());
            }
        });
    }

    pub fn fill_hitmap(&mut self) {
        self.hitmap.clear();
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.barrier {
                let height = self.objs[i].sprite.size().1;
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

    pub fn reset(&mut self) {
        self.layers.iter_mut().for_each(|layer| layer.reset());
    }
}

pub struct Layer {
    pub size: Size,
    pub sprite: Sprite,
    pub barrier: bool,
    pub shift: u16,
    pub offset: u16,
}

impl Layer {
    pub fn new(width: u16, mut sprite: Sprite, gap: usize, barrier: bool, shift: u16) -> Layer {
        // format sprite
        sprite.stretch(gap, ' ');

        let offset = 0;
        let size = Size {
            width,
            height: sprite.size().1,
        };
        Layer {
            size,
            sprite,
            barrier,
            offset,
            shift,
        }
    }

    pub fn is_static(&self) -> bool {
        self.shift == 0
    }

    pub fn shift(&mut self) {
        self.offset = (self.offset + self.shift) % self.sprite.size().0;
    }

    pub fn ascii_matrix(&self) -> Vec<char> {
        let width = self.size.width;
        let (sp_width, sp_height) = self.sprite.size();
        let mut ascii_matrix = Vec::with_capacity((width * sp_height) as usize);

        for i in 0..sp_height {
            for j in (self.offset)..(self.offset + width) {
                let j = j % sp_width;
                let c = self.sprite.get(i, j).expect("Sprite element not found!");
                ascii_matrix.push(*c);
            }
        }

        ascii_matrix
    }

    pub fn as_sprite(&self) -> Sprite {
        let ascii_matrix = self.ascii_matrix();
        Sprite::new(ascii_matrix, self.size.width)
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }
}
