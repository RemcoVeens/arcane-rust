// ⚔️ Quest 3.3 — Option: The Absence of Null
// 📖 Read: book/src/03_grimoire_of_forms/03_option.md
//
// Replace each `todo!()`. No nulls, no sentinel values — only `Option`.

/// (name, mana cost)
pub const SPELLS: [(&str, u32); 5] = [
    ("Fireball", 30),
    ("Frostbolt", 25),
    ("Arcane Blast", 40),
    ("Heal", 20),
    ("Meteor", 90),
];

/// The mana cost of a spell, looked up by name (ignoring case),
/// or `None` if no such spell exists.
pub fn spell_cost(name: &str) -> Option<u32> {
    todo!()
}

/// The staff's bonus power, or 0 if the mage carries no staff.
pub fn staff_bonus(staff: Option<u32>) -> u32 {
    todo!()
}

/// Share the gold between `mages` mages. Zero mages? Then there is no
/// share — return `None` instead of crashing on division by zero.
pub fn share_loot(gold: u32, mages: u32) -> Option<u32> {
    todo!()
}

/// An empowered spell does double power. No spell, no power.
/// Try `.map(...)`.
pub fn empowered(power: Option<u32>) -> Option<u32> {
    todo!()
}

/// The total cost of casting two spells in a row, or `None` if either
/// one is unknown. Try the `?` operator on `spell_cost(...)`.
pub fn combo_cost(first: &str, second: &str) -> Option<u32> {
    todo!()
}

/// Call your familiar:
///   Some("Archimedes") → "Archimedes hoots!"
///   None               → "No familiar answers."
pub fn call_familiar(familiar: Option<&str>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looking_up_spells() {
        assert_eq!(spell_cost("Fireball"), Some(30));
        assert_eq!(spell_cost("arcane blast"), Some(40));
        assert_eq!(spell_cost("Summon Pizza"), None);
    }

    #[test]
    fn staff_bonus_defaults_to_zero() {
        assert_eq!(staff_bonus(Some(12)), 12);
        assert_eq!(staff_bonus(None), 0);
    }

    #[test]
    fn sharing_loot_safely() {
        assert_eq!(share_loot(100, 3), Some(33));
        assert_eq!(share_loot(100, 0), None, "🪙 no mages, no share — and no panic");
    }

    #[test]
    fn empowering_maybe_a_spell() {
        assert_eq!(empowered(Some(45)), Some(90));
        assert_eq!(empowered(None), None);
    }

    #[test]
    fn combos_need_both_spells() {
        assert_eq!(combo_cost("Fireball", "Frostbolt"), Some(55));
        assert_eq!(combo_cost("Fireball", "Dance"), None);
        assert_eq!(combo_cost("Dance", "Heal"), None);
    }

    #[test]
    fn calling_the_familiar() {
        assert_eq!(call_familiar(Some("Archimedes")), "Archimedes hoots!");
        assert_eq!(call_familiar(None), "No familiar answers.");
    }
}
