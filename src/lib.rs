use std::io::Write;
use termion::color::Color;

pub mod components;
pub mod graphics;
pub mod runner;

use components::{enemies::Walls, player::Player, Stage, DynComp, Hud};
use graphics::{Sprite, Pos, Size, Render};

pub struct Game {
    player: Player,
    stage: Stage,
    walls: Walls,
    hud: Hud
}

impl Game {
    pub fn new<C: Color>(player_sprite: Sprite, player_fg: C, mut stage: Stage) -> Game {
        stage.fill_hitmap();

        let floor = *stage.floor().expect("Empty stage!");
        let player_spawn = Pos { col: 8, row: floor };
        let player = Player::new(player_sprite, player_fg, player_spawn);
        let walls_spawn = Pos {
            col: stage.size.width,
            row: floor,
        };
        let walls = Walls::new(walls_spawn, '|', 4, 2);
        let hud = Hud::new(stage.size.clone());

        Game {
            player,
            walls,
            stage,
            hud
        }
    }

    pub fn update(&mut self) {
        self.stage.update();
        self.walls.update();
        self.player.update();
        self.hud.update();
    }

    pub fn render<O: Write>(&self, out: &mut O) {
        self.stage.render(out);
        self.walls.render(out);
        self.player.render(out);
        self.hud.render(out);
    }

    pub fn reset(&mut self) {
        self.stage.reset();
        self.walls.reset();
        self.player.reset();
        self.hud.reset();
    }
}
