// ⚔️ Quest 4.5 — The `?` Operator: The Path of Least Resistance
// 📖 Read: book/src/04_forge_of_resilience/05_question_mark.md
//
// Replace each `todo!()`. Every function here can be written WITHOUT `match`:
// let `?` do the propagating.

use std::error::Error;
use std::num::ParseIntError;

/// Parse a mana cost like `" 30 "`. The error type is the one `str::parse` gives you.
pub fn parse_cost(input: &str) -> Result<u32, ParseIntError> {
    todo!()
}

/// Parse two costs and add them: `combined_cost("30", "25")` → `Ok(55)`.
/// If either fails to parse, return that error. Use `parse_cost` and `?`.
pub fn combined_cost(a: &str, b: &str) -> Result<u32, ParseIntError> {
    todo!()
}

/// The first word of an incantation, upper-cased: `"ignis magna"` → `Some("IGNIS")`.
/// No words at all → `None`. Use `?` on an `Option`!
pub fn first_word_shouted(incantation: &str) -> Option<String> {
    todo!()
}

/// A spell line has the form `"<name> <cost> <power>"`, e.g. `"Fireball 30 45"`.
/// Return how much power you get per point of mana, rounded down (45 / 30 → 1).
///
/// Return `None` if a part is missing, a number does not parse, or the cost is 0.
/// Hints: `.split_whitespace()`, `.next()?`, `.parse().ok()?`, `.checked_div()?`
pub fn power_per_mana(line: &str) -> Option<u32> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub power: u32,
}

/// Parse a spell line `"<name> <cost> <power>"` into a `Spell`.
///
/// `Box<dyn Error>` can hold *any* error, and `?` converts into it automatically:
/// - a missing part → use `.ok_or("missing cost")?` (a `&str` converts too!)
///   the messages must be "missing name", "missing cost" and "missing power"
/// - a bad number → just `?` the `ParseIntError`
pub fn parse_spell(line: &str) -> Result<Spell, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_costs() {
        assert_eq!(parse_cost(" 30 "), Ok(30));
        assert!(parse_cost("thirty").is_err());
    }

    #[test]
    fn combining_costs() {
        assert_eq!(combined_cost("30", "25"), Ok(55));
        assert!(combined_cost("30", "lots").is_err(), "the second error must propagate");
        assert!(combined_cost("lots", "30").is_err(), "the first error must propagate");
    }

    #[test]
    fn shouting_the_first_word() {
        assert_eq!(first_word_shouted("ignis magna"), Some("IGNIS".to_string()));
        assert_eq!(first_word_shouted("   "), None);
    }

    #[test]
    fn power_per_mana_for_good_lines() {
        assert_eq!(power_per_mana("Fireball 30 45"), Some(1));
        assert_eq!(power_per_mana("Meteor 90 150"), Some(1));
        assert_eq!(power_per_mana("Spark 5 40"), Some(8));
    }

    #[test]
    fn power_per_mana_for_broken_lines() {
        assert_eq!(power_per_mana("Fireball 30"), None, "missing power");
        assert_eq!(power_per_mana("Fireball thirty 45"), None, "bad number");
        assert_eq!(power_per_mana("Freebie 0 45"), None, "➗ division by zero");
    }

    #[test]
    fn parsing_a_whole_spell() {
        let spell = parse_spell("Frostbolt 25 35").expect("this line is valid");
        assert_eq!(
            spell,
            Spell { name: "Frostbolt".to_string(), cost: 25, power: 35 }
        );
    }

    #[test]
    fn missing_parts_are_named() {
        assert_eq!(parse_spell("").unwrap_err().to_string(), "missing name");
        assert_eq!(parse_spell("Heal").unwrap_err().to_string(), "missing cost");
        assert_eq!(parse_spell("Heal 20").unwrap_err().to_string(), "missing power");
    }

    #[test]
    fn bad_numbers_propagate() {
        let err = parse_spell("Heal 20 lots").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }
}
