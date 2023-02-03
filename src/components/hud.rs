use std::io::Write;
use termion::{color::{Color, Fg}, cursor::Goto};

use super::DynComp;
use crate::graphics::{
    object::{Obj, Sprite},
    Pos, Render, Size,
};

/// Controls the HUD components.
///
/// The `Hud` has, mainly, two purposes: track user score throught the `Score` struct and show
/// splash screens throught the `Splash` struct.
#[derive(Debug)]
pub struct Hud {
    score: Score,
    splash: Splash,
}

impl Hud {
    /// Returns a new instance of `Hud`.
    pub fn new(splash: Splash) -> Self {
        let score = Score::new();
        Self { score, splash }
    }

    /// Ruturns a reference to the HUD score.
    pub fn score(&self) -> &Score {
        &self.score
    }

    /// Ruturns a mutable reference to the HUD score.
    pub fn score_mut(&mut self) -> &mut Score {
        &mut self.score
    }

    /// Ruturns a reference to the HUD splash screen.
    pub fn splash(&self) -> &Splash {
        &self.splash
    }

    /// Ruturns a mutable reference to the HUD splash screen.
    pub fn splash_mut(&mut self) -> &mut Splash {
        &mut self.splash
    }
}

impl Render for Hud {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        self.score.render(out);
        self.splash.render(out);
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        self.score.erase(out);
        self.splash.erase(out);
    }
}

impl DynComp for Hud {
    /// Updates the score.
    fn update(&mut self) {
        self.score.update();
    }

    /// Resets both the splash screen and the score.
    fn reset(&mut self) {
        self.score.reset();
        self.splash.default_state();
    }
}


/// Tracks the current and the best user score (which is displayed in the top-left corner) .
#[derive(Debug)]
pub struct Score {
    current: u32,
    record: u32,
}

impl Score {
    // current score label
    const CURR_LABEL: &'static str = "Score";
    // record score label
    const REC_LABEL: &'static str = "Record";

    /// Returns a new instace of `Score`.
    pub fn new() -> Self {
        Score {
            current: 0,
            record: 0,
        }
    }

    /// Returns the current score.
    ///
    /// The score is updated each time `DynComp` trait methods are called. [Read more]
    ///
    /// [Read more]: #impl-DynComp-for-Score
    pub fn current(&self) -> u32 {
        self.current
    }

    /// Returns the best score.
    ///
    /// The best score is updated each time `DynComp` trait methods are called. [Read more]
    ///
    /// [Read more]: #method.reset
    pub fn record(&self) -> u32 {
        self.record
    }

    /// Returns the current score as a formated `String`.
    pub fn curr_to_string(&self) -> String {
        format!("{}: {:0>10}", Self::CURR_LABEL, self.current)
    }

    /// Returns the current score as a formated `String`.
    pub fn best_to_string(&self) -> String {
        format!("{}: {:0>10}", Self::REC_LABEL, self.record)
    }

}

impl Default for Score {
    fn default() -> Self {
        Score::new()
    }
}

impl DynComp for Score {
    /// Increases the current score by 1.
    fn update(&mut self) {
        self.current += 1;
    }

    /// Sets the current score to 0 and checks if the best score has changed.
    fn reset(&mut self) {
        self.record = self.record.max(self.current);
        self.current = 0;
    }
}

impl Render for Score {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        write!(out, "{}{}", Goto(1,1), self.best_to_string()).unwrap();
        write!(out, "{}{}", Goto(1,2), self.curr_to_string()).unwrap();
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        let curr_empty = " ".repeat(self.curr_to_string().len());
        let best_empty = " ".repeat(self.best_to_string().len());
        write!(out, "{}{}", Goto(1,1), curr_empty).unwrap();
        write!(out, "{}{}", Goto(1,2), best_empty).unwrap();
    }
}

/// Creates a new splash screen object.
fn splash_obj<C: Color>(msg: &str, color: C, size: &Size) -> Obj {
    let h_center = (size.width / 2) + 1;
    let v_center = size.height / 2;
    let col = h_center.checked_sub((msg.len() / 2) as u16).unwrap_or(1);
    let center_pos = Pos { col, row: v_center };

    let chars = msg.chars().collect();
    let sprite = Sprite::new(chars, msg.len() as u16);
    Obj::new(center_pos, sprite, &Fg(color))
}


/// Represents the possible display options for the splash screen.
#[derive(Debug)]
pub enum SplashState {
    Title,
    Pause,
    OffScreen,
}

impl Default for SplashState {
    fn default() -> Self {
        Self::OffScreen
    }
}



/// Manages "title" and "pause" splash-screens with vertically and horizontally centered text.
#[derive(Debug)]
pub struct Splash {
    title: Obj,
    pause: Obj,
    state: SplashState,
}

impl Splash {
    /// Returns a new instance of `Splash`.
    ///
    /// `tcolor` and `pcolor` refers to the `title` and the `pause_msg` foreground colors on the
    /// splash screen, respectively.
    ///
    /// # Example
    ///
    /// ```
    /// use termion::color;
    /// use crabbit::graphics::Size;
    /// use crabbit::components::hud::Splash;
    ///
    /// let screen= Size {
    ///     width: 300,
    ///     height: 400
    /// };
    ///
    /// let title = "CRABBIT";
    /// let title_color = color::Blue;
    ///
    /// let pause_msg = "Paused";
    /// let pause_msg_color = color::Magenta;
    ///
    /// let splash: Splash = Splash::new(&screen, title, title_color, pause_msg, pause_msg_color);
    /// ```
    pub fn new<T, P>(size: &Size, title: &str, tcolor: T, pause_msg: &str, pcolor: P) -> Self
    where
        T: Color,
        P: Color,
    {
        let title = splash_obj(title, tcolor, size);
        let pause = splash_obj(pause_msg, pcolor, size);
        let state = SplashState::default();

        Splash {
            title,
            pause,
            state,
        }
    }

    /// Set the splash screen to the default.
    pub fn default_state(&mut self) {
        self.state = SplashState::default();
    }

    /// Set the splash screen to the pause text.
    pub fn pause(&mut self) {
        self.state = SplashState::Pause
    }

    /// Set the splash screen to the title text.
    pub fn title(&mut self) {
        self.state = SplashState::Title
    }

    /// Turn off the splash screen.
    pub fn off(&mut self) {
        self.state = SplashState::OffScreen
    }

    /// Returns `true` if some splash screen is setted and `false` otherwise.
    pub fn is_off(&self) -> bool {
        if let SplashState::OffScreen = self.state {
            return true;
        }
        false
    }

    /// Returns the current state.
    pub fn state(&self) -> &SplashState {
        &self.state
    }
}

impl Render for Splash {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        match self.state {
            SplashState::Title => self.title.render(out),
            SplashState::Pause => self.pause.render(out),
            SplashState::OffScreen => (),
        }
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        match self.state {
            SplashState::Title => self.title.erase(out),
            SplashState::Pause => self.pause.erase(out),
            SplashState::OffScreen => (),
        }
    }
}
