mod score;

use crate::graphics::{Pos, Render, Size};
use score::Score;

use super::DynComp;

pub struct Hud {
    pub size: Size,
    pub score: Score,
}

impl Hud {
    pub fn new(size: Size) -> Hud {
        let corner_col = size.width.checked_sub(Score::sprite_width()).unwrap_or(1);
        let score = Score::new(Pos {
            col: corner_col,
            row: 1,
        });
        Hud { size, score }
    }
}

impl Render for Hud {
    fn render<O: std::io::Write>(&self, out: &mut O) {
        self.score.render(out);
    }

    fn erase<O: std::io::Write>(&self, out: &mut O) {
        self.score.erase(out);
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
