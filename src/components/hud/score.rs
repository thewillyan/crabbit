use termion::color;

use crate::{
    components::DynComp,
    graphics::{Obj, Pos, Render, Sprite},
};

const CURR_LABEL: &'static str = "Score";
const REC_LABEL: &'static str = "Record";

pub struct Score {
    pub current: u32,
    pub record: u32,
    curr_obj: Obj,
    rec_obj: Obj,
}

impl Score {
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

    pub fn sprite_width() -> u16 {
        let max_label = CURR_LABEL.len().max(REC_LABEL.len()) as u16;
        // max label len + separator len + score len
        max_label + 2 + 10
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
    fn render<O: std::io::Write>(&self, out: &mut O) {
        self.curr_obj.render(out);
        self.rec_obj.render(out);
    }

    fn erase<O: std::io::Write>(&self, out: &mut O) {
        self.curr_obj.erase(out);
        self.rec_obj.erase(out)
    }
}
