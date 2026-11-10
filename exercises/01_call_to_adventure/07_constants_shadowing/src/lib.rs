// ⚔️ Quest 1.7 — Constants & Shadowing: The Unchanging Laws
// 📖 Read: book/src/01_call_to_adventure/07_constants_shadowing.md
//
// ❌ This file does not compile yet: constants are picky.

/// The maximum mana any mage can hold.
pub const MAX_MANA: u32 = 100;

/// Fill a mana pool, but never beyond MAX_MANA.
pub fn fill(mana: u32, amount: u32) -> u32 {
    mana.saturating_add(amount).min(MAX_MANA)
}

/// A mage types their mana into the terminal, e.g. "  42\n".
/// Use *shadowing* to turn the text into a number:
///     let input = input.trim();
///     let input: u32 = ...;
/// If the text is not a number, return 0.
/// Hint: `.parse()` returns a Result; `.unwrap_or(0)` gives a default.
pub fn parse_mana(input: &str) -> u32 {
    input.trim().parse().unwrap_or(0u32)
}

/// Shadowing can even change the type. Take the spell name, and shadow it
/// with its length, then return that length doubled.
pub fn name_power(spell: &str) -> usize {
    spell.len() * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_mana_is_one_hundred() {
        assert_eq!(MAX_MANA, 100u32);
    }

    #[test]
    fn filling_caps_at_max() {
        assert_eq!(fill(50, 30), 80);
        assert_eq!(fill(90, 30), 100);
    }

    #[test]
    fn parsing_typed_mana() {
        assert_eq!(parse_mana("  42\n"), 42);
        assert_eq!(parse_mana("100"), 100);
        assert_eq!(parse_mana("lots"), 0);
    }

    #[test]
    fn shadowing_changes_type() {
        assert_eq!(name_power("Fireball"), 16);
    }
}
