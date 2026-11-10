// ⚔️ Quest 1.5 — Control Flow: The Forking Paths of Fate
// 📖 Read: book/src/01_call_to_adventure/05_if_else.md

/// Classify a spell by its power:
///   0        → "utility"
///   1..=29   → "minor"
///   30..=59  → "major"
///   60+      → "legendary"
pub fn spell_tier(power: u32) -> &'static str {
    todo!()
}

/// A mage can cast a spell when they have enough mana AND are high enough level.
pub fn can_cast(mana: u32, cost: u32, level: u32, required_level: u32) -> bool {
    todo!()
}

/// `if` is an expression! Use `let aura = if ... { ... } else { ... };`
/// Mana above 50 → "blue", otherwise → "grey".
/// Return a sentence like: "Your aura glows blue"
pub fn aura(mana: u32) -> String {
    todo!()
}

/// The Rune Gate opens on multiples of 3 ("Fizz"), 5 ("Buzz") or both ("FizzBuzz").
/// Otherwise the gate whispers the number back.
pub fn rune_gate(n: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spell_tiers() {
        assert_eq!(spell_tier(0), "utility");
        assert_eq!(spell_tier(1), "minor");
        assert_eq!(spell_tier(29), "minor");
        assert_eq!(spell_tier(30), "major");
        assert_eq!(spell_tier(59), "major");
        assert_eq!(spell_tier(60), "legendary");
        assert_eq!(spell_tier(9000), "legendary");
    }

    #[test]
    fn casting_requires_mana_and_level() {
        assert!(can_cast(100, 30, 7, 5));
        assert!(!can_cast(20, 30, 7, 5), "Not enough mana");
        assert!(!can_cast(100, 30, 3, 5), "Level too low");
        assert!(can_cast(30, 30, 5, 5), "Exactly enough is enough");
    }

    #[test]
    fn auras() {
        assert_eq!(aura(51), "Your aura glows blue");
        assert_eq!(aura(50), "Your aura glows grey");
    }

    #[test]
    fn the_rune_gate() {
        assert_eq!(rune_gate(3), "Fizz");
        assert_eq!(rune_gate(10), "Buzz");
        assert_eq!(rune_gate(15), "FizzBuzz");
        assert_eq!(rune_gate(7), "7");
    }
}
