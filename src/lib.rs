use std::io::Write;
use termion::{
    color::{Bg, Color, Fg},
    cursor,
};

pub mod object;
pub mod runner;
pub mod stage;

use object::Obj;
use stage::Stage;
use termion::color;

// ascii matrix
pub type Sprite = Vec<Vec<char>>;

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
    pub height: u16,
}

pub struct Game {
    player: Player,
    _obstacles: Obs,
    stage: Stage,
}

impl Game {
    pub fn new<C: Color>(player_sprite: Sprite, player_fg: Fg<C>, mut stage: Stage) -> Game {
        stage.fill_hitmap();

        let floor = stage.hitmap.iter().min().expect("Empty stage!");
        let player_height = player_sprite.len() as u16;
        let pos = Pos {
            col: 5,
            row: floor - player_height,
        };
        let player = Player::new(player_sprite, player_fg, pos);

        Game {
            player,
            _obstacles: Obs,
            stage,
        }
    }

    pub fn update(&mut self) {
        self.player.score += 1;
        self.stage.shift();
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        let score = format!("Score: {:0>10}", self.player.score);
        let score_width = score.len() as u16;
        let corner = if self.stage.size.width > score_width {
            self.stage.size.width - score_width
        } else {
            1
        };

        self.stage.render(out);
        self.player.render(out);
        write!(out, "{}{}", cursor::Goto(corner, 1), score).expect("Error while rendering score!");
    }
}

// Player
pub struct Player {
    pub score: u32,
    pub state: PlayerState,
    obj: Obj,
}

impl Player {
    pub fn new<C: Color>(sprite: Sprite, fg: Fg<C>, pos: Pos) -> Player {
        let score = 0;
        let state = PlayerState::Running;
        let obj = Obj::new(pos, sprite, Bg(color::Reset), fg);
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

    pub fn render<O: Write>(&self, out: &mut O) {
        self.obj.render(out);
    }
}

pub enum PlayerState {
    Jumping,
    Running,
    Killed,
}

// Obstacles
pub struct Obs;
