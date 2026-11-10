// ⚔️ Quest 6.6 — Profiles & Features: The Forge Settings
// 📖 Read: book/src/06_grand_library/06_features.md
//
// One spellbook, three editions, chosen at compile time with Cargo features:
//
//   cargo test                          → the Apprentice Edition
//   cargo test --features enchanted     → the Enchanted Edition
//   cargo test --features dragon_lore   → the Draconic Edition
//                                         (dragon_lore ALWAYS enables enchanted)
//
// `wr` runs all three. To get there:
//   1. Declare the two features in this package's Cargo.toml (`[features]`),
//      with `dragon_lore` depending on `enchanted`.
//   2. Implement the functions below using `#[cfg(feature = "…")]` and/or
//      `cfg!(feature = "…")`.
//   3. Create the `dragon_lore` module — it must exist ONLY when the
//      `dragon_lore` feature is on.
//
// Don't edit the tests: notice how they are themselves feature-gated.

/// The name of this edition:
///   no features  → "Apprentice Edition"
///   enchanted    → "Enchanted Edition"
///   dragon_lore  → "Draconic Edition"
pub fn edition() -> &'static str {
    todo!()
}

/// The spells in this edition, in this order:
///   always:            "Fireball", "Frostbolt", "Heal"
///   + enchanted:       "Arcane Blast"
///   + dragon_lore:     "Meteor"
pub fn spell_list() -> Vec<&'static str> {
    todo!()
}

/// Enchanted editions double all spell damage.
pub fn empowered(power: u32) -> u32 {
    todo!()
}

// TODO: a module `dragon_lore`, compiled only with the `dragon_lore` feature,
// containing `pub fn true_name(dragon: &str) -> String` that returns
// "<dragon> the Undying", e.g. "Vyrmathax the Undying".

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_basic_spells_are_always_there() {
        let spells = spell_list();
        assert_eq!(&spells[..3], &["Fireball", "Frostbolt", "Heal"]);
    }

    #[test]
    #[cfg(not(feature = "enchanted"))]
    fn the_apprentice_edition() {
        assert_eq!(edition(), "Apprentice Edition");
        assert_eq!(spell_list().len(), 3);
        assert_eq!(empowered(45), 45);
    }

    #[test]
    #[cfg(all(feature = "enchanted", not(feature = "dragon_lore")))]
    fn the_enchanted_edition() {
        assert_eq!(edition(), "Enchanted Edition");
        assert_eq!(spell_list(), vec!["Fireball", "Frostbolt", "Heal", "Arcane Blast"]);
    }

    #[test]
    #[cfg(feature = "enchanted")]
    fn enchantment_doubles_damage() {
        assert_eq!(empowered(45), 90, "✨ enchanted spells hit twice as hard");
    }

    #[test]
    #[cfg(feature = "dragon_lore")]
    fn the_draconic_edition() {
        assert!(cfg!(feature = "enchanted"), "🐉 dragon_lore must enable enchanted");
        assert_eq!(edition(), "Draconic Edition");
        assert_eq!(spell_list().last(), Some(&"Meteor"));
        assert_eq!(spell_list().len(), 5);
    }

    #[test]
    #[cfg(feature = "dragon_lore")]
    fn dragons_have_true_names() {
        assert_eq!(dragon_lore::true_name("Vyrmathax"), "Vyrmathax the Undying");
    }
}
