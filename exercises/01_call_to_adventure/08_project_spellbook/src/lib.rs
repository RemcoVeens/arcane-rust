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
    format!(
        "╔══════════════════════════════════╗\r\n║      ARCANE GRIMOIRE v0.1        ║\r\n╚══════════════════════════════════╝\r\n"
    )
}

/// Find a spell by name, ignoring upper/lower case.
/// `Option` is a box that holds either `Some(spell)` or `None`.
/// (Chapter 3 explains it fully — for now: `return Some(spell);` or `None`.)
pub fn find_spell(name: &str) -> Option<Spell> {
    for spell in SPELLS {
        if name.to_lowercase() == spell.0.to_lowercase() {
            return Some(spell);
        }
    }
    None
}

/// One line per spell, numbered from 1:
///   "1. Fireball — mana 30, power 45, level 1"
/// Lines are separated by '\n'.
pub fn spell_list() -> String {
    let mut res = "".to_string();
    for (i, spell) in SPELLS.iter().enumerate() {
        let lc = i + 1;
        let (name, mana, power, level) = spell;
        res = format!("{res}{lc}. {name} — mana {mana}, power {power}, level {level}\r\n")
    }
    res
}

/// Drinking a potion restores POTION_STRENGTH mana, capped at MAX_MANA.
pub fn drink_potion(mana: u32) -> u32 {
    (mana + POTION_STRENGTH).min(MAX_MANA)
}

/// Try to cast a spell. Returns (mana afterwards, message).
///
/// - Unknown spell:   mana unchanged, message contains "Unknown spell"
/// - Level too low:   mana unchanged, message contains "level"  (e.g. "Requires level 10")
/// - Too little mana: mana unchanged, message contains "Not enough mana"
/// - Success:         mana reduced by cost, message contains the spell's
///                    proper name and its power, e.g. "You cast Fireball! (45 damage)"
pub fn cast(name: &str, mana: u32, level: u32) -> (u32, String) {
    let spell = find_spell(name);
    let Some((spell_name, spell_mana, spell_power, spell_level)) = spell else {
        return (mana, "Unknown spell: {name}".to_string());
    };
    if spell_level > level {
        return (
            mana,
            "Requires level {spell_level}, current level {level}".to_string(),
        );
    } else if spell_mana > mana {
        return (mana, "Not enough mana. current {mana}".to_string());
    } else {
        return (
            (mana - spell_mana),
            format!("You cast {spell_name}! ({spell_power} damage)"),
        );
    }
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
    let iinput = input.trim().to_lowercase();
    if iinput == "list" {
        return (mana, spell_list(), true);
    } else if iinput == "rest" {
        return (MAX_MANA, "you are rested".to_string(), true);
    } else if iinput == "potion" {
        return (drink_potion(mana), "you drank a potion".to_string(), true);
    } else if iinput.contains("cast") {
        let spell_name = iinput.trim_start_matches("cast ").trim();
        let (return_mana, msg) = cast(spell_name, mana, level);
        (return_mana, msg, true)
    } else if iinput == "help" {
        let message = "list: lists all spells\r\nrest: rest to restore health\r\npotion: drink a potion\r\ncast <spell>: cast a spell\r\nhelp: print this\r\nquit: close this shit\r\n";
        (mana, message.to_string(), true)
    } else if iinput == "quit" {
        (mana, "bye bye".to_string(), false)
    } else {
        return (mana, "Unknown command".to_string(), true);
    }
}
