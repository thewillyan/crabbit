use termion::color;

use crate::{
    components::DynComp,
    graphics::{Obj, Pos, Render, Sprite},
};

// current score label
const CURR_LABEL: &'static str = "Score";
// record score label
const REC_LABEL: &'static str = "Record";

/// Tracks the user score.
pub struct Score {
    pub current: u32,
    pub record: u32,
    curr_obj: Obj,
    rec_obj: Obj,
}

impl Score {
    /// Returns a new instace of Score.
    pub fn new(mut pos: Pos) -> Score {
        let color = color::Fg(color::Reset);
        let str_score = "0".repeat(10);

        let chars: Vec<char> = format!("{}: {}", REC_LABEL, str_score).chars().collect();
        let width = chars.len() as u16;
        let sprite = Sprite::new(chars, width);
        let rec_obj = Obj::new(pos.clone(), sprite, &color);

        pos.row += 1;
        let chars: Vec<char> = format!("Score: {}", str_score).chars().collect();
        let width = chars.len() as u16;
        let sprite = Sprite::new(chars, width);
        let curr_obj = Obj::new(pos, sprite, &color);

        Score {
            current: 0,
            record: 0,
            curr_obj,
            rec_obj,
        }
    }

    fn curr_as_string(&self) -> String {
        format!("{}: {:0>10}", CURR_LABEL, self.current)
    }

    fn best_as_string(&self) -> String {
        format!("{}: {:0>10}", REC_LABEL, self.record)
    }

    pub fn curr_ascii_matrix(&self) -> Vec<char> {
        self.curr_as_string().chars().collect()
    }

    pub fn best_ascii_matrix(&self) -> Vec<char> {
        self.best_as_string().chars().collect()
    }
}

impl DynComp for Score {
    fn update(&mut self) {
        self.current += 1;
        self.curr_obj.sprite.set_ascii(self.curr_ascii_matrix());
    }

    fn reset(&mut self) {
        self.record = self.record.max(self.current);
        self.current = 0;
        self.curr_obj.sprite.set_ascii(self.curr_ascii_matrix());
        self.rec_obj.sprite.set_ascii(self.best_ascii_matrix());
    }
}

impl Render for Score {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        self.curr_obj.render(out);
        self.rec_obj.render(out);
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        self.curr_obj.erase(out);
        self.rec_obj.erase(out)
    }
}
