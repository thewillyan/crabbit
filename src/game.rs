use termion::color::Color;

use crate::{
    components::{enemies::Walls, player::Player, DynComp, Hud, Stage},
    graphics::{Pos, Render, Sprite},
};

pub struct Game {
    pub player: Player,
    pub stage: Stage,
    pub walls: Walls,
    pub hud: Hud,
}

impl Game {
    pub fn new<C: Color>(player_sprite: Sprite, player_fg: C, stage: Stage) -> Game {
        let player_spawn = Pos {
            col: 8,
            row: stage.floor,
        };
        let player = Player::new(player_sprite, player_fg, player_spawn);
        let walls_spawn = Pos {
            col: stage.size.width,
            row: stage.floor,
        };
        let walls = Walls::new(walls_spawn, '|', 4, 2);
        let hud = Hud::new(stage.size.clone());

        Game {
            player,
            walls,
            stage,
            hud,
        }
    }
}

impl DynComp for Game {
    fn update(&mut self) {
        self.stage.update();
        self.walls.update();
        self.player.update();
        self.hud.update();
    }

    fn reset(&mut self) {
        self.stage.reset();
        self.walls.reset();
        self.player.reset();
        self.hud.reset();
    }
}

impl Render for Game {
    fn render<O: std::io::Write>(&self, out: &mut O) {
        self.stage.render(out);
        self.walls.render(out);
        self.player.render(out);
        self.hud.render(out);
    }

    fn erase<O: std::io::Write>(&self, out: &mut O) {
        self.stage.erase(out);
        self.walls.erase(out);
        self.player.erase(out);
    }
}
