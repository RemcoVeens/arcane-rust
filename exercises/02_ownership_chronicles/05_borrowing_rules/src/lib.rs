// ⚔️ Quest 2.5 — Borrowing Rules: The Compact of the Arcane
// 📖 Read: book/src/02_ownership_chronicles/05_borrowing_rules.md
//
// ❌ This file does not compile yet. That's the quest!
// Each function breaks one rule of the Compact:
//   • many readers (`&`) OR one writer (`&mut`) — never both at once
//   • a reference may never outlive what it points to
// Fix them without `.clone()`. Keep every function's signature
// except the one in `conjure_name`.

/// Learn Frostbolt, and report the grimoire's length before and after.
/// Expected: (8, 19)
pub fn grimoire_growth() -> (usize, usize) {
    let mut grimoire = String::from("Fireball");
    let before = &grimoire;
    grimoire.push_str(", Frostbolt");
    (before.len(), grimoire.len())
}

/// Two drains on the same mana pool: 30, then 25.
/// Expected: 45
pub fn double_drain() -> u32 {
    let mut mana = 100;
    let first = &mut mana;
    let second = &mut mana;
    *first -= 30;
    *second -= 25;
    mana
}

fn spend(mana: &mut u32, cost: u32) {
    *mana -= cost;
}

/// Glance at the mana before casting, then cast Fireball (30).
/// Expected: (100, 70) — before and after.
pub fn glance_then_cast() -> (u32, u32) {
    let mut mana = 100;
    let glance = &mana;
    spend(&mut mana, 30);
    (*glance, mana)
}

/// Learn a new spell, then describe the book.
/// Expected for a book ["Fireball"] and new spell "Meteor":
///   "First spell: Fireball, spells known: 2"
pub fn learn_and_describe(book: &mut Vec<String>, new_spell: String) -> String {
    let first = &book[0];
    book.push(new_spell);
    format!("First spell: {first}, spells known: {}", book.len())
}

/// Conjure the mage's name. The name dies at the end of this function,
/// so a reference to it would dangle! Change the return type so the
/// caller receives the *owned* name instead.
pub fn conjure_name() -> &String {
    let name = String::from("Tiberius");
    &name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grimoire_grows() {
        assert_eq!(grimoire_growth(), (8, 19));
    }

    #[test]
    fn two_drains_in_turn() {
        assert_eq!(double_drain(), 45);
    }

    #[test]
    fn glancing_before_casting() {
        assert_eq!(glance_then_cast(), (100, 70));
    }

    #[test]
    fn learning_while_reading() {
        let mut book = vec![String::from("Fireball")];
        let text = learn_and_describe(&mut book, String::from("Meteor"));
        assert_eq!(text, "First spell: Fireball, spells known: 2");
        assert_eq!(book, vec!["Fireball", "Meteor"]);
    }

    #[test]
    fn conjured_names_are_owned() {
        assert_eq!(conjure_name(), "Tiberius");
    }
}
