// ⚔️ Quest 4.6 — Custom Errors: Forging Your Own Failure Modes
// 📖 Read: book/src/04_forge_of_resilience/06_custom_errors.md
//
// ❌ This file does not compile yet: `SpellError` is not a real error type.
// 1. fill in `Display`   2. add `impl std::error::Error`   3. fill in `From`
// 4. implement `parse_mana` and `cast`

use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum SpellError {
    UnknownSpell(String),
    NotEnoughMana { needed: u32, available: u32 },
    BadNumber(ParseIntError),
}

impl fmt::Display for SpellError {
    /// The messages, exactly:
    ///   UnknownSpell("Pizza")                  → "unknown spell 'Pizza'"
    ///   NotEnoughMana { needed: 90, available: 10 } → "not enough mana: need 90, have 10"
    ///   BadNumber(e)                           → "bad number: {e}"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

// TODO: make `SpellError` a proper `std::error::Error`.
//       Bonus (tested!): `BadNumber` should report its inner error as `source()`.

impl From<ParseIntError> for SpellError {
    /// Lets `?` turn a `ParseIntError` into a `SpellError::BadNumber` automatically.
    fn from(err: ParseIntError) -> Self {
        todo!()
    }
}

/// The mana cost of each known spell (case-sensitive).
pub fn spell_cost(spell: &str) -> Option<u32> {
    match spell {
        "Fireball" => Some(30),
        "Frostbolt" => Some(25),
        "Arcane Blast" => Some(40),
        "Heal" => Some(20),
        "Meteor" => Some(90),
        _ => None,
    }
}

/// Parse a (trimmed) mana amount. Use `?` — your `From` impl does the conversion.
pub fn parse_mana(input: &str) -> Result<u32, SpellError> {
    todo!()
}

/// Cast `spell` with the mana the player typed. Returns the remaining mana.
/// Check in this order: mana parses → spell is known → enough mana.
pub fn cast(spell: &str, mana_input: &str) -> Result<u32, SpellError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn casting_works() {
        assert_eq!(cast("Fireball", "100"), Ok(70));
    }

    #[test]
    fn unknown_spells() {
        let err = cast("Pizza", "100").unwrap_err();
        assert_eq!(err, SpellError::UnknownSpell("Pizza".to_string()));
        assert_eq!(err.to_string(), "unknown spell 'Pizza'");
    }

    #[test]
    fn not_enough_mana() {
        let err = cast("Meteor", "10").unwrap_err();
        assert_eq!(err, SpellError::NotEnoughMana { needed: 90, available: 10 });
        assert_eq!(err.to_string(), "not enough mana: need 90, have 10");
    }

    #[test]
    fn bad_numbers_convert_with_question_mark() {
        let err = parse_mana("lots").unwrap_err();
        assert!(matches!(err, SpellError::BadNumber(_)), "got {err:?}");
        assert_eq!(err.to_string(), "bad number: invalid digit found in string");
    }

    #[test]
    fn mana_is_checked_before_the_spell() {
        let err = cast("Pizza", "lots").unwrap_err();
        assert!(matches!(err, SpellError::BadNumber(_)), "got {err:?}");
    }

    #[test]
    fn it_is_a_real_error() {
        // A `Box<dyn Error>` only accepts real `std::error::Error` types.
        let boxed: Box<dyn Error> = Box::new(SpellError::UnknownSpell("Pizza".to_string()));
        assert_eq!(boxed.to_string(), "unknown spell 'Pizza'");
    }

    #[test]
    fn bad_numbers_keep_their_source() {
        let err = parse_mana("lots").unwrap_err();
        assert!(err.source().is_some(), "🔗 BadNumber should expose the ParseIntError as source()");
        let err = SpellError::UnknownSpell("Pizza".to_string());
        assert!(err.source().is_none());
    }
}
