use crate::graphics::{Pos, Render};

pub mod walls;
pub use walls::Walls;

use super::DynComp;

/// Defines behavior of components that can hit the player.
pub trait Enemy: Render + DynComp {
    /// Verify if the component hit the given position.
    fn hits(&self, pos: &Pos) -> bool;
}

/// Components that can hit the player.
pub struct Enemies {
    comps: Vec<Box<dyn Enemy>>
}

impl Enemies {
    /// Returns a new instance of `Enemies`.
    pub fn new() -> Self {
        Enemies { comps: Vec::new() }
    }

    /// Add a enemy to  the enemies set.
    pub fn add_enemy<E: Enemy + 'static>(&mut self, enemy: E) {
        self.comps.push(Box::new(enemy));
    }
}

impl DynComp for Enemies {
    fn update(&mut self) {
        self.comps.iter_mut().for_each(|comp| comp.update());
    }

    fn reset(&mut self) {
        self.comps.iter_mut().for_each(|comp| comp.reset());
    }
}

impl Render for Enemies {
    fn render(&self, out: &mut crate::graphics::TermOut) {
        self.comps.iter().for_each(|comp| comp.render(out));
    }

    fn erase(&self, out: &mut crate::graphics::TermOut) {
        self.comps.iter().for_each(|comp| comp.erase(out));
    }
}


impl Enemy for Enemies {
    fn hits(&self, pos: &Pos) -> bool {
        self.comps.iter().any(|comp| comp.hits(pos))
    }
}
