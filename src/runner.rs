use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use termion::{clear, cursor, event::Key, input::TermRead};

use super::Game;

pub struct Runner;

impl Runner {
    pub fn run<O: Write>(mut game: Game, out: &mut O) {
        let act_stream = Self::act_input();

        write!(out, "{}{}", clear::All, cursor::Hide).unwrap();
        let mut frame = 0;

        loop {
            game.render(out);

            // debug
            frame += 1;
            write!(out, "{}frame: {}", cursor::Goto(1, 1), frame).unwrap();
            write!(
                out,
                "{}player possition: {:?}",
                cursor::Goto(1, 2),
                game.player.obj.pos
            )
            .unwrap();

            out.flush().unwrap();
            thread::sleep(Duration::from_millis(95));

            if let Ok(act) = act_stream.try_recv() {
                match act {
                    Act::Quit => break,
                    Act::PlayerJump => game.player.jump(3),
                }
            }
            game.update();
        }

        // debug
        write!(
            out,
            "{}{}Hitmap: {:?}\r\n",
            clear::All,
            cursor::Goto(1, 1),
            game.stage.hitmap
        )
        .unwrap();

        write!(out, "{}", cursor::Show).unwrap();
    }

    pub fn act_input() -> Receiver<Act> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char(' ') => tx.send(Act::PlayerJump).unwrap(),
                    Key::Char('q') | Key::Char('Q') => tx.send(Act::Quit).unwrap(),
                    _ => (),
                }
            }
        });

        rx
    }
}

pub enum Act {
    PlayerJump,
    Quit,
}
