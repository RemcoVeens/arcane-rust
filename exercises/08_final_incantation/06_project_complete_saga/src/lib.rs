// 🏗️ Project 8 — The Complete Saga
// 📖 Read: book/src/08_final_incantation/06_project_complete_saga.md
//
// Everything you built is here — the Grand Battle from Chapter 7 included,
// fully working. This final project ties it into one saga:
//
//   spells.rs    🔨 Phase 1 — the `grimoire!` macro (the spell table DSL)
//   party.rs     🔨 Phase 2 — the party: Rc<RefCell<Mage>> heroes + a shared stash
//   rewards.rs   🔨 Phase 3 — bounties: a trait with an associated type + iterators
//   saga.rs      🔨 Phase 4 — the saga: party ⇄ threaded battles ⇄ rewards
//   creature.rs  ✅ · mage.rs ✅ · error.rs ✅ · battle/ ✅ (Chapter 7, solved)
//
// Fill in every `todo!()` in the 🔨 files, then play:
//
//     cargo run -p ch08_06_project_complete_saga
//
// The tests in `tests/saga.rs` check your work, phase by phase.

//! # The Complete Saga
//!
//! A small text RPG: a party of mages, a macro-defined grimoire, battles where
//! every combatant acts on its own thread, and bounties for the victors.
//!
//! ```
//! use ch08_06_project_complete_saga::find_spell;
//! assert_eq!(find_spell("Frostbolt").unwrap().cost, 25);
//! ```

pub mod battle;
pub mod creature;
pub mod error;
pub mod mage;
pub mod party;
pub mod rewards;
pub mod saga;
pub mod spells;

pub use battle::{BattleEvent, BattleReport, BattleState, Battlefield, Outcome, run_battle};
pub use creature::Creature;
pub use error::BattleError;
pub use mage::Mage;
pub use party::{Hero, Party, Stash};
pub use rewards::{Bounty, Chest, Loot, spoils, total_gold};
pub use saga::Saga;
pub use spells::{Element, Spell, find_spell, grimoire};
