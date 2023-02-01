mod score;
mod splash;

use crate::graphics::{object::Obj, Pos, Render, Size};
use score::Score;
use termion::color::{Color, Fg};

use super::DynComp;

/// HUD Components.
#[derive(Debug)]
pub struct Hud {
    pub size: Size,
    pub score: Score,
    pub splash: Option<Obj>,
}

impl Hud {
    /// Returns a new instance of Hud.
    pub fn new(size: Size) -> Self {
        let score = Score::new(Pos { col: 1, row: 1 });
        let splash = None;
        Self {
            size,
            score,
            splash,
        }
    }

    /// Set a splash screen with the given mensage and color.
    pub fn set_splash<C: Color>(&mut self, msg: &str, color: &Fg<C>) {
        let splash = splash::splash_screen(msg, color, &self.size);
        self.splash = Some(splash);
    }

    /// Takes the splash screen out. Returns `Some` if has splash screen or `None` otherwise.
    pub fn take_splash(&mut self) -> Option<Obj> {
        self.splash.take()
    }
}

impl Render for Hud {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        self.score.render(out);
        if let Some(obj) = &self.splash {
            obj.render(out);
        }
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        self.score.erase(out);
        if let Some(obj) = &self.splash {
            obj.erase(out);
        }
    }
}

impl DynComp for Hud {
    fn update(&mut self) {
        self.score.update();
    }

    fn reset(&mut self) {
        self.score.reset();
    }
}
