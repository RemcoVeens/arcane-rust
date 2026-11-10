// ⚔️ Quest 4.4 — Result: The Fork in the Road
// 📖 Read: book/src/04_forge_of_resilience/04_result.md
//
// Replace each `todo!()` with real code. No `unwrap()` allowed in this quest —
// handle every `Err` with `match` (or `if let` / `let ... else`).

/// Cast a spell costing `cost` from `mana`.
///
/// - Enough mana → `Ok(remaining mana)`
/// - Too little  → `Err("Not enough mana: need {cost}, have {mana}")`
pub fn cast(mana: u32, cost: u32) -> Result<u32, String> {
    todo!()
}

/// Parse the mana value a player typed, e.g. `" 42\n"` → `Ok(42)`.
/// Surrounding whitespace is ignored.
/// Anything that is not a valid `u32` → `Err("'{input}' is not a mana amount")`
/// where `{input}` is the *trimmed* input.
pub fn parse_mana(input: &str) -> Result<u32, String> {
    todo!()
}

/// Potions from the alchemist:
///   "healing" → Ok(30), "mana" → Ok(50), anything else → Err("Unknown potion: {name}")
pub fn potion_strength(name: &str) -> Result<u32, String> {
    todo!()
}

/// Cast a chain of spells, one after the other, starting with `mana`.
/// Stop at the FIRST spell that fails and return that error.
/// If all succeed, return the mana that is left.
pub fn cast_chain(mana: u32, costs: &[u32]) -> Result<u32, String> {
    todo!()
}

/// Try to drink a potion; a failed attempt simply restores nothing.
/// Return the mana afterwards (no cap in this quest).
/// Hint: `unwrap_or` is the polite cousin of `unwrap`.
pub fn drink_or_nothing(mana: u32, potion: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casting_with_enough_mana() {
        assert_eq!(cast(100, 30), Ok(70));
        assert_eq!(cast(30, 30), Ok(0), "exactly enough is enough");
    }

    #[test]
    fn casting_without_enough_mana() {
        assert_eq!(cast(10, 30), Err("Not enough mana: need 30, have 10".to_string()));
    }

    #[test]
    fn parsing_mana() {
        assert_eq!(parse_mana(" 42\n"), Ok(42));
        assert_eq!(parse_mana("lots"), Err("'lots' is not a mana amount".to_string()));
        assert_eq!(parse_mana("-5"), Err("'-5' is not a mana amount".to_string()));
    }

    #[test]
    fn potions() {
        assert_eq!(potion_strength("healing"), Ok(30));
        assert_eq!(potion_strength("mana"), Ok(50));
        assert_eq!(potion_strength("glitter"), Err("Unknown potion: glitter".to_string()));
    }

    #[test]
    fn a_chain_that_succeeds() {
        assert_eq!(cast_chain(100, &[30, 25, 40]), Ok(5));
    }

    #[test]
    fn a_chain_stops_at_the_first_failure() {
        assert_eq!(
            cast_chain(100, &[30, 80, 5]),
            Err("Not enough mana: need 80, have 70".to_string()),
            "⛓️ the chain must break at the second spell"
        );
    }

    #[test]
    fn drinking_unknown_potions_does_nothing() {
        assert_eq!(drink_or_nothing(10, "mana"), 60);
        assert_eq!(drink_or_nothing(10, "glitter"), 10);
    }
}
