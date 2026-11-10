// ⚔️ Quest 3.2 — Enums: The Choice of Forms
// 📖 Read: book/src/03_grimoire_of_forms/02_enums.md
//
// ❌ This file does not compile yet: the `Potion` enum is missing.
// Define it, then replace each `todo!()`.

/// The three elements. Each one beats the next:
/// Fire beats Frost, Frost beats Arcane, Arcane beats Fire.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
}

/// The element that `element` is strong against.
/// `strong_against(Element::Fire)` → `Element::Frost`
pub fn strong_against(element: Element) -> Element {
    todo!()
}

/// The rune for each element: Fire '🔥', Frost '❄', Arcane '✨'.
pub fn rune(element: Element) -> char {
    todo!()
}

// TODO: define `pub enum Potion` with #[derive(Debug, Clone, PartialEq)]
// and these four variants:
//   - `Health`, carrying a u32 (the HP it restores)
//   - `Mana`, carrying a u32 (the mana it restores)
//   - `Invisibility`, with a named field `turns: u32`
//   - `Water`, carrying nothing

/// A label for the shop window:
///   Health(30)                 → "Health potion (+30 HP)"
///   Mana(50)                   → "Mana potion (+50 mana)"
///   Invisibility { turns: 3 }  → "Invisibility potion (3 turns)"
///   Water                      → "Just water"
pub fn label(potion: &Potion) -> String {
    todo!()
}

/// How much mana drinking this potion restores. Only Mana potions do.
/// Hint: a `_` arm catches "everything else".
pub fn mana_restored(potion: &Potion) -> u32 {
    todo!()
}

/// Ranks with explicit discriminants: the number is the minimum level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Apprentice = 1,
    Adept = 5,
    Archmage = 10,
}

/// The minimum level for a rank. Hint: `rank as u32`.
pub fn min_level(rank: Rank) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cycle_of_elements() {
        assert_eq!(strong_against(Element::Fire), Element::Frost);
        assert_eq!(strong_against(Element::Frost), Element::Arcane);
        assert_eq!(strong_against(Element::Arcane), Element::Fire);
    }

    #[test]
    fn element_runes() {
        assert_eq!(rune(Element::Fire), '🔥');
        assert_eq!(rune(Element::Frost), '❄');
        assert_eq!(rune(Element::Arcane), '✨');
    }

    #[test]
    fn potion_labels() {
        assert_eq!(label(&Potion::Health(30)), "Health potion (+30 HP)");
        assert_eq!(label(&Potion::Mana(50)), "Mana potion (+50 mana)");
        assert_eq!(label(&Potion::Invisibility { turns: 3 }), "Invisibility potion (3 turns)");
        assert_eq!(label(&Potion::Water), "Just water");
    }

    #[test]
    fn only_mana_potions_restore_mana() {
        assert_eq!(mana_restored(&Potion::Mana(50)), 50);
        assert_eq!(mana_restored(&Potion::Health(50)), 0);
        assert_eq!(mana_restored(&Potion::Water), 0);
    }

    #[test]
    fn rank_levels() {
        assert_eq!(min_level(Rank::Apprentice), 1);
        assert_eq!(min_level(Rank::Adept), 5);
        assert_eq!(min_level(Rank::Archmage), 10);
    }
}
