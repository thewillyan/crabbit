use crate::{components::enemies::Hitmap, Game};
use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use termion::{clear, cursor, event::Key, input::TermRead};

// intial delay (milliseconds)
const INI_DELAY: u64 = 60;
// minimum delay (milliseconds), in other words, the delay cannot be lower than this.
const MIN_DELAY: u64 = 30;
// how many points are needed for the speed to increase
const DELAY_STEP: u32 = 50;
// player jump height
const JUMP_HEIGHT: u16 = 3;

/// Posible user actions.
pub enum Act {
    PlayerJump,
    Restart,
    Quit,
}

/// Controls the run of a Game.
pub struct Runner {
    game: Game,
    delay: u64,
}

impl Runner {
    /// Returns a runner for the givenn game.
    pub fn new(game: Game) -> Runner {
        Runner {
            game,
            delay: INI_DELAY,
        }
    }

    /// Run game.
    pub fn run<O: Write>(&mut self, out: &mut O) {
        let act_stream = Self::act_input();

        write!(out, "{}{}", clear::All, cursor::Hide).unwrap();
        loop {
            self.game.render(out);
            out.flush().unwrap();

            //check if player has died
            let player_pos = &self.game.player.obj.pos;
            let has_died = self.game.walls.hits(player_pos);

            if has_died {
                self.restart();
                continue;
            }

            // check user input
            if let Ok(act) = act_stream.try_recv() {
                match act {
                    Act::PlayerJump => self.game.player.jump(JUMP_HEIGHT),
                    Act::Restart => self.restart(),
                    Act::Quit => break,
                }
            }
            self.game.update();

            if self.delay > MIN_DELAY {
                self.delay = INI_DELAY - (self.game.player.score / DELAY_STEP) as u64;
            }
            thread::sleep(Duration::from_millis(self.delay));
        }
        write!(out, "{}{}\r", cursor::Show, clear::All).unwrap();
    }

    /// Restart game.
    fn restart(&mut self) {
        self.delay = INI_DELAY;
        self.game.reset();
        self.game.update();
    }

    /// Returns a mpsc receiver over the user action.
    fn act_input() -> Receiver<Act> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char(' ') => tx.send(Act::PlayerJump).unwrap(),
                    Key::Char('q') | Key::Char('Q') => tx.send(Act::Quit).unwrap(),
                    Key::Char('r') | Key::Char('R') => tx.send(Act::Restart).unwrap(),
                    _ => (),
                }
            }
        });
        rx
    }
}
