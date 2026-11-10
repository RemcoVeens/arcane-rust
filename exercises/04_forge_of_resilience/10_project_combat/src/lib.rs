// 🏗️ Project 4 — The Combat System
// 📖 Read: book/src/04_forge_of_resilience/10_project_combat.md
//
// The scaffold already contains the Chapter 3 game: `Element`, `Spell`, `Mage`
// and `Command` parsing. Your job, phase by phase (see the book page):
//
//   Phase 1 — creatures as trait objects        (search for "PHASE 1")
//   Phase 2 — spellbook (Vec) + inventory (HashMap)
//   Phase 3 — Result-based casting with `CastError`
//   Phase 4 — the full turn
//
// ❌ The crate does not compile until you write `impl Creature for Dragon`
//    (Phase 1) — that's your first task.
//
// The tests in `tests/combat.rs` check your work. When they pass, fight:
//
//     cargo run -p ch04_10_project_combat

use std::collections::HashMap;
use std::fmt;

pub const MAX_MANA: u32 = 100;
pub const MAX_HP: u32 = 100;
pub const HEAL_AMOUNT: u32 = 30;
pub const MANA_POTION: u32 = 50;
pub const HEALTH_POTION: u32 = 40;

// ══════════════════════════════════════════════════════════════════════
//  SCAFFOLD (Chapter 3) — already done, read it but don't change it
// ══════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Fire,
    Ice,
    Arcane,
    /// Holy spells heal the caster instead of hurting a target.
    Holy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub power: u32,
    pub level: u32,
    pub element: Element,
}

impl Spell {
    pub fn new(name: &str, cost: u32, power: u32, level: u32, element: Element) -> Self {
        Spell { name: name.to_string(), cost, power, level, element }
    }
}

