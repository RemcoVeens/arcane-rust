//! The battle engine: shared state, a chronicler, and concurrent rounds.

mod events;
mod field;
mod report;
mod round;

pub use events::{BattleEvent, Outcome};
pub use field::{BattleState, Battlefield};
pub use report::{BattleReport, spawn_chronicler};
pub use round::{creature_turn, mage_turn, play_round, run_battle};
