//! The battle engine from Chapter 7 (solved): shared state, a chronicler
//! thread, and rounds where every combatant acts on its own thread.

mod events;
mod field;
mod report;
mod round;

pub use events::{BattleEvent, Outcome};
pub use field::{BattleState, Battlefield};
pub use report::{BattleReport, spawn_chronicler};
pub use round::{creature_turn, mage_turn, play_round, run_battle};
