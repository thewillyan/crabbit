use crate::{graphics::{Sprite, Size}, components::DynComp};

/// A `Stage` layer.
pub struct Layer {
    pub size: Size,
    sprite: Sprite,
    shift: u16,
    offset: u16,
}

impl Layer {
    /// Returns a new layer builder.
    pub fn new(width: u16, sprite: Sprite) -> LayerBuild {
        LayerBuild { width, sprite, shift: None }
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
    pub fn as_sprite(&self) -> Sprite {
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
