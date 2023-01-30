use crate::{
    components::{enemies::Enemies, Player, DynComp, Hud, Stage},
    graphics::{Render, TermOut},
};

/// A jumper game.
pub struct Game {
    pub player: Player,
    pub stage: Stage,
    pub enemies: Enemies,
    pub hud: Hud,
}

impl Game {
    /// Returns a new instance of `Game`.
    pub fn new(player: Player, stage: Stage, enemies: Enemies) -> Game {
        let hud = Hud::new(stage.size.clone());

        Game {
            player,
            enemies,
            stage,
            hud,
        }
    }
}

impl DynComp for Game {
    fn update(&mut self) {
        self.stage.update();
        self.enemies.update();
        self.player.update();
        self.hud.update();
    }

    fn reset(&mut self) {
        self.stage.reset();
        self.enemies.reset();
        self.player.reset();
        self.hud.reset();
    }
}

impl Render for Game {
    fn render(&self, out: &mut TermOut) {
        self.stage.render(out);
        self.enemies.render(out);
        self.player.render(out);
        self.hud.render(out);
    }

    fn erase(&self, out: &mut TermOut) {
        self.stage.erase(out);
        self.enemies.erase(out);
        self.player.erase(out);
    }
}
