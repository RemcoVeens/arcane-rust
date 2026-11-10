//! The chronicler: one thread receives every event and writes the report.

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};

use super::events::{BattleEvent, Outcome};

/// The chronicle of one battle.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleReport {
    pub lines: Vec<String>,
    pub damage_dealt: HashMap<String, u32>,
    pub defeated: Vec<String>,
    pub fizzles: u32,
    pub rounds: u32,
    pub outcome: Outcome,
}

impl BattleReport {
    /// Add one event to the chronicle.
    pub fn record(&mut self, event: &BattleEvent) {
        self.lines.push(event.to_string());
        match event {
            BattleEvent::Attack { attacker: who, damage, .. }
            | BattleEvent::SpellCast { caster: who, damage, .. } => {
                *self.damage_dealt.entry(who.clone()).or_insert(0) += damage;
            }
            BattleEvent::Fizzle { .. } => self.fizzles += 1,
            BattleEvent::Defeated { name } => self.defeated.push(name.clone()),
        }
    }

    /// Total damage dealt by `name`.
    pub fn damage_by(&self, name: &str) -> u32 {
        self.damage_dealt.get(name).copied().unwrap_or(0)
    }
}

/// Spawn a thread that records events until the channel closes.
pub fn spawn_chronicler(events: Receiver<BattleEvent>) -> JoinHandle<BattleReport> {
    thread::spawn(move || {
        let mut report = BattleReport::default();
        for event in events {
            report.record(&event);
        }
        report
    })
}
