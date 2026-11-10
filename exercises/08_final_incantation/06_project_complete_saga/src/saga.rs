//! 🔨 Phase 4 — The Saga: the party meets the battle engine.
//!
//! Heroes live in `Rc<RefCell<Mage>>` — fine on one thread, but `Rc` may
//! never cross into the battle threads (it isn't `Send`). So each encounter:
//! clone the `Mage` *out* of its cell, let `run_battle` do its threaded
//! work, then write the survivor *back* into the cell.

use crate::battle::{Outcome, run_battle};
use crate::creature::Creature;
use crate::error::BattleError;
use crate::party::Party;

pub struct Saga {
    pub party: Party,
    /// One line per encounter, e.g. "Tiberius vs Goblin, Wraith: Victory in 3 rounds".
    pub chronicle: Vec<String>,
}

impl Saga {
    pub fn new(party: Party) -> Self {
        Saga { party, chronicle: Vec::new() }
    }

    /// Fight one encounter with the party's current leader.
    ///
    /// 1. No leader → `Err(BattleError::NoHeroes)`.
    /// 2. Clone the leader's `Mage` out of the `RefCell` and `run_battle`.
    /// 3. Write the battle's final mage back into the leader's cell.
    /// 4. On `Victory`: add the `spoils` of the final creatures to the stash
    ///    (gold added, trophies appended).
    /// 5. Push a chronicle line:
    ///    "<leader> vs <creature names joined with ", ">: <outcome:?> in <rounds> rounds"
    /// 6. Return the outcome.
    pub fn encounter(&mut self, creatures: Vec<Creature>, plan: &[&str]) -> Result<Outcome, BattleError> {
        todo!()
    }
}
