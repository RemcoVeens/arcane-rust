// ⚔️ Quest 6.1 — Crates & Packages: The Bound Tomes
// 📖 Read: book/src/06_grand_library/01_crates_packages.md
//
// ❌ This package does not compile yet: it uses the `runestone` crate,
// but never declared it as a dependency.
//
// `runestone` lives right next to you in `runestone/` (it's finished).
//   1. Add it to *this* package's `Cargo.toml` as a path dependency.
//   2. Implement the `todo!()`s below using its public API:
//        runestone::Runestone::carve(text)  .inscription()  .power()
//        runestone::rune_count(text)
//        runestone::glyphs::to_glyph(c)
//
// This package is BOTH a library (src/lib.rs) and a binary (src/main.rs).
// Try it once it builds:  cargo run -p ch06_01_crates_packages -- Tiberius

use runestone::Runestone;

/// Carve a spell name into a stone and return the stone's power.
/// "Fireball" has 8 letters → 24 power.
pub fn spell_power(name: &str) -> u32 {
    todo!()
}

/// Translate a name into runes, character by character.
/// "Tiberius" → "ᛏᛁᛒᛖᚱᛁᚢᛊ"
pub fn glyph_name(name: &str) -> String {
    todo!()
}

/// Carve a greeting for a mage: the inscription is "Hail, <mage>!".
pub fn greeting_stone(mage: &str) -> Runestone {
    todo!()
}

/// Of all the spell names, return the one with the most letters
/// (the first one wins ties). `None` for an empty list.
/// Count letters with `runestone::rune_count` — spaces don't count!
pub fn mightiest<'a>(names: &[&'a str]) -> Option<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_give_power() {
        assert_eq!(spell_power("Fireball"), 24);
        assert_eq!(spell_power("Arcane Blast"), 33, "🪨 spaces carry no power");
    }

    #[test]
    fn names_become_glyphs() {
        assert_eq!(glyph_name("Tiberius"), "ᛏᛁᛒᛖᚱᛁᚢᛊ");
        assert_eq!(glyph_name("Lv 7"), "ᛚᚡ 7");
    }

    #[test]
    fn greeting_stones_are_carved() {
        let stone = greeting_stone("Tiberius");
        assert_eq!(stone.inscription(), "Hail, Tiberius!");
        assert_eq!(stone.power(), 4 * 3 + 8 * 3);
    }

    #[test]
    fn the_mightiest_spell_has_the_most_letters() {
        let spells = ["Heal", "Arcane Blast", "Fireball", "Frostbolt"];
        assert_eq!(mightiest(&spells), Some("Arcane Blast"));
        assert_eq!(mightiest(&["Heal", "Fire"]), Some("Heal"), "first wins ties");
        assert_eq!(mightiest(&[]), None);
    }
}
