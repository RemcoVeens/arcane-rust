// ⚔️ Quest 2.2 — Moves: The Passing of the Staff
// 📖 Read: book/src/02_ownership_chronicles/02_moves.md
//
// ❌ This file does not compile yet. That's the quest!
// Every function uses a value after it was moved away.
// Fix them WITHOUT `.clone()` — reorder, use the new owner, or give
// ownership back. (Cloning is next quest; here you learn to avoid it.)

/// The staff is handed to the vault. Report on it *through its new owner*.
/// Expected: "Staff of Embers is safe in the vault"
pub fn store_in_vault() -> String {
    let staff = String::from("Staff of Embers");
    let vault = staff;
    format!("{vault} is safe in the vault")
}

/// Count the runes in a scroll and give it back to the caller.
/// The signature of `count_runes` is wrong: it swallows the scroll.
/// Change it so it returns the scroll together with the count,
/// as `(String, usize)`, and fix `read_after_counting` to use it.
pub fn count_runes(scroll: String) -> (String, usize) {
    let count = scroll.chars().count();
    (scroll, count)
}

/// Expected: (16, "ᚠ Scroll of Wind") — 16 runes, scroll still readable.
pub fn read_after_counting() -> (usize, String) {
    let scroll = String::from("ᚠ Scroll of Wind");
    let (scroll, count) = count_runes(scroll);
    (count, scroll)
}

/// Pack a potion in the bag, and also return the length of its label.
/// Once the potion is in the bag, the bag owns it!
/// Expected: (vec!["Potion of Swiftness"], 19)
pub fn pack_bag() -> (Vec<String>, usize) {
    let mut bag = Vec::new();
    let potion = String::from("Potion of Swiftness");
    let label_len = potion.len();
    bag.push(potion);
    (bag, label_len)
}

/// A spell is (name, power). Take the name out of the tuple and describe it.
/// Expected: "Frostbolt has power 35"
pub fn describe(spell: (String, u32)) -> String {
    let name = spell.0;
    let power = spell.1;
    format!("{name} has power {power}")
}

/// Copy types are never moved: this one already works. Don't touch it —
/// but do figure out *why* it compiles.
pub fn mana_twice() -> (u32, u32) {
    let mana = 100;
    let backup = mana;
    (mana, backup)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_vault_owns_the_staff() {
        assert_eq!(store_in_vault(), "Staff of Embers is safe in the vault");
    }

    #[test]
    fn counting_gives_the_scroll_back() {
        let (scroll, count) = count_runes(String::from("ᚠᚢᚦ"));
        assert_eq!((scroll.as_str(), count), ("ᚠᚢᚦ", 3));
        assert_eq!(
            read_after_counting(),
            (16, String::from("ᚠ Scroll of Wind"))
        );
    }

    #[test]
    fn the_bag_owns_the_potion() {
        let (bag, len) = pack_bag();
        assert_eq!(bag, vec![String::from("Potion of Swiftness")]);
        assert_eq!(len, 19);
    }

    #[test]
    fn describing_a_spell_tuple() {
        assert_eq!(
            describe((String::from("Frostbolt"), 35)),
            "Frostbolt has power 35"
        );
    }

    #[test]
    fn copy_types_are_copied() {
        assert_eq!(mana_twice(), (100, 100));
    }
}