/// The five spells every mage of the Academy starts with.
pub fn starter_spells() -> Vec<Spell> {
    vec![
        Spell::new("Fireball", 30, 45, 1, Element::Fire),
        Spell::new("Frostbolt", 25, 35, 2, Element::Ice),
        Spell::new("Arcane Blast", 40, 60, 5, Element::Arcane),
        Spell::new("Heal", 20, 0, 1, Element::Holy),
        Spell::new("Meteor", 90, 150, 10, Element::Fire),
    ]
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mage {
    pub name: String,
    pub level: u32,
    pub hp: u32,
    pub mana: u32,
}

impl Mage {
    pub fn new(name: &str, level: u32) -> Self {
        Mage { name: name.to_string(), level, hp: MAX_HP, mana: MAX_MANA }
    }

    /// Restore hp, never above `MAX_HP`.
    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(MAX_HP);
    }

    /// Restore mana, never above `MAX_MANA`.
    pub fn restore_mana(&mut self, amount: u32) {
        self.mana = (self.mana + amount).min(MAX_MANA);
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    /// `target` is a 0-based index into the enemy list.
    Cast { spell: String, target: usize },
    Drink(String),
    Status,
    Help,
    Quit,
}

/// Parse a line of player input (case-insensitive):
///
///   "cast 2 frostbolt"   → Cast { spell: "frostbolt", target: 1 }   (players count from 1)
///   "cast heal"          → Cast { spell: "heal", target: 0 }
///   "drink mana potion"  → Drink("mana potion")
///   "status" / "help" / "quit"
pub fn parse_command(input: &str) -> Option<Command> {
    let input = input.trim().to_lowercase();
    let (verb, rest) = match input.split_once(' ') {
        Some((verb, rest)) => (verb, rest.trim()),
        None => (input.as_str(), ""),
    };
    match verb {
        "cast" if !rest.is_empty() => {
            let (first, after) = match rest.split_once(' ') {
                Some((first, after)) => (first, after.trim()),
                None => (rest, ""),
            };
            match first.parse::<usize>() {
                Ok(number) => {
                    let target = number.checked_sub(1)?;
                    if after.is_empty() {
                        return None;
                    }
                    Some(Command::Cast { spell: after.to_string(), target })
                }
                Err(_) => Some(Command::Cast { spell: rest.to_string(), target: 0 }),
            }
        }
        "drink" if !rest.is_empty() => Some(Command::Drink(rest.to_string())),
        "status" => Some(Command::Status),
        "help" => Some(Command::Help),
        "quit" => Some(Command::Quit),
        _ => None,
    }
}

pub const HELP: &str = "Commands:
  cast <enemy#> <spell>   e.g. `cast 2 frostbolt`
  cast heal               heal yourself
  drink <item>            e.g. `drink mana potion`
  status                  show the battlefield
  help                    this text
  quit                    flee the battle";

// ══════════════════════════════════════════════════════════════════════
//  PHASE 1 — Creatures as trait objects
// ══════════════════════════════════════════════════════════════════════

pub trait Creature {
    fn name(&self) -> &str;
    fn hp(&self) -> u32;
    fn attack_power(&self) -> u32;
    /// Lose `amount` hp, never below 0.
    fn take_damage(&mut self, amount: u32);

    /// The element that hurts this creature double. Most creatures have none.
    fn weakness(&self) -> Option<Element> {
        None
    }

    /// The element that hurts this creature only half. Most creatures have none.
    fn resistance(&self) -> Option<Element> {
        None
    }

    /// A creature is alive while it has more than 0 hp.
    fn is_alive(&self) -> bool {
        todo!()
    }
}

/// Example creature — done for you.
pub struct Goblin {
    pub hp: u32,
}

impl Goblin {
    pub fn new() -> Self {
        Goblin { hp: 30 }
    }
}

impl Creature for Goblin {
    fn name(&self) -> &str {
        "Goblin"
    }
    fn hp(&self) -> u32 {
        self.hp
    }
    fn attack_power(&self) -> u32 {
        5
    }
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
}

/// "Ice Wraith": 50 hp, attacks for 8, weak to Fire, resists Ice.
pub struct IceWraith {
    pub hp: u32,
}

impl IceWraith {
    pub fn new() -> Self {
        IceWraith { hp: 50 }
    }
}

impl Creature for IceWraith {
    fn name(&self) -> &str {
        todo!()
    }
    fn hp(&self) -> u32 {
        todo!()
    }
    fn attack_power(&self) -> u32 {
        todo!()
    }
    fn take_damage(&mut self, amount: u32) {
        todo!()
    }
    // TODO: override `weakness` and `resistance`
}

/// "Dragon": 150 hp, attacks for 12, weak to Ice, resists Fire.
pub struct Dragon {
    pub hp: u32,
}

impl Dragon {
    pub fn new() -> Self {
        Dragon { hp: 150 }
    }
}

// TODO: impl Creature for Dragon (all of it this time!)

/// Hit `target` with a spell of `power` and `element`.
///
/// The damage is doubled against the target's weakness, halved (rounded down)
/// against its resistance, unchanged otherwise. Apply it and return it.
pub fn strike(target: &mut dyn Creature, power: u32, element: Element) -> u32 {
    todo!()
}

// ══════════════════════════════════════════════════════════════════════
//  PHASE 2 — Spellbook & Inventory
// ══════════════════════════════════════════════════════════════════════

/// Everything that can go wrong in battle.
#[derive(Debug, PartialEq)]
pub enum CastError {
    UnknownSpell(String),
    AlreadyKnown(String),
    LevelTooLow { needed: u32, have: u32 },
    NotEnoughMana { needed: u32, have: u32 },
    /// The 0-based index of a target that doesn't exist or is already defeated.
    InvalidTarget(usize),
    OutOfItem(String),
    NotDrinkable(String),
    UnknownCommand(String),
}

pub struct Spellbook {
    spells: Vec<Spell>,
}

impl Spellbook {
    /// A spellbook with the `starter_spells`.
    pub fn new() -> Self {
        Spellbook { spells: starter_spells() }
    }

    /// The spell names, in order.
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for spell in &self.spells {
            names.push(spell.name.clone());
        }
        names
    }

    /// Find a spell by name, ignoring upper/lower case.
    pub fn find(&self, name: &str) -> Option<&Spell> {
        todo!()
    }

    /// Add a spell to the end of the book.
    /// A spell with the same name (ignoring case) → `Err(AlreadyKnown(<name of the new spell>))`.
    pub fn learn(&mut self, spell: Spell) -> Result<(), CastError> {
        todo!()
    }
}

pub struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: HashMap::new() }
    }

    /// Add `qty` of `item` (stacking with what you already have).
    pub fn add(&mut self, item: &str, qty: u32) {
        todo!()
    }

    /// How many of `item` you carry (0 if none).
    pub fn count(&self, item: &str) -> u32 {
        todo!()
    }

    /// Use up one `item`; it disappears from the map when the last one is used.
    /// None left → `Err(OutOfItem(item))`.
    pub fn take(&mut self, item: &str) -> Result<(), CastError> {
        todo!()
    }

    /// "health potion ×1, mana potion ×2" — sorted by name, for the status screen.
    pub fn listing(&self) -> String {
        let mut entries = Vec::new();
        for (item, qty) in &self.items {
            entries.push(format!("{item} ×{qty}"));
        }
        entries.sort();
        entries.join(", ")
    }
}

// ══════════════════════════════════════════════════════════════════════
//  PHASE 3 — Result-based casting
// ══════════════════════════════════════════════════════════════════════

