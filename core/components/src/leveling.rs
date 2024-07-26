//! Leveling components.

use mythmallow_core_dependencies::*;


/// Component for the level of the entities of the game.
#[derive(Clone, Component, Copy, Debug, Deref, DerefMut, Eq, PartialEq, Reflect)]
pub struct Level(pub NonZeroU16);

impl Level {
    /// Creates a new level.
    ///
    /// # Panics
    ///
    /// - Panics if `level` is zero.
    pub fn new(level: u16) -> Level {
        Level(NonZeroU16::new(level).expect("expected level to be strictly positive"))
    }
}

impl Default for Level {
    fn default() -> Level {
        Level(NonZeroU16::new(1).unwrap())
    }
}


/// Component for the experience of the entities of the game.
#[derive(Clone, Copy, Component, Debug, Default, Deref, DerefMut, PartialOrd, PartialEq, Reflect)]
pub struct Experience(pub f64);

impl Experience {
    /// Zero experience.
    pub const ZERO: Experience = Experience(0.00);

    /// One experience.
    pub const ONE: Experience = Experience(1.00);
}

impl Display for Experience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_nan() {
            return write!(f, "?");
        }
        if self.is_infinite() {
            write!(f, "{}∞", if self.is_sign_positive() { "" } else { "-" })
        } else {
            write!(f, "{:.2}", self.0)
        }
    }
}
