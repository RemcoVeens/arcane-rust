// ⚔️ Quest 4.0 — Vectors: The Dynamic Grimoire
// 📖 Read: book/src/04_forge_of_resilience/00_vectors.md
//
// Replace each `todo!()` with real code.

/// A fresh apprentice's spellbook: exactly `["Fireball", "Frostbolt"]`, in that order.
/// (Use the `vec!` macro.)
pub fn new_spellbook() -> Vec<String> {
    todo!()
}

/// Add `spell` to the end of the book — unless the book already contains it.
/// A mage never writes the same spell twice.
pub fn learn(book: &mut Vec<String>, spell: &str) {
    todo!()
}

/// Remove `spell` from the book. Returns `true` if it was there, `false` otherwise.
/// The order of the remaining spells must be preserved.
pub fn forget(book: &mut Vec<String>, spell: &str) -> bool {
    todo!()
}

/// The spell on page `page` (0-based), or `None` if the book is too short.
/// Must never panic!
pub fn page(book: &[String], page: usize) -> Option<&String> {
    todo!()
}

/// Tear out the *last* page and return the spell written on it.
/// An empty book gives `None`.
pub fn tear_last_page(book: &mut Vec<String>) -> Option<String> {
    todo!()
}

/// The total mana cost of casting every spell in `costs`.
pub fn total_cost(costs: &[u32]) -> u32 {
    todo!()
}

/// The highest power in `powers`, or `None` for an empty slice.
pub fn strongest(powers: &[u32]) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apprentices_start_with_two_spells() {
        assert_eq!(new_spellbook(), vec!["Fireball", "Frostbolt"]);
    }

    #[test]
    fn learning_appends_new_spells() {
        let mut book = new_spellbook();
        learn(&mut book, "Heal");
        learn(&mut book, "Meteor");
        assert_eq!(book, vec!["Fireball", "Frostbolt", "Heal", "Meteor"]);
    }

    #[test]
    fn learning_twice_changes_nothing() {
        let mut book = new_spellbook();
        learn(&mut book, "Fireball");
        assert_eq!(book.len(), 2, "📕 Fireball was written twice: {book:?}");
    }

    #[test]
    fn forgetting_keeps_the_order() {
        let mut book = vec![
            "Fireball".to_string(),
            "Frostbolt".to_string(),
            "Heal".to_string(),
        ];
        assert!(forget(&mut book, "Frostbolt"));
        assert_eq!(book, vec!["Fireball", "Heal"]);
        assert!(!forget(&mut book, "Frostbolt"), "it is already gone");
    }

    #[test]
    fn pages_are_looked_up_safely() {
        let book = new_spellbook();
        assert_eq!(page(&book, 1), Some(&"Frostbolt".to_string()));
        assert_eq!(page(&book, 99), None, "🔥 page 99 does not exist — don't panic!");
    }

    #[test]
    fn tearing_pages_from_the_back() {
        let mut book = new_spellbook();
        assert_eq!(tear_last_page(&mut book), Some("Frostbolt".to_string()));
        assert_eq!(tear_last_page(&mut book), Some("Fireball".to_string()));
        assert_eq!(tear_last_page(&mut book), None);
    }

    #[test]
    fn costs_add_up() {
        assert_eq!(total_cost(&[30, 25, 40]), 95);
        assert_eq!(total_cost(&[]), 0);
    }

    #[test]
    fn finding_the_strongest_spell() {
        assert_eq!(strongest(&[45, 150, 35, 60]), Some(150));
        assert_eq!(strongest(&[]), None);
    }
}
