use killers::Walls;
use std::io::Write;
use termion::{
    color::{Color, Fg},
    cursor,
};

pub mod killers;
pub mod object;
pub mod player;
pub mod runner;
pub mod sprite;
pub mod stage;

use player::Player;
use sprite::Sprite;
use stage::Stage;

#[derive(Clone)]
pub struct Pos {
    pub col: u16,
    pub row: u16,
}

#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Game {
    player: Player,
    stage: Stage,
    walls: Walls,
}

impl Game {
    pub fn new<C: Color>(player_sprite: Sprite, player_fg: Fg<C>, mut stage: Stage) -> Game {
        stage.fill_hitmap();

        let floor = stage.hitmap.iter().min().expect("Empty stage!");
        let player_height = player_sprite.len() as u16;
        let pos = Pos {
            col: 8,
            row: floor - player_height,
        };
        let player = Player::new(player_sprite, player_fg, pos);
        let walls_pos = Pos {
            col: stage.size.width,
            row: *floor,
        };
        let walls = Walls::new(walls_pos, '|', 4, 2);

        Game {
            player,
            walls,
            stage,
        }
    }

    pub fn update(&mut self) {
        self.stage.shift();
        self.walls.update();
        self.player.score += 1;
        self.player.mv();
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
        self.walls.render(out);
        self.player.render(out);
        write!(out, "{}{}", cursor::Goto(corner, 1), score).expect("Error while rendering score!");
    }
}
