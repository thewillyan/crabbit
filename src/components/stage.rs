use termion::color::{Color, Fg};

use crate::{
    components::DynComp,
    graphics::{
        object::{Obj, Sprite},
        Pos, Render, Size, TermOut,
    },
};

/// A [`Stage`] layer.
pub struct Layer {
    pub size: Size,
    sprite: Sprite,
    shift: u16,
    offset: u16,
}

impl Layer {
    /// Returns a new layer builder.
    pub fn new(width: u16, sprite: Sprite) -> LayerBuild {
        LayerBuild {
            width,
            sprite,
            shift: None,
        }
    }

    /// Returns `true` if the layer doesn't move and `false` otherwise.
    pub fn is_static(&self) -> bool {
        self.shift == 0
    }

    /// Returns a visual ascii matrix representation of the layer.
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

    /// Returns the sprite representation of the layer.
    pub fn to_sprite(&self) -> Sprite {
        let ascii_matrix = self.ascii_matrix();
        Sprite::new(ascii_matrix, self.size.width)
    }
}

impl DynComp for Layer {
    fn update(&mut self) {
        self.offset = (self.offset + self.shift) % self.sprite.size().0;
    }

    fn reset(&mut self) {
        self.offset = 0;
    }
}

/// Builds a new stage `Layer`.
pub struct LayerBuild {
    width: u16,
    sprite: Sprite,
    shift: Option<u16>,
}

impl LayerBuild {
    /// Changes the step that the layer moves.
    pub fn shift(mut self, step: u16) -> Self {
        self.shift = Some(step);
        self
    }

    /// Add gap between the sprites.
    pub fn gap(mut self, size: usize) -> Self {
        self.sprite.stretch(size, ' ');
        self
    }

    /// Builds a new `Layer`.
    pub fn build(self) -> Layer {
        let size = Size {
            width: self.width,
            height: self.sprite.size().1,
        };

        Layer {
            size,
            sprite: self.sprite,
            shift: self.shift.unwrap_or(1),
            offset: 0,
        }
    }
}

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
    pub fn new(width: u16, height: u16) -> Self {
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
        let sprite = layer.to_sprite();
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
    fn render(&self, out: &mut TermOut) {
        for obj in &self.objs {
            obj.render(out);
        }
    }

    fn erase(&self, out: &mut TermOut) {
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
