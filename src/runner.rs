use std::{
    collections::LinkedList,
    io::Write,
    thread,
    time::Duration
};
use termion::{clear, cursor};

use super::Game;

pub struct Runner {
    pub game: Game,
    pub move_queue: LinkedList<Move> 
}

impl Runner {
    pub fn new(game: Game) -> Runner {
        Runner { game, move_queue: LinkedList::new() }
    }

    pub fn run<O: Write>(&mut self, out: &mut O) {
        let stage = &mut self.game.stage;
        stage.fill_hitmap();
        let player = &self.game.player;

        write!(out, "{}", clear::All).unwrap();
        for i in 1..=30 {
            stage.render(out);
            player.obj.render(out);
            write!(out, "{}frame: {}", cursor::Goto(1,1), i).unwrap();
            write!(out, "{}player possition: {:?}", cursor::Goto(1,2), player.obj.pos).unwrap();
            out.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            stage.shift();
        }

        write!(out, "{}{}Hitmap: {:?}\r\n", clear::All, cursor::Goto(1,1), stage.hitmap)
            .unwrap();
    }
}

pub enum Move {
    Up(u16),
    Down(u16)
}
