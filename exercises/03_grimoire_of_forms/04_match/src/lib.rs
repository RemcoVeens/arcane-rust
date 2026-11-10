// ⚔️ Quest 3.4 — Match: The Art of Divination
// 📖 Read: book/src/03_grimoire_of_forms/04_match.md
//
// Replace each `todo!()` with a `match`. Every future must be foreseen!

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
}

/// Damage multiplier when `attack` hits a defender of element `defender`.
/// Fire beats Frost, Frost beats Arcane, Arcane beats Fire:
///   strong pairs   → 2.0   (e.g. Fire vs Frost)
///   reversed pairs → 0.5   (e.g. Frost vs Fire)
///   same element   → 1.0
/// Hint: match on the tuple `(attack, defender)`.
pub fn multiplier(attack: Element, defender: Element) -> f64 {
    todo!()
}

/// Titles by level:
///   0        → "Unawakened"
///   1..=4    → "Apprentice"
///   5..=9    → "Adept"
///   10..=19  → "Archmage"
///   20 and up → "Legend"
pub fn title(level: u32) -> &'static str {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Creature {
    Slime,
    Goblin { hp: u32 },
    Wraith { hp: u32, element: Element },
    Dragon { hp: u32, element: Element, flying: bool },
}

/// How scared should you be?
///   Slime                         → "harmless"
///   Goblin with less than 20 hp   → "weak"
///   any other Goblin              → "annoying"
///   Wraith of Arcane              → "deadly"
///   any other Wraith              → "dangerous"
///   flying Dragon                 → "run"
///   any other Dragon              → "deadly"
/// Hint: match guards (`if ...`) and nested patterns.
pub fn threat(creature: &Creature) -> &'static str {
    todo!()
}

/// The loot message. Bind the matched value with `n @ range`:
///   0        → "Nothing."
///   1..=9    → "A few coins (n)."
///   10..=99  → "A pouch of n gold."
///   100+     → "A dragon's hoard: n gold!"
pub fn loot_message(gold: u32) -> String {
    todo!()
}

/// Is this a boss? Dragons always are; Wraiths with more than 100 hp too.
/// Hint: the `matches!` macro.
pub fn is_boss(creature: &Creature) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Element::*;

    #[test]
    fn elemental_multipliers() {
        assert_eq!(multiplier(Fire, Frost), 2.0);
        assert_eq!(multiplier(Frost, Arcane), 2.0);
        assert_eq!(multiplier(Arcane, Fire), 2.0);
        assert_eq!(multiplier(Frost, Fire), 0.5);
        assert_eq!(multiplier(Fire, Arcane), 0.5);
        assert_eq!(multiplier(Arcane, Arcane), 1.0);
    }

    #[test]
    fn titles_by_level() {
        assert_eq!(title(0), "Unawakened");
        assert_eq!(title(1), "Apprentice");
        assert_eq!(title(7), "Adept");
        assert_eq!(title(10), "Archmage");
        assert_eq!(title(19), "Archmage");
        assert_eq!(title(250), "Legend");
    }

    #[test]
    fn threat_levels() {
        assert_eq!(threat(&Creature::Slime), "harmless");
        assert_eq!(threat(&Creature::Goblin { hp: 12 }), "weak");
        assert_eq!(threat(&Creature::Goblin { hp: 20 }), "annoying");
        assert_eq!(threat(&Creature::Wraith { hp: 50, element: Arcane }), "deadly");
        assert_eq!(threat(&Creature::Wraith { hp: 50, element: Frost }), "dangerous");
        assert_eq!(threat(&Creature::Dragon { hp: 500, element: Fire, flying: true }), "run");
        assert_eq!(threat(&Creature::Dragon { hp: 500, element: Fire, flying: false }), "deadly");
    }

    #[test]
    fn loot_messages() {
        assert_eq!(loot_message(0), "Nothing.");
        assert_eq!(loot_message(7), "A few coins (7).");
        assert_eq!(loot_message(42), "A pouch of 42 gold.");
        assert_eq!(loot_message(1000), "A dragon's hoard: 1000 gold!");
    }

    #[test]
    fn spotting_bosses() {
        assert!(is_boss(&Creature::Dragon { hp: 1, element: Frost, flying: false }));
        assert!(is_boss(&Creature::Wraith { hp: 150, element: Fire }));
        assert!(!is_boss(&Creature::Wraith { hp: 100, element: Fire }));
        assert!(!is_boss(&Creature::Goblin { hp: 999 }));
    }
}
