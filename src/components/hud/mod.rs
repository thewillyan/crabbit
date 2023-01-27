mod score;
mod splash;

use crate::graphics::{Obj, Pos, Render, Size};
use score::Score;
use termion::color::{Color, Fg};

use super::DynComp;

pub struct Hud {
    pub size: Size,
    pub score: Score,
    pub splash: Option<Obj>,
}

impl Hud {
    pub fn new(size: Size) -> Self {
        let score = Score::new(Pos { col: 1, row: 1 });
        let splash = None;
        Self {
            size,
            score,
            splash,
        }
    }

    pub fn set_splash<C: Color>(&mut self, msg: &str, color: &Fg<C>) {
        let splash = splash::splash_screen(msg, color, &self.size);
        self.splash = Some(splash);
    }

    pub fn take_splash(&mut self) -> Option<Obj> {
        self.splash.take()
    }
}

impl Render for Hud {
    fn render<O: std::io::Write>(&self, out: &mut O) {
        self.score.render(out);
        if let Some(obj) = &self.splash {
            obj.render(out);
        }
    }

    fn erase<O: std::io::Write>(&self, out: &mut O) {
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
