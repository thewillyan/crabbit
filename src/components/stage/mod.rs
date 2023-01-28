use crate::{
    components::DynComp,
    graphics::{object::Obj, sprite::Sprite, Pos, Render, Size},
};
use termion::color::{Color, Fg};

mod layer;

use layer::Layer;

/// A game stage (scenario).
pub struct Stage {
    pub size: Size,
    pub floor: u16,
    layers: Vec<Layer>,
    objs: Vec<Obj>,
    layers_height: u16,
}

impl Stage {
    /// Returns a new Stage instance.
    pub fn new(width: u16, height: u16) -> Stage {
        Stage {
            size: Size { width, height },
            floor: 1,
            layers: Vec::new(),
            objs: Vec::new(),
            layers_height: 0,
        }
    }

    /// Push a new layer to the layer stack.
    fn push<C: Color>(&mut self, layer: Layer, color: C, is_floor: bool) {
        let sprite = layer.as_sprite();
        let row = self.size.height
            .checked_sub(layer.size.height + self.layers_height - 1)
            .unwrap_or(1);

        if is_floor {
            self.floor = row;
        }

        let pos = Pos { col: 1, row };
        let obj = Obj::new(pos, sprite, &Fg(color));

        self.layers_height += layer.size.height;
        self.objs.push(obj);
        self.layers.push(layer);
    }

    /// Add a new layer to the stage.
    pub fn add_layer<C: Color>(
        &mut self,
        sprite: Sprite,
        color: C,
        gap: usize,
        shift: u16,
        is_floor: bool,
    ) {
        let layer = Layer::new(self.size.width, sprite, gap, shift);
        self.push(layer, color, is_floor);
    }
}

impl Render for Stage {
    fn render<O: std::io::Write>(&self, out: &mut O) {
        for obj in &self.objs {
            obj.render(out);
        }
    }

    fn erase<O: std::io::Write>(&self, out: &mut O) {
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