impl fmt::Display for CastError {
    /// The exact messages:
    ///   UnknownSpell(s)            → "Unknown spell '{s}'"
    ///   AlreadyKnown(s)            → "You already know {s}"
    ///   LevelTooLow { needed, have }   → "Requires level {needed}, you are level {have}"
    ///   NotEnoughMana { needed, have } → "Not enough mana: need {needed}, have {have}"
    ///   InvalidTarget(i)           → "There is no living enemy #{i + 1}"   (players count from 1)
    ///   OutOfItem(s)               → "You have no {s} left"
    ///   NotDrinkable(s)            → "You can't drink {s}"
    ///   UnknownCommand(s)          → "Unknown command '{s}' (type 'help')"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for CastError {}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Victory,
    Defeat,
}

pub struct Battle {
    pub mage: Mage,
    pub spellbook: Spellbook,
    pub inventory: Inventory,
    pub enemies: Vec<Box<dyn Creature>>,
}

impl Battle {
    /// Tiberius (level 7) with the starter spells, 2 mana potions and 1 health potion.
    pub fn new(enemies: Vec<Box<dyn Creature>>) -> Self {
        let mut inventory = Inventory::new();
        inventory.add("mana potion", 2);
        inventory.add("health potion", 1);
        Battle { mage: Mage::new("Tiberius", 7), spellbook: Spellbook::new(), inventory, enemies }
    }

    /// The standard encounter: a Goblin, an Ice Wraith and a Dragon.
    pub fn standard() -> Self {
        Battle::new(vec![Box::new(Goblin::new()), Box::new(IceWraith::new()), Box::new(Dragon::new())])
    }

    /// The battlefield as text.
    pub fn status(&self) -> String {
        let mut out = format!(
            "{} — Lv {} | ♥ {}/{} | ✦ {}/{}\nBag: {}\nEnemies:",
            self.mage.name, self.mage.level, self.mage.hp, MAX_HP, self.mage.mana, MAX_MANA,
            self.inventory.listing()
        );
        for (i, enemy) in self.enemies.iter().enumerate() {
            if enemy.is_alive() {
                out.push_str(&format!("\n  {}. {} (♥ {})", i + 1, enemy.name(), enemy.hp()));
            } else {
                out.push_str(&format!("\n  {}. {} (defeated)", i + 1, enemy.name()));
            }
        }
        out
    }

    /// Cast `spell_name` at `self.enemies[target]`. Check, in this order:
    ///
    /// 1. the spell is in the spellbook      → else `UnknownSpell(spell_name)`
    /// 2. the mage's level is high enough    → else `LevelTooLow`
    /// 3. the mage has enough mana           → else `NotEnoughMana`
    /// 4. Holy spells ignore `target`: they heal the mage by `HEAL_AMOUNT`.
    ///    Other spells need a living target  → else `InvalidTarget(target)`
    ///
    /// Only when every check passes is the mana spent (an error costs nothing).
    /// On success return a message naming the spell, e.g.
    /// "Tiberius casts Fireball on Goblin for 45 damage!"
    pub fn cast(&mut self, spell_name: &str, target: usize) -> Result<String, CastError> {
        todo!()
    }

    /// Drink a potion from the inventory:
    ///   "mana potion"   → restore `MANA_POTION` mana
    ///   "health potion" → heal `HEALTH_POTION` hp
    ///   anything else   → `NotDrinkable(item)` (checked first)
    /// Missing potion → the `OutOfItem` error from `Inventory::take` (use `?`).
    pub fn drink(&mut self, item: &str) -> Result<String, CastError> {
        todo!()
    }

    /// Every *living* enemy attacks the mage once (hp never below 0).
    /// Returns the total damage dealt.
    pub fn enemies_turn(&mut self) -> u32 {
        todo!()
    }

    /// `Some(Defeat)` if the mage has 0 hp, `Some(Victory)` if every enemy is
    /// defeated, `None` while the battle goes on. (Defeat is checked first.)
    pub fn outcome(&self) -> Option<Outcome> {
        todo!()
    }

    // ══════════════════════════════════════════════════════════════════
    //  PHASE 4 — The full turn
    // ══════════════════════════════════════════════════════════════════

    /// Handle one line of player input.
    ///
    /// - Unparseable input → `Err(UnknownCommand(<trimmed input>))`  (hint: `ok_or` + `?`)
    /// - `Cast` / `Drink`: perform the action (errors propagate with `?` and the
    ///   enemies do NOT get a turn). On success, if the battle is not over yet,
    ///   the enemies strike back: append a line containing "strike back" and
    ///   the damage dealt to the message.
    /// - `Status` → `self.status()`, `Help` → `HELP`, `Quit` → any farewell message.
    pub fn take_turn(&mut self, input: &str) -> Result<String, CastError> {
        todo!()
    }
}
