use crate::{
    components::DynComp,
    graphics::{object::Obj, sprite::Sprite, Render},
    Pos, Size,
};
use std::{collections::VecDeque, io::Write};
use termion::color::{Color, Fg};

mod layer;

use layer::Layer;

pub struct Stage {
    pub size: Size,
    layers: Vec<Layer>,
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
}

impl Render for Stage {
    fn render<O: Write>(&self, out: &mut O) {
        for obj in &self.objs {
            obj.render(out);
        }
    }

    fn erase<O: Write>(&self, out: &mut O) {
        for obj in &self.objs {
            obj.erase(out);
        }
    }
}

impl DynComp for Stage {
    fn update(&mut self) {
        self.layers.iter_mut().enumerate().for_each(|(i, layer)| {
            if !layer.is_static() {
                layer.shift();
                self.objs[i].sprite.set_ascii(layer.ascii_matrix());
            }
        });
    }

    fn reset(&mut self) {
        self.layers.iter_mut().for_each(|layer| layer.reset());
    }
}
