//! `crate::spells::frost` — the frost scrolls.

use spells::Spell;

/// ("Frostbolt", 25 mana, 35 power).
pub fn frostbolt() -> Spell {
    ("Frostbolt", 25, 35)
}

/// A frozen enemy shatters for double the spell's power.
fn shatter(power: u32) -> u32 {
    power * 2
}
