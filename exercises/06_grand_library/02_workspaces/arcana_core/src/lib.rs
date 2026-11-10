//! # arcana_core
//!
//! Shared foundations used by every tower of the citadel.
//! Only items marked `pub` (and reachable through `pub` modules or
//! `pub use` re-exports) can be used by other crates.

mod elements;

// 🔧 The other towers need `arcana_core::Element`, but `elements` is a
// private module. Don't make the module public — re-export just the enum.

/// The version of this crate, as written in its Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A crystal that stores mana.
#[derive(Debug)]
pub struct ManaCrystal {
    charge: u32,
}

impl ManaCrystal {
    /// A new crystal holding `charge` mana.
    pub fn new(charge: u32) -> ManaCrystal {
        ManaCrystal { charge }
    }

    /// The mana currently stored.
    pub fn charge(&self) -> u32 {
        self.charge
    }

    /// Take up to `amount` mana out of the crystal; returns what was drained.
    fn drain(&mut self, amount: u32) -> u32 {
        let drained = amount.min(self.charge);
        self.charge -= drained;
        drained
    }
}
