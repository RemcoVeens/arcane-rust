// 🏗️ Project 1 — The Spellbook CLI
// 📖 Read: book/src/01_call_to_adventure/08_project_spellbook.md
//
// `src/main.rs` (already written) is the interactive game loop.
// It calls the functions below. Implement them, then play:
//
//     cargo run -p ch01_08_project_spellbook
//
// The tests in `tests/spellbook.rs` check your work.

pub const MAX_MANA: u32 = 100;
pub const POTION_STRENGTH: u32 = 50;

/// (name, mana cost, power, required level)
pub type Spell = (&'static str, u32, u32, u32);

pub const SPELLS: [Spell; 5] = [
    ("Fireball", 30, 45, 1),
    ("Frostbolt", 25, 35, 2),
    ("Arcane Blast", 40, 60, 5),
    ("Heal", 20, 0, 1),
    ("Meteor", 90, 150, 10),
];

/// The banner shown at startup. Must contain "ARCANE GRIMOIRE".
pub fn banner() -> String {
    todo!()
}

/// Find a spell by name, ignoring upper/lower case.
/// `Option` is a box that holds either `Some(spell)` or `None`.
/// (Chapter 3 explains it fully — for now: `return Some(spell);` or `None`.)
pub fn find_spell(name: &str) -> Option<Spell> {
    todo!()
}

/// One line per spell, numbered from 1:
///   "1. Fireball — mana 30, power 45, level 1"
/// Lines are separated by '\n'.
pub fn spell_list() -> String {
    todo!()
}

/// Drinking a potion restores POTION_STRENGTH mana, capped at MAX_MANA.
pub fn drink_potion(mana: u32) -> u32 {
    todo!()
}

/// Try to cast a spell. Returns (mana afterwards, message).
///
/// - Unknown spell:   mana unchanged, message contains "Unknown spell"
/// - Level too low:   mana unchanged, message contains "level"  (e.g. "Requires level 10")
/// - Too little mana: mana unchanged, message contains "Not enough mana"
/// - Success:         mana reduced by cost, message contains the spell's
///                    proper name and its power, e.g. "You cast Fireball! (45 damage)"
pub fn cast(name: &str, mana: u32, level: u32) -> (u32, String) {
    todo!()
}

/// Handle one line of player input. Returns (new mana, message, keep_playing).
///
///   "list"          → spell_list()
///   "rest"          → mana becomes MAX_MANA
///   "potion"        → drink_potion
///   "cast <spell>"  → cast(...)
///   "help"          → text that mentions every command
///   "quit"          → keep_playing = false
///   anything else   → message contains "Unknown command"
///
/// Input may have surrounding whitespace and any capitalisation of the command.
pub fn handle(input: &str, mana: u32, level: u32) -> (u32, String, bool) {
    todo!()
}
