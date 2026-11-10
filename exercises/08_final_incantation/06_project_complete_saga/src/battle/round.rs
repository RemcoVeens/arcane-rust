//! Concurrent rounds: every combatant acts on its own thread.

use std::sync::mpsc::{self, Sender};
use std::thread;

use super::events::{BattleEvent, Outcome};
use super::field::{BattleState, Battlefield};
use super::report::{BattleReport, spawn_chronicler};
use crate::creature::Creature;
use crate::error::BattleError;
use crate::mage::Mage;
use crate::spells::find_spell;

/// A creature attacks the mage.
pub fn creature_turn(field: &Battlefield, attacker: &Creature, mage_name: &str, tx: &Sender<BattleEvent>) {
    let felled = field.strike_mage(attacker.attack);
    tx.send(BattleEvent::Attack {
        attacker: attacker.name.clone(),
        target: mage_name.to_string(),
        damage: attacker.attack,
    })
    .unwrap();
    if felled {
        tx.send(BattleEvent::Defeated { name: mage_name.to_string() }).unwrap();
    }
}

/// The mage casts `spell_name` at the first living creature (or fizzles).
pub fn mage_turn(field: &Battlefield, spell_name: &str, tx: &Sender<BattleEvent>) {
    let state = field.snapshot();
    let caster = state.mage.name.clone();
    let fizzle = |err: BattleError| {
        tx.send(BattleEvent::Fizzle { caster: caster.clone(), reason: err.to_string() }).unwrap();
    };

    let Some((index, target)) = state.creatures.iter().enumerate().find(|(_, c)| c.is_alive()) else {
        return fizzle(BattleError::NoTargets);
    };
    let spell = match find_spell(spell_name).and_then(|spell| field.cast(&spell).map(|_| spell)) {
        Ok(spell) => spell,
        Err(err) => return fizzle(err),
    };

    let damage = target.damage_from(&spell);
    let felled = field.strike_creature(index, damage);
    tx.send(BattleEvent::SpellCast {
        caster: caster.clone(),
        spell: spell.name.clone(),
        target: target.name.clone(),
        damage,
    })
    .unwrap();
    if felled {
        tx.send(BattleEvent::Defeated { name: target.name.clone() }).unwrap();
    }
}

/// One simultaneous round: everyone alive at the start acts on its own thread.
pub fn play_round(field: &Battlefield, spell_name: &str, tx: &Sender<BattleEvent>) {
    let start = field.snapshot();
    let mut handles = Vec::new();

    if start.mage.is_alive() {
        let (field, tx, spell) = (field.clone(), tx.clone(), spell_name.to_string());
        handles.push(thread::spawn(move || mage_turn(&field, &spell, &tx)));
    }
    for creature in start.creatures.into_iter().filter(|c| c.is_alive()) {
        let (field, tx, mage_name) = (field.clone(), tx.clone(), start.mage.name.clone());
        handles.push(thread::spawn(move || creature_turn(&field, &creature, &mage_name, &tx)));
    }

    for h in handles {
        h.join().unwrap();
    }
}

/// A full battle: one round per spell in `plan`, stopping at the first outcome.
///
/// Returns the final battlefield state (so the caller can see who is left
/// standing and what the mage has left) together with the chronicle.
pub fn run_battle(mage: Mage, creatures: Vec<Creature>, plan: &[&str]) -> (BattleState, BattleReport) {
    let field = Battlefield::new(mage, creatures);
    let (tx, rx) = mpsc::channel();
    let chronicler = spawn_chronicler(rx);

    let mut rounds = 0;
    for spell in plan {
        if field.outcome().is_some() {
            break;
        }
        play_round(&field, spell, &tx);
        rounds += 1;
    }

    drop(tx);
    let mut report = chronicler.join().unwrap();
    report.rounds = rounds;
    report.outcome = field.outcome().unwrap_or(Outcome::Retreat);
    (field.snapshot(), report)
}
