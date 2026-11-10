// 🏗️ Project 5 — Potions, Buffs & Effect Chains
// 📖 Read: book/src/05_temporal_loom/07_project_potions_buffs.md
//
// The top half of this file is the Chapter 4 combat system, already
// written for you: spells, `CastError`, the `Creature` trait and the `Mage`.
// The bottom half is yours: potions, buffs, burning effects and battle stats,
// built with closures, lifetimes and iterators.
//
// `src/main.rs` (already written) is a playable battle. Once your code works:
//
//     cargo run -p ch05_07_project_potions_buffs
//
// The tests in `tests/potions_buffs.rs` check your work, phase by phase.

use std::collections::HashMap;
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════
//  PROVIDED — the Chapter 4 combat system (no changes needed)
// ═══════════════════════════════════════════════════════════════════════

pub const MAX_MANA: u32 = 100;

#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub power: u32,
    pub level: u32,
}

impl Spell {
    pub fn new(name: &str, cost: u32, power: u32, level: u32) -> Self {
        Spell { name: name.to_string(), cost, power, level }
    }
}

/// The Academy's standard spellbook.
pub fn spellbook() -> Vec<Spell> {
    vec![
        Spell::new("Fireball", 30, 45, 1),
        Spell::new("Frostbolt", 25, 35, 2),
        Spell::new("Arcane Blast", 40, 60, 5),
        Spell::new("Heal", 20, 0, 1),
        Spell::new("Meteor", 90, 150, 10),
    ]
}

#[derive(Debug, Clone, PartialEq)]
pub enum CastError {
    UnknownSpell(String),
    NotEnoughMana { needed: u32, available: u32 },
    LevelTooLow { required: u32, current: u32 },
}

impl fmt::Display for CastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastError::UnknownSpell(name) => write!(f, "Unknown spell: {name}"),
            CastError::NotEnoughMana { needed, available } => {
                write!(f, "Not enough mana: need {needed}, have {available}")
            }
            CastError::LevelTooLow { required, current } => {
                write!(f, "Requires level {required} (you are level {current})")
            }
        }
    }
}

impl std::error::Error for CastError {}

/// Anything that can be fought.
pub trait Creature {
    fn name(&self) -> &str;
    fn hp(&self) -> u32;
    /// Damage this creature deals per turn.
    fn attack(&self) -> u32;
    fn take_damage(&mut self, amount: u32);
    fn is_alive(&self) -> bool {
        self.hp() > 0
    }
}

pub struct Goblin {
    pub hp: u32,
}

impl Creature for Goblin {
    fn name(&self) -> &str {
        "Goblin"
    }
    fn hp(&self) -> u32 {
        self.hp
    }
    fn attack(&self) -> u32 {
        3
    }
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
}

/// Wraiths are half in the spirit realm: they take only half damage.
pub struct Wraith {
    pub hp: u32,
}

impl Creature for Wraith {
    fn name(&self) -> &str {
        "Wraith"
    }
    fn hp(&self) -> u32 {
        self.hp
    }
    fn attack(&self) -> u32 {
        5
    }
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount / 2);
    }
}

pub struct Dragon {
    pub hp: u32,
}

impl Creature for Dragon {
    fn name(&self) -> &str {
        "Dragon"
    }
    fn hp(&self) -> u32 {
        self.hp
    }
    fn attack(&self) -> u32 {
        8
    }
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
}

#[derive(Debug, Clone)]
pub struct Mage {
    pub name: String,
    pub level: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub mana: u32,
    pub spells: Vec<Spell>,
}

impl Mage {
    /// A fresh mage: 100/100 HP, full mana, the standard spellbook.
    pub fn new(name: &str, level: u32) -> Self {
        Mage {
            name: name.to_string(),
            level,
            hp: 100,
            max_hp: 100,
            mana: MAX_MANA,
            spells: spellbook(),
        }
    }

