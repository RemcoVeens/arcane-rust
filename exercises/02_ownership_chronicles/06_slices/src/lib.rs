// ⚔️ Quest 2.6 — Slices: A Window Into the Grimoire
// 📖 Read: book/src/02_ownership_chronicles/06_slices.md
//
// Replace each `todo!()`. Every function *borrows a view* — no copies.

/// (name, mana cost, power)
pub type Spell = (&'static str, u32, u32);

/// The first word of an incantation (up to the first space),
/// or the whole thing if there is no space.
/// `first_word("Fireball Frostbolt")` → "Fireball"
pub fn first_word(incantation: &str) -> &str {
    todo!()
}

/// Spell names are written "School::Name". Return the school.
/// `school("Pyromancy::Fireball")` → "Pyromancy"
/// No "::" at all? Then the school is "" (an empty slice).
/// Hint: `.find("::")` gives `Some(index)` or `None`.
pub fn school(full_name: &str) -> &str {
    todo!()
}

/// How many *runes* (chars) vs how many *bytes*: return (chars, bytes).
/// `rune_and_byte_count("🔥fire")` → (5, 8)
pub fn rune_and_byte_count(text: &str) -> (usize, usize) {
    todo!()
}

/// The highest power in a slice of powers, or 0 for an empty slice.
/// Works for arrays AND vectors, because both can lend out a `&[u32]`.
pub fn strongest(powers: &[u32]) -> u32 {
    todo!()
}

/// The first `n` spells of a spellbook — or all of them, if there are
/// fewer than `n`. Return a slice of the original, don't build a new Vec.
pub fn first_spells(spellbook: &[Spell], n: usize) -> &[Spell] {
    todo!()
}

/// Total mana cost of a slice of spells.
pub fn total_cost(spells: &[Spell]) -> u32 {
    todo!()
}

/// Empower a slice of damage values in place: double each one.
pub fn empower(damage: &mut [u32]) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOOK: [Spell; 5] = [
        ("Fireball", 30, 45),
        ("Frostbolt", 25, 35),
        ("Arcane Blast", 40, 60),
        ("Heal", 20, 0),
        ("Meteor", 90, 150),
    ];

    #[test]
    fn first_words() {
        assert_eq!(first_word("Fireball Frostbolt Arcane Blast"), "Fireball");
        assert_eq!(first_word("Meteor"), "Meteor");
        let owned = String::from("Heal thyself");
        assert_eq!(first_word(&owned), "Heal", "📜 a &String works as a &str too");
    }

    #[test]
    fn schools_of_magic() {
        assert_eq!(school("Pyromancy::Fireball"), "Pyromancy");
        assert_eq!(school("Cryomancy::Frostbolt"), "Cryomancy");
        assert_eq!(school("Fireball"), "");
    }

    #[test]
    fn runes_are_not_bytes() {
        assert_eq!(rune_and_byte_count("fire"), (4, 4));
        assert_eq!(rune_and_byte_count("🔥fire"), (5, 8));
    }

    #[test]
    fn strongest_power() {
        assert_eq!(strongest(&[45, 35, 60, 0, 150]), 150);
        assert_eq!(strongest(&Vec::from([3, 9, 4])), 9);
        assert_eq!(strongest(&[]), 0);
    }

    #[test]
    fn first_spells_are_a_view() {
        assert_eq!(first_spells(&BOOK, 2), &BOOK[..2]);
        assert_eq!(first_spells(&BOOK, 99).len(), 5);
        assert!(first_spells(&BOOK, 0).is_empty());
    }

    #[test]
    fn total_cost_of_a_slice() {
        assert_eq!(total_cost(&BOOK), 205);
        assert_eq!(total_cost(&BOOK[1..3]), 65);
    }

    #[test]
    fn empowering_part_of_an_array() {
        let mut damage = [10, 20, 30, 40, 50];
        empower(&mut damage[1..4]);
        assert_eq!(damage, [10, 40, 60, 80, 50]);
    }
}
