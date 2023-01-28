pub mod layer;
pub use layer::Layer;

use termion::color::{Color, Fg};

use crate::{
    components::DynComp,
    graphics::{object::Obj, Pos, Render, Size},
};

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
    pub fn push_layer<C: Color>(&mut self, layer: Layer, color: C) {
        let sprite = layer.as_sprite();
        let row = self
            .size
            .height
            .checked_sub(layer.size.height + self.layers_height - 1)
            .unwrap_or(1);

        let pos = Pos { col: 1, row };
        let obj = Obj::new(pos, sprite, &Fg(color));

        self.layers_height += layer.size.height;
        self.objs.push(obj);
        self.layers.push(layer);
    }

    /// Sets the `Layer` on top of the stack as the floor of the stage.
    pub fn set_floor(&mut self) {
        self.floor = match self.objs.last() {
            Some(obj) => obj.pos.row,
            None => 1,
        }
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
                layer.update();
                self.objs[i].sprite.set_ascii(layer.ascii_matrix());
            }
        });
    }

    fn reset(&mut self) {
        self.layers.iter_mut().for_each(|layer| layer.reset());
    }
}
