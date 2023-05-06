//! Controls the `Game` flow and handle user actions.

use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use termion::{clear, cursor, event::Key, input::TermRead};

use crate::{
    components::{enemies::Enemy, DynComp},
    game::Game,
    graphics::Render,
};

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
    Pause,
    Restart,
    Quit,
}

/// Controls the run of a [`Game`].
///
/// [`Game`]: crate::game::Game
pub struct Runner {
    game: Game,
    delay: u64,
    proceed: bool,
}

impl Runner {
    /// Returns a new Runner instance.
    pub fn new(game: Game) -> Self {
        Runner {
            game,
            delay: INI_DELAY,
            proceed: true,
        }
    }

    /// Runs the game.
    pub fn run(&mut self, out: &mut crate::graphics::TermOut) {
        let act_stream = Self::act_input();
        self.game.hud.splash_mut().title();

        write!(out, "{}{}", clear::All, cursor::Hide).unwrap();
        loop {
            self.game.render(out);
            out.flush().unwrap();

            // check if player has died
            let player_pos = &self.game.player.obj.pos;
            let has_died = self.game.enemies.hits(player_pos);
            if has_died {
                self.restart();
                continue;
            }

            // check the user input. the game pauses (freezes) each time that some splash screen is
            // displayed.
            if !self.game.hud.splash().is_off() {
                let act = act_stream.recv().expect("Error on input (paused game).");
                self.act_handler(act);
                // remove splash screen
                self.game.hud.splash().erase(out);
                self.game.hud.splash_mut().off();
            } else if let Ok(act) = act_stream.try_recv() {
                self.act_handler(act);
            }

            if !self.proceed {
                break;
            }
            self.game.update();

            // decrement the delay (makes the game go faster)
            if self.delay > MIN_DELAY {
                self.delay = INI_DELAY - (self.game.hud.score().current() / DELAY_STEP) as u64;
            }
            thread::sleep(Duration::from_millis(self.delay));
        }
        write!(out, "{}{}\r", cursor::Show, clear::All).unwrap();
    }

    /// Returns a mpsc receiver over the user actions.
    fn act_input() -> Receiver<Act> {
        let (tx, rx) = mpsc::sync_channel(1);

        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char(' ') => tx.send(Act::PlayerJump).unwrap(),
                    Key::Char('q') | Key::Char('Q') => tx.send(Act::Quit).unwrap(),
                    Key::Char('r') | Key::Char('R') => tx.send(Act::Restart).unwrap(),
                    Key::Esc => tx.send(Act::Pause).unwrap(),
                    _ => (),
                }
            }
        });
        rx
    }

    /// Handle user actions.
    fn act_handler(&mut self, act: Act) {
        match act {
            Act::PlayerJump => self.game.player.jump(JUMP_HEIGHT),
            Act::Pause => {
                if self.game.hud.splash().is_off() {
                    self.game.hud.splash_mut().pause()
                }
            }
            Act::Restart => self.restart(),
            Act::Quit => self.quit(),
        }
    }

    /// Restart the game.
    fn restart(&mut self) {
        self.delay = INI_DELAY;
        self.game.reset();
        self.game.update();
    }

    /// Quit the game.
    fn quit(&mut self) {
        self.proceed = false;
    }
}