    /// Look up a known spell, ignoring case.
    pub fn find_spell(&self, name: &str) -> Result<&Spell, CastError> {
        self.spells
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name.trim()))
            .ok_or_else(|| CastError::UnknownSpell(name.trim().to_string()))
    }

    /// Check level and mana, then spend the mana. Returns the spell cast.
    pub fn spend_for(&mut self, name: &str) -> Result<Spell, CastError> {
        let spell = self.find_spell(name)?.clone();
        if spell.level > self.level {
            return Err(CastError::LevelTooLow { required: spell.level, current: self.level });
        }
        if spell.cost > self.mana {
            return Err(CastError::NotEnoughMana { needed: spell.cost, available: self.mana });
        }
        self.mana -= spell.cost;
        Ok(spell)
    }

    /// Cast a spell at a creature. Returns the power unleashed.
    pub fn cast(&mut self, name: &str, target: &mut dyn Creature) -> Result<u32, CastError> {
        let spell = self.spend_for(name)?;
        target.take_damage(spell.power);
        Ok(spell.power)
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  YOUR WORK STARTS HERE
// ═══════════════════════════════════════════════════════════════════════

// ── Phase 1: potions (a closure you can only call once) ────────────────

/// A potion holds its effect as a boxed closure. Drinking a potion uses
/// it up, so the effect is an `FnOnce`.
pub struct Potion {
    pub name: String,
    effect: Box<dyn FnOnce(&mut Mage)>,
}

impl Potion {
    /// Brew a potion with any effect.
    pub fn new(name: &str, effect: impl FnOnce(&mut Mage) + 'static) -> Self {
        todo!()
    }

    /// Drink the potion: apply its effect to `mage`. The potion is gone after.
    pub fn drink(self, mage: &mut Mage) {
        todo!()
    }
}

/// "Healing Draught": restores `amount` HP, never above `max_hp`.
pub fn healing_draught(amount: u32) -> Potion {
    todo!()
}

/// "Mana Elixir": restores `amount` mana, never above `MAX_MANA`.
pub fn mana_elixir(amount: u32) -> Potion {
    todo!()
}

/// A satchel of potions.
#[derive(Default)]
pub struct Satchel {
    pub potions: Vec<Potion>,
}

impl Satchel {
    pub fn new() -> Self {
        Satchel { potions: Vec::new() }
    }

    pub fn add(&mut self, potion: Potion) {
        self.potions.push(potion);
    }

    /// The names of all potions, in order.
    pub fn names(&self) -> Vec<&str> {
        todo!()
    }

    /// Drink the first potion called `name` (ignoring case): remove it from
    /// the satchel and apply it. Returns `false` if there is no such potion.
    /// Hint: `position` + `Vec::remove`.
    pub fn drink(&mut self, name: &str, mage: &mut Mage) -> bool {
        todo!()
    }
}

// ── Phase 2: buffs & effect chains (closures you can call many times) ──

/// A timed buff that modifies spell power.
pub struct Buff {
    pub name: String,
    pub turns_left: u32,
    modifier: Box<dyn Fn(u32) -> u32>,
}

impl Buff {
    pub fn new(name: &str, turns: u32, modifier: impl Fn(u32) -> u32 + 'static) -> Self {
        todo!()
    }

    /// Run `power` through this buff's modifier.
    pub fn apply(&self, power: u32) -> u32 {
        todo!()
    }
}

/// "Empower": multiplies power by (100 + percent) / 100 (integer math:
/// multiply first, then divide). `empower(50, _)` turns 45 into 67.
pub fn empower(percent: u32, turns: u32) -> Buff {
    todo!()
}

/// "Arcane Focus": adds a flat `bonus` to the power.
pub fn arcane_focus(bonus: u32, turns: u32) -> Buff {
    todo!()
}

/// Feed `power` through every buff, in order — the output of one is the
/// input of the next. (Hint: `fold`.)
pub fn chain_power(buffs: &[Buff], power: u32) -> u32 {
    todo!()
}

impl Mage {
    /// Like `cast`, but the spell's power goes through `modifier` first.
    /// The modified power is dealt to the target and returned.
    /// All `CastError`s of `cast` still apply (reuse `spend_for`!).
    pub fn cast_with(
        &mut self,
        name: &str,
        target: &mut dyn Creature,
        modifier: impl Fn(u32) -> u32,
    ) -> Result<u32, CastError> {
        todo!()
    }
}

// ── Phase 3: timed effects that tick every turn ─────────────────────────

/// One turn passes for the buffs: every buff loses a turn, and buffs with
/// no turns left are removed. (Hint: `iter_mut` + `retain`.)
pub fn tick_buffs(buffs: &mut Vec<Buff>) {
    todo!()
}

/// Damage over time.
#[derive(Debug, Clone, PartialEq)]
pub struct Burn {
    pub source: String,
    pub damage_per_turn: u32,
    pub turns_left: u32,
}

/// One turn passes for the burns on `target`: every burn deals its damage
/// to the target and loses a turn; burns with no turns left are removed.
/// Returns the total damage dealt this turn (as passed to `take_damage`).
pub fn tick_burns(burns: &mut Vec<Burn>, target: &mut dyn Creature) -> u32 {
    todo!()
}

// ── Phase 4: battle statistics (iterators & lifetimes) ──────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CastRecord {
    pub turn: u32,
    pub spell: String,
    pub damage: u32,
}

/// Total damage in the log.
pub fn total_damage(log: &[CastRecord]) -> u32 {
    todo!()
}

/// The spell cast most often. On a tie, the alphabetically first name.
/// `None` for an empty log. The result borrows from the log.
pub fn favourite_spell(log: &[CastRecord]) -> Option<&str> {
    todo!()
}

/// The names of the creatures that are still alive, in order.
pub fn living(creatures: &[Box<dyn Creature>]) -> Vec<&str> {
    todo!()
}

/// The index of the living creature with the lowest HP (on a tie: the
/// first one), or `None` if all are defeated.
pub fn weakest_living(creatures: &[Box<dyn Creature>]) -> Option<usize> {
    todo!()
}

/// One line per cast: "Turn 1: Fireball for 45", joined with '\n'.
pub fn battle_report(log: &[CastRecord]) -> String {
    todo!()
}

/// How many casts of each spell, e.g. {"Fireball": 2, "Heal": 1}.
pub fn spell_counts(log: &[CastRecord]) -> HashMap<&str, usize> {
    todo!()
}
