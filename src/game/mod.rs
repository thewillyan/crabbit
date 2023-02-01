//! Defines how all the [`components`] are interrelated and their behavior on runtime.
//!
//! [`components`]: crate::components

use crate::{
    components::{enemies::Enemies, DynComp, Hud, Player, Stage},
    graphics::{Render, TermOut},
};

mod runner;
use runner::Runner;

/// A jumper game. Aggregates all the `components` in a sigle structure.
pub struct Game {
    player: Player,
    stage: Stage,
    enemies: Enemies,
    hud: Hud,
}

impl Game {
    /// Returns a new instance of `Game`.
    pub fn new(player: Player, stage: Stage, enemies: Enemies) -> Self {
        let hud = Hud::new(stage.size.clone());

        Game {
            player,
            enemies,
            stage,
            hud,
        }
    }

    /// Runs the game on the terminal `out`. Displays the given `start_msg` on the start with the
    /// given color `C`.
    pub fn run<C: termion::color::Color>(self, start_msg: &str, msg_color: C, out: &mut TermOut) {
        Runner::new(self, start_msg, msg_color).run(out);
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
