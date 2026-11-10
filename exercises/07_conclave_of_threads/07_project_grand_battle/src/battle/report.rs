//! 🔨 Phase 2 — The Chronicler: one thread receives every event.

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use super::events::{BattleEvent, Outcome};

/// The chronicle of one battle.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleReport {
    /// Every event, formatted with `Display`, in the order it was received.
    pub lines: Vec<String>,
    /// Total damage per attacker/caster (from `Attack` and `SpellCast`).
    pub damage_dealt: HashMap<String, u32>,
    /// Names from `Defeated` events, in the order received.
    pub defeated: Vec<String>,
    /// Number of `Fizzle` events.
    pub fizzles: u32,
    /// Filled in by `run_battle` (Phase 4), not by `record`.
    pub rounds: u32,
    /// Filled in by `run_battle` (Phase 4), not by `record`.
    pub outcome: Outcome,
}

impl BattleReport {
    /// Add one event to the chronicle (see the field docs above).
    pub fn record(&mut self, event: &BattleEvent) {
        todo!()
    }

    /// Total damage dealt by `name` (0 if they dealt none).
    pub fn damage_by(&self, name: &str) -> u32 {
        todo!()
    }
}

/// Spawn the chronicler thread. It records every event it receives and,
/// once the channel closes (every `Sender` dropped), returns the report.
pub fn spawn_chronicler(events: Receiver<BattleEvent>) -> JoinHandle<BattleReport> {
    todo!()
}
