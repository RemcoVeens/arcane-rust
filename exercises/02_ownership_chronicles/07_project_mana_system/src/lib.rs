// 🏗️ Project 2 — The Mana System
// 📖 Read: book/src/02_ownership_chronicles/07_project_mana_system.md
//
// Below you find the finished Chapter 1 spellbook (module `chapter1`).
// It works, but it passes everything around *by value*: mana goes in and
// comes back out of every function, and the spell list is a fixed global.
//
// Your job: rebuild it with ownership tools — `&str` parameters, `&mut`
// mana, slices of spells, and a Vec of scrolls that *moves* new spells
// into your spellbook. Implement the functions marked `todo!()`, then play:
//
//     cargo run -p ch02_07_project_mana_system
//
// The tests in `tests/mana_system.rs` check your work, phase by phase.

pub const MAX_MANA: u32 = 100;
pub const POTION_STRENGTH: u32 = 50;

/// (name, mana cost, power, required level)
pub type Spell = (&'static str, u32, u32, u32);

/// The spells Tiberius knows when the game starts.
pub const STARTING_SPELLS: [Spell; 3] = [
    ("Fireball", 30, 45, 1),
    ("Frostbolt", 25, 35, 2),
    ("Heal", 20, 0, 1),
];

/// The scrolls hidden in the tower. Reading one teaches its spell.
pub const TOWER_SCROLLS: [Spell; 2] = [("Arcane Blast", 40, 60, 5), ("Meteor", 90, 150, 10)];

// ── Phase 1: reading through borrows ────────────────────────────────

/// Split a line of input into (command, argument) — both are *slices of
/// the input*, no new Strings. Surrounding whitespace is ignored, and so
/// is whitespace around the argument.
///
///   "  cast   Arcane Blast \n" → ("cast", "Arcane Blast")
///   "list\n"                   → ("list", "")
pub fn split_command(input: &str) -> (&str, &str) {
    todo!()
}

/// One line per spell, numbered from 1, separated by '\n':
///   "1. Fireball — mana 30, power 45, level 1"
/// An empty spellbook gives "Your spellbook is empty."
pub fn spell_list(spellbook: &[Spell]) -> String {
    todo!()
}

/// Find a spell in the (borrowed) spellbook by name, ignoring case.
/// Spells are Copy tuples, so you can return a copy.
pub fn find_spell(spellbook: &[Spell], name: &str) -> Option<Spell> {
    todo!()
}

// ── Phase 2: mana through `&mut` ────────────────────────────────────

/// Fill the mana pool to MAX_MANA.
pub fn rest(mana: &mut u32) {
    todo!()
}

/// Restore POTION_STRENGTH mana, capped at MAX_MANA.
pub fn drink_potion(mana: &mut u32) {
    todo!()
}

/// Spend `cost` mana if there is enough. Returns whether it worked;
/// on failure the mana is left untouched.
pub fn spend_mana(mana: &mut u32, cost: u32) -> bool {
    todo!()
}

/// Try to cast a spell from the spellbook. Returns the message; mana is
/// only reduced when the spell is actually cast.
///
/// - Unknown spell:   message contains "Unknown spell"
/// - Level too low:   message contains "level"  (e.g. "Requires level 10")
/// - Too little mana: message contains "Not enough mana"
/// - Success:         message contains the spell's proper name and its
///   power, e.g. "You cast Fireball! (45 damage)"
pub fn cast(spellbook: &[Spell], name: &str, mana: &mut u32, level: u32) -> String {
    todo!()
}

// ── Phase 3: scrolls & commands ─────────────────────────────────────

/// Read the first scroll: it is *removed* from `scrolls` and *moved* into
/// the `spellbook` (at the end). Message: "You learn Arcane Blast!"
/// (it must contain the spell name). No scrolls left → message contains
/// "No scrolls".
pub fn read_scroll(scrolls: &mut Vec<Spell>, spellbook: &mut Vec<Spell>) -> String {
    todo!()
}

/// Handle one line of player input. Returns (message, keep_playing).
///
///   "list"          → spell_list(spellbook)
///   "rest"          → rest
///   "potion"        → drink_potion
///   "cast <spell>"  → cast(...)
///   "read"          → read_scroll
///   "help"          → text that mentions every command
///   "quit"          → keep_playing = false
///   anything else   → message contains "Unknown command"
///
/// The command may be in any capitalisation ("CAST fireball").
/// The message for rest/potion should mention the new mana.
pub fn handle(
    input: &str,
    mana: &mut u32,
    level: u32,
    spellbook: &mut Vec<Spell>,
    scrolls: &mut Vec<Spell>,
) -> (String, bool) {
    todo!()
}

/// 📜 The Chapter 1 spellbook — working, but everything by value.
/// Use it as a reference while you rebuild. Delete it when you're done
/// (nothing else uses it).
pub mod chapter1 {
    pub const MAX_MANA: u32 = 100;
    pub const POTION_STRENGTH: u32 = 50;

    pub type Spell = (&'static str, u32, u32, u32);

    pub const SPELLS: [Spell; 5] = [
        ("Fireball", 30, 45, 1),
        ("Frostbolt", 25, 35, 2),
        ("Arcane Blast", 40, 60, 5),
        ("Heal", 20, 0, 1),
        ("Meteor", 90, 150, 10),
    ];

    pub fn find_spell(name: &str) -> Option<Spell> {
        for spell in SPELLS {
            if spell.0.to_lowercase() == name.to_lowercase() {
                return Some(spell);
            }
        }
        None
    }

    pub fn spell_list() -> String {
        let mut list = String::new();
        for (i, (name, cost, power, level)) in SPELLS.iter().enumerate() {
            if i > 0 {
                list.push('\n');
            }
            list += &format!("{}. {name} — mana {cost}, power {power}, level {level}", i + 1);
        }
        list
    }

    pub fn drink_potion(mana: u32) -> u32 {
        (mana + POTION_STRENGTH).min(MAX_MANA)
    }

    pub fn cast(name: &str, mana: u32, level: u32) -> (u32, String) {
        for (spell_name, cost, power, required) in SPELLS {
            if spell_name.to_lowercase() == name.to_lowercase() {
                return if level < required {
                    (mana, format!("{spell_name} requires level {required}."))
                } else if mana < cost {
                    (mana, format!("Not enough mana! {spell_name} needs {cost}, you have {mana}."))
                } else {
                    (mana - cost, format!("You cast {spell_name}! ({power} damage)"))
                };
            }
        }
        (mana, format!("Unknown spell: '{name}'"))
    }

    pub fn handle(input: &str, mana: u32, level: u32) -> (u32, String, bool) {
        let input = input.trim().to_lowercase();
        if input == "list" {
            (mana, spell_list(), true)
        } else if input == "rest" {
            (MAX_MANA, String::from("You rest. Mana fully restored."), true)
        } else if input == "potion" {
            let mana = drink_potion(mana);
            (mana, format!("Glug glug. Mana is now {mana}."), true)
        } else if input.starts_with("cast ") {
            let (mana, message) = cast(input["cast ".len()..].trim(), mana, level);
            (mana, message, true)
        } else if input == "help" {
            let help = "Commands: list, cast <spell>, rest, potion, help, quit";
            (mana, String::from(help), true)
        } else if input == "quit" {
            (mana, String::from("Farewell, mage."), false)
        } else {
            (mana, format!("Unknown command: '{input}'. Type 'help'."), true)
        }
    }
}
