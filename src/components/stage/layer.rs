use crate::graphics::{Sprite, Size};

pub struct Layer {
    pub size: Size,
    pub sprite: Sprite,
    pub shift: u16,
    pub offset: u16,
}

impl Layer {
    pub fn new(width: u16, mut sprite: Sprite, gap: usize, shift: u16) -> Layer {
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
