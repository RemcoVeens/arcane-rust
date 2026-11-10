//! 🔨 Phase 3 + 4 — The Clash: every combatant acts on its own thread.

use std::sync::mpsc::Sender;

use super::events::BattleEvent;
use super::field::Battlefield;
use super::report::BattleReport;
use crate::creature::Creature;
use crate::mage::Mage;

/// One creature's turn: it attacks the mage for `attacker.attack`.
///
/// Send `BattleEvent::Attack { attacker, target: mage_name, damage }`.
/// If that blow felled the mage (`strike_mage` returned `true`), also send
/// `BattleEvent::Defeated { name: mage_name }`.
pub fn creature_turn(field: &Battlefield, attacker: &Creature, mage_name: &str, tx: &Sender<BattleEvent>) {
    todo!()
}

/// The mage's turn: cast `spell_name` at the FIRST creature that is still
/// alive (lowest index).
///
/// 1. No living creature → send a `Fizzle` whose reason is `BattleError::NoTargets`'s message.
/// 2. `find_spell`, then `field.cast(&spell)`; any error → send a `Fizzle`
///    with `reason: err.to_string()`.
/// 3. Success: damage = `target.damage_from(&spell)`; strike the creature and
///    send `SpellCast { caster, spell, target, damage }` (use the spell's
///    proper name from the grimoire), plus `Defeated` if the blow felled it.
pub fn mage_turn(field: &Battlefield, spell_name: &str, tx: &Sender<BattleEvent>) {
    todo!()
}

/// One round of simultaneous combat.
///
/// Take a snapshot at the START of the round. Everyone alive in that snapshot
/// acts this round — even if they fall mid-round (blows are simultaneous):
/// - the mage, if alive, runs `mage_turn` on its own thread;
/// - every living creature runs `creature_turn` on its own thread.
///
/// Wait for every thread before returning, so all of this round's events
/// have been sent when `play_round` returns.
pub fn play_round(field: &Battlefield, spell_name: &str, tx: &Sender<BattleEvent>) {
    todo!()
}

/// 🔨 Phase 4 — The full battle.
///
/// 1. Build a `Battlefield`, a channel, and spawn the chronicler.
/// 2. Play one round per spell in `plan`, stopping early as soon as the
///    battlefield has an outcome.
/// 3. Close the channel, join the chronicler, then fill in `rounds`
///    (rounds actually played) and `outcome` (`Retreat` if nobody fell).
pub fn run_battle(mage: Mage, creatures: Vec<Creature>, plan: &[&str]) -> BattleReport {
    todo!()
}
