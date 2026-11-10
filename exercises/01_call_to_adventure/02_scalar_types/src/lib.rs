// ⚔️ Quest 1.2 — Scalar Types: The Elemental Runes
// 📖 Read: book/src/01_call_to_adventure/02_scalar_types.md
//
// Replace each `todo!()` with real code. Mind the overflows!

/// Mana stored in a `u8` (0..=255). Casting a spell must never underflow:
/// if the cost is higher than the mana you have, you end up at 0.
pub fn drain_mana(mana: u8, cost: u8) -> u8 {
    todo!()
}

/// Ancient runestones can only hold damage values up to 255 (a `u8`).
/// Anything above 255 is stored as 255 — NOT wrapped around.
pub fn engrave_damage(damage: u32) -> u8 {
    todo!()
}

/// Return the mana as a percentage of max mana, e.g. 25 of 200 → 12.5
pub fn mana_percent(current: u32, max: u32) -> f64 {
    todo!()
}

/// Every rune (char) has a numeric value: its Unicode code point.
/// 'A' → 65, 'ᚱ' → 5809
pub fn rune_value(rune: char) -> u32 {
    todo!()
}

/// Gold is split evenly between mages. Return how much each mage gets.
/// (Integer division — leftover coins go to the guild.)
pub fn gold_per_mage(gold: u32, mages: u32) -> u32 {
    todo!()
}

/// ...and return the leftover coins the guild keeps.
pub fn guild_share(gold: u32, mages: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draining_mana_normally() {
        assert_eq!(drain_mana(100, 30), 70);
    }

    #[test]
    fn draining_more_than_you_have_leaves_zero() {
        assert_eq!(drain_mana(10, 30), 0, "💀 Underflow! Mana can't go below zero.");
    }

    #[test]
    fn small_damage_is_engraved_as_is() {
        assert_eq!(engrave_damage(200), 200);
    }

    #[test]
    fn huge_damage_is_capped_not_wrapped() {
        assert_eq!(engrave_damage(300), 255, "🌀 300 as u8 wraps to 44 — cap it instead!");
    }

    #[test]
    fn mana_percentage_uses_floats() {
        assert_eq!(mana_percent(25, 200), 12.5);
        assert_eq!(mana_percent(200, 200), 100.0);
    }

    #[test]
    fn runes_have_values() {
        assert_eq!(rune_value('A'), 65);
        assert_eq!(rune_value('ᚱ'), 5809);
    }

    #[test]
    fn gold_is_split_with_integer_division() {
        assert_eq!(gold_per_mage(100, 3), 33);
        assert_eq!(guild_share(100, 3), 1);
    }
}
