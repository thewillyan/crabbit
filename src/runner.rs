use std::{collections::LinkedList, io::Write, thread, time::Duration};
use termion::{clear, cursor};

use super::Game;

pub struct Runner {
    pub game: Game,
    pub move_queue: LinkedList<Move>,
}

impl Runner {
    pub fn new(game: Game) -> Runner {
        Runner {
            game,
            move_queue: LinkedList::new(),
        }
    }

    pub fn run<O: Write>(&mut self, out: &mut O) {
        self.game.stage.fill_hitmap();

        write!(out, "{}", clear::All).unwrap();
        for i in 1..=30 {
            self.game.render(out);

            // debug
            write!(out, "{}frame: {}", cursor::Goto(1, 1), i).unwrap();
            write!(
                out,
                "{}player possition: {:?}",
                cursor::Goto(1, 2),
                self.game.player.obj.pos
            )
            .unwrap();

            out.flush().unwrap();
            thread::sleep(Duration::from_millis(95));
            self.game.update();
        }

        // debug
        write!(
            out,
            "{}{}Hitmap: {:?}\r\n",
            clear::All,
            cursor::Goto(1, 1),
            self.game.stage.hitmap
        )
        .unwrap();
    }
}

pub enum Move {
    Up(u16),
    Down(u16),
}
