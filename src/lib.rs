use std::io::Write;

pub mod stage;
pub mod object;
pub mod runner;

use stage::Stage;
use object::Obj;

// ascii matrix
type Sprite = Vec<Vec<char>>;

pub trait AsSprite {
    fn as_sprite(&self) -> Sprite;
}

// not used for now
pub trait Render {
    fn render<O: Write>(&self, out: &mut O);
    fn erase<O: Write>(&self, out: &mut O);
}

#[derive(Debug)]
pub struct Pos {
    pub col: u16,
    pub row: u16,
}

pub struct Size {
    pub width: u16,
    pub height: u16
}

pub struct Game {
    player: Player,
    _obstacles: Obs,
    stage: Stage
}

impl Game {
    pub fn new(player_sprite: Sprite, stage: Stage) -> Game {
        let floor = stage.floor().expect("Empty stage!");
        let player_height = player_sprite.len() as u16;
        let player = Player::new(player_sprite, 5, floor - player_height);
        Game { player, _obstacles: Obs, stage }
    }
}

// Player
pub struct Player {
    pub score: u32,
    pub state: PlayerState,
    obj: Obj
}

impl Player {
    pub fn new(sprite: Sprite, col: u16, row: u16) -> Player {
        let score = 0;
        let state = PlayerState::Running;
        let pos = Pos { col, row };
        let obj = Obj { pos, sprite };
        Player { score, state, obj }
    }

    pub fn up(&mut self) {
        if self.obj.pos.row > 1 {
            self.obj.pos.row -= 1
        }
    }

    pub fn down(&mut self) {
        self.obj.pos.row += 1
    }
}

pub enum PlayerState {
    Jumping,
    Running,
    Killed
}

// Obstacles
pub struct Obs;
