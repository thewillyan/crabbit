use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use termion::{clear, cursor, event::Key, input::TermRead};

use crate::{components::player::PlayerState, enemies::Hitmap, Game};

pub enum Act {
    PlayerJump,
    Reset,
    Quit,
}

pub struct Runner;

impl Runner {
    pub fn run<O: Write>(mut game: Game, out: &mut O) {
        let act_stream = Self::act_input();

        write!(out, "{}{}", clear::All, cursor::Hide).unwrap();
        loop {
            game.render(out);
            out.flush().unwrap();

            // check user input
            if let Ok(act) = act_stream.try_recv() {
                match (act, &game.player.state) {
                    (Act::Quit, _) => break,
                    (Act::Reset, _) => game.reset(),
                    (Act::PlayerJump, PlayerState::Running) => game.player.jump(3),
                    _ => (),
                }
            }
            game.update();

            //check if player has died
            let player_pos = &game.player.obj.pos;
            let has_died = game.walls.hits(player_pos);

            if has_died {
                game.reset();
            }

            thread::sleep(Duration::from_millis(70));
        }
        write!(out, "{}{}\r", cursor::Show, clear::All).unwrap();
    }

    pub fn act_input() -> Receiver<Act> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char(' ') => tx.send(Act::PlayerJump).unwrap(),
                    Key::Char('q') | Key::Char('Q') => tx.send(Act::Quit).unwrap(),
                    Key::Char('r') | Key::Char('R') => tx.send(Act::Reset).unwrap(),
                    _ => (),
                }
            }
        });
        rx
    }
}

