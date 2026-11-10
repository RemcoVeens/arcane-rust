//! 🔨 Phase 1 — The Battlefield: one shared state, many threads.

use std::sync::{Arc, Mutex};

use super::events::Outcome;
use crate::creature::Creature;
use crate::error::BattleError;
use crate::mage::Mage;
use crate::spells::Spell;

/// Everything on the battlefield at one moment.
#[derive(Debug, Clone, PartialEq)]
pub struct BattleState {
    pub mage: Mage,
    pub creatures: Vec<Creature>,
}

/// A handle to the shared battlefield. Cloning it is cheap and gives another
/// handle to the SAME state — hand one clone to every combatant thread.
///
/// Every method locks the mutex, does its work, and lets the guard drop.
/// Never hold the lock longer than one method call.
#[derive(Debug, Clone)]
pub struct Battlefield {
    state: Arc<Mutex<BattleState>>,
}

impl Battlefield {
    pub fn new(mage: Mage, creatures: Vec<Creature>) -> Self {
        todo!()
    }

    /// A copy of the current state (for reading without holding the lock).
    pub fn snapshot(&self) -> BattleState {
        todo!()
    }

    /// Damage creature number `index`. Returns `true` only if THIS blow
    /// brought it from alive to 0 hp. Hitting a creature that is already down
    /// (or an index that doesn't exist) does nothing and returns `false`.
    pub fn strike_creature(&self, index: usize, damage: u32) -> bool {
        todo!()
    }

    /// Damage the mage. Returns `true` only if THIS blow brought the mage
    /// from alive to 0 hp — so exactly one attacker reports the mage's fall,
    /// whichever thread happens to land it.
    pub fn strike_mage(&self, damage: u32) -> bool {
        todo!()
    }

    /// The mage pays for `spell` (see `Mage::cast`) on the shared state.
    pub fn cast(&self, spell: &Spell) -> Result<u32, BattleError> {
        todo!()
    }

    /// - mage down            → `Some(Outcome::Defeat)` (checked first)
    /// - all creatures down   → `Some(Outcome::Victory)`
    /// - otherwise            → `None` (the battle goes on)
    pub fn outcome(&self) -> Option<Outcome> {
        todo!()
    }
}
