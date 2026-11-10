// ⚔️ Quest 1.4 — Functions: The Incantation Forms
// 📖 Read: book/src/01_call_to_adventure/04_functions.md
//
// ❌ This file does not compile yet. One function is fine,
// one has a bug, and one doesn't exist at all.

/// Spell damage = base damage + 5 per mage level.
pub fn spell_damage(base: u32, level: u32) -> u32 {
    base + (level * 5)
}

/// Returns the mana left after casting.
/// The compiler complains: "expected `u32`, found `()`". Why?
pub fn mana_after_cast(mana: u32, cost: u32) -> u32 {
    mana.saturating_sub(cost)
}

// TODO: write a function `describe_spell` that takes a spell name (`&str`)
// and a cost (`u32`) and returns a `String` like:
//     "Fireball costs 30 mana"
// Hint: format!() works like println!() but returns a String.
pub fn describe_spell(name: &str, cost: u32) -> String {
    format!("{name} costs {cost} mana")
}

/// Uses a block expression: the value of a `{ }` block is its last expression.
/// Return (base * 2) + 1, computed inside a block.
pub fn empowered(base: u32) -> u32 {
    let result = {
        let doubled = base * 2;
        doubled + 1
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_scales_with_level() {
        assert_eq!(spell_damage(45, 7), 80);
    }

    #[test]
    fn casting_costs_mana() {
        assert_eq!(mana_after_cast(100, 30), 70);
    }

    #[test]
    fn spells_can_be_described() {
        assert_eq!(describe_spell("Fireball", 30), "Fireball costs 30 mana");
        assert_eq!(describe_spell("Heal", 20), "Heal costs 20 mana");
    }

    #[test]
    fn block_expressions_have_values() {
        assert_eq!(empowered(10), 21);
    }
}
