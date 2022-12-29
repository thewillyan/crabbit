use std::io::Write;
use termion::cursor;

pub mod stage;

use stage::Stage;

// (width, height)
type Pos = (u16, u16);
// (x, y)
type Size = (usize, usize);
// ascii matrix
type Sprite = Vec<Vec<char>>;

pub trait AsObj {
    fn as_obj(&self) -> Obj;
}

pub trait AsSprite {
    fn as_sprite(&self) -> Sprite;
}

pub struct Obj {
    pub pos: Pos,
    pub sprite: Sprite
}

impl Obj {
    pub fn width(&self) -> usize {
        self.sprite[0].len()
    }

    pub fn height(&self) -> usize {
        self.sprite.len()
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        let (x, mut y) = self.pos;
        for line in &self.sprite {
            let line: String = line.iter().collect();
            write!(out, "{}{}", cursor::Goto(x,y), line).unwrap();
            y += 1;
        }
    }

    pub fn erase<O: Write>(&self, out: &mut O) {
        let (x, mut y) = self.pos;
        for line in &self.sprite {
            let overwrite = " ".repeat(line.len());
            write!(out, "{}{}", cursor::Goto(x,y), overwrite).unwrap();
            y += 1;
        }
    }
}

pub struct Game {
    player: Player,
    obstacles: Obs,
    stage: Stage
}

// Player
pub struct Player {
    score: u32,
    obj: Obj
}

// Obstacles
pub struct Obs;
