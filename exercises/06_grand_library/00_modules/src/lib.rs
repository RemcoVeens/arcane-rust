// ⚔️ Quest 6.0 — Modules: The Shelves of the Library
// 📖 Read: book/src/06_grand_library/00_modules.md
//
// ❌ This crate does not compile yet. The Grand Library is a mess:
//   1. Some scrolls sit on private shelves (missing `pub`).
//   2. Two `use` paths point at the wrong shelf.
//   3. `src/potions.rs` does not exist — you must write it (spec below).
//   4. The tests want `fireball()` and `frostbolt()` reachable straight
//      from the crate root (`crate::fireball()`), not only via `spells::…`.
//
// The module tree:
//
//     crate                 src/lib.rs
//     ├── spells            src/spells/mod.rs
//     │   ├── fire          src/spells/fire.rs
//     │   └── frost         src/spells/frost.rs
//     └── potions           src/potions.rs        ← create me!
//         └── brewing       (an inline `pub mod brewing { … }` inside potions.rs)

pub mod spells;

/// The potion shelf. Create `src/potions.rs` with:
///
/// - `pub const POTION_STRENGTH: u32 = 50;`
/// - `pub fn mana_potion(mana: u32) -> u32` — restores `POTION_STRENGTH` mana,
///   capped at `crate::spells::MAX_MANA`.
/// - an inline module `pub mod brewing` containing
///   `pub fn brew(herb: &str, essence: &str) -> String` that returns
///   `"<herb>-<essence> Elixir"`, e.g. `brew("Moonleaf", "Ember")` →
///   `"Moonleaf-Ember Elixir"`.
pub mod potions;

// TODO: re-export `spells::fire::fireball` and `spells::frost::frostbolt`
// here so that `crate::fireball()` and `crate::frostbolt()` work.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fireball_is_reachable_from_the_crate_root() {
        assert_eq!(fireball(), ("Fireball", 30, 45));
        assert_eq!(crate::frostbolt(), ("Frostbolt", 25, 35));
    }

    #[test]
    fn the_full_path_still_works() {
        assert_eq!(spells::fire::fireball(), fireball());
        assert_eq!(spells::frost::frostbolt(), frostbolt());
    }

    #[test]
    fn frost_shatters_for_double_power() {
        assert_eq!(spells::frost::shatter(35), 70, "🧊 shatter doubles the power");
    }

    #[test]
    fn affording_a_fireball() {
        assert!(spells::fire::can_afford(30));
        assert!(!spells::fire::can_afford(29), "🔥 Fireball costs 30 mana");
    }

    #[test]
    fn the_catalog_lists_every_shelf() {
        let names: Vec<&str> = spells::catalog().iter().map(|s| s.0).collect();
        assert_eq!(names, vec!["Fireball", "Frostbolt"]);
    }

    #[test]
    fn mana_potions_restore_and_cap() {
        assert_eq!(potions::POTION_STRENGTH, 50);
        assert_eq!(potions::mana_potion(10), 60);
        assert_eq!(potions::mana_potion(80), spells::MAX_MANA, "🧪 never above MAX_MANA");
    }

    #[test]
    fn brewing_lives_inside_potions() {
        assert_eq!(potions::brewing::brew("Moonleaf", "Ember"), "Moonleaf-Ember Elixir");
    }
}
