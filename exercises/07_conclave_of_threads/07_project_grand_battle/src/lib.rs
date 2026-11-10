// 🏗️ Project 7 — The Grand Battle
// 📖 Read: book/src/07_conclave_of_threads/07_project_grand_battle.md
//
// The game from the earlier chapters is already here, organised in modules:
//
//   spells.rs    ✅ Element, Spell, the grimoire, find_spell
//   creature.rs  ✅ Creature (goblins, wraiths, golems, dragons)
//   mage.rs      ✅ Mage (Tiberius) and casting rules
//   error.rs     ✅ BattleError
//   battle/
//     events.rs  ✅ BattleEvent + Outcome
//     field.rs   🔨 Phase 1 — the shared battlefield (Arc<Mutex<…>>)
//     report.rs  🔨 Phase 2 — the chronicler thread (channels)
//     round.rs   🔨 Phase 3 + 4 — concurrent turns and the full battle (threads)
//
// Fill in every `todo!()` in the 🔨 files, then play:
//
//     cargo run -p ch07_07_project_grand_battle
//
// The tests in `tests/grand_battle.rs` check your work, phase by phase.

//! # Grand Battle
//!
//! Tiberius faces a horde. Every combatant acts on its own thread, all
//! events flow through a channel to a chronicler, and the battlefield is
//! shared behind an `Arc<Mutex<…>>`.

pub mod battle;
pub mod creature;
pub mod error;
pub mod mage;
pub mod spells;

pub use battle::{
    BattleEvent, BattleReport, BattleState, Battlefield, Outcome, creature_turn, mage_turn,
    play_round, run_battle, spawn_chronicler,
};
pub use creature::Creature;
pub use error::BattleError;
pub use mage::Mage;
pub use spells::{Element, Spell, find_spell, grimoire};
