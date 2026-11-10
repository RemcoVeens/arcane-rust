// ⚔️ Quest 4.8 — Traits: The Schools of Magic
// 📖 Read: book/src/04_forge_of_resilience/08_traits.md
//
// ❌ This file does not compile yet: the tests use trait impls and derives
// that don't exist. Follow the TODOs from top to bottom.

use std::fmt;

/// Everything magical shares this behaviour.
pub trait Enchantment {
    fn name(&self) -> String;
    fn power(&self) -> u32;

    /// Default description: `"{name} ({power} power)"`, e.g. `"Fireball (45 power)"`.
    fn describe(&self) -> String {
        todo!()
    }
}

pub struct Fireball;

pub struct Frostbolt {
    /// Extra power from a colder winter.
    pub chill: u32,
}

// TODO: derive what the tests need: they print, compare and copy runes.
pub struct Rune {
    pub glyph: char,
    pub strength: u32,
}

// TODO: impl Enchantment for Fireball   → name "Fireball", power 45, default describe
// TODO: impl Enchantment for Frostbolt  → name "Frostbolt", power 35 + chill, default describe
// TODO: impl Enchantment for Rune       → name "Rune {glyph}", power strength × 10,
//       and OVERRIDE describe: "{glyph} glows with {power} power"

// TODO: impl fmt::Display for Rune so that `Rune { glyph: 'ᚱ', strength: 3 }`
//       prints as `[ᚱ:3]`

/// Announce any enchantment: `"✨ " + describe()`.
pub fn announce(e: &impl Enchantment) -> String {
    todo!()
}

/// The total power of a slice of enchantments of the *same* type.
pub fn total_power<T: Enchantment>(items: &[T]) -> u32 {
    todo!()
}

/// The name of the mightier of two enchantments, which may be of *different*
/// types. On a tie, `a` wins.
pub fn mightier<A: Enchantment, B: Enchantment>(a: &A, b: &B) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fireball_uses_the_default_description() {
        assert_eq!(Fireball.name(), "Fireball");
        assert_eq!(Fireball.power(), 45);
        assert_eq!(Fireball.describe(), "Fireball (45 power)");
    }

    #[test]
    fn frostbolt_gets_colder() {
        let bolt = Frostbolt { chill: 5 };
        assert_eq!(bolt.power(), 40);
        assert_eq!(bolt.describe(), "Frostbolt (40 power)");
    }

    #[test]
    fn runes_override_the_description() {
        let rune = Rune { glyph: 'ᚱ', strength: 3 };
        assert_eq!(rune.name(), "Rune ᚱ");
        assert_eq!(rune.describe(), "ᚱ glows with 30 power");
    }

    #[test]
    fn derived_traits_on_runes() {
        let rune = Rune { glyph: 'ᚠ', strength: 2 };
        let copy = rune; // Copy: `rune` is still usable below
        assert_eq!(rune, copy, "PartialEq lets us compare runes");
        assert_eq!(format!("{rune:?}"), "Rune { glyph: 'ᚠ', strength: 2 }");
    }

    #[test]
    fn runes_display_nicely() {
        assert_eq!(Rune { glyph: 'ᚱ', strength: 3 }.to_string(), "[ᚱ:3]");
    }

    #[test]
    fn announcing_anything() {
        assert_eq!(announce(&Fireball), "✨ Fireball (45 power)");
        assert_eq!(announce(&Rune { glyph: 'ᚦ', strength: 1 }), "✨ ᚦ glows with 10 power");
    }

    #[test]
    fn summing_power() {
        let runes = [
            Rune { glyph: 'ᚠ', strength: 1 },
            Rune { glyph: 'ᚢ', strength: 2 },
        ];
        assert_eq!(total_power(&runes), 30);
        assert_eq!(total_power(&[Frostbolt { chill: 0 }, Frostbolt { chill: 10 }]), 80);
    }

    #[test]
    fn comparing_different_schools() {
        assert_eq!(mightier(&Fireball, &Frostbolt { chill: 20 }), "Frostbolt");
        assert_eq!(mightier(&Rune { glyph: 'ᚱ', strength: 9 }, &Fireball), "Rune ᚱ");
        assert_eq!(mightier(&Fireball, &Frostbolt { chill: 10 }), "Fireball", "ties go to `a`");
    }
}
