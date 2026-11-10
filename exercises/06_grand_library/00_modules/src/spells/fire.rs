//! `crate::spells::fire` — the fire scrolls.

use super::Spell;
use crate::MAX_MANA;

/// The classic: ("Fireball", 30 mana, 45 power).
pub fn fireball() -> Spell {
    ("Fireball", 30, 45)
}

/// Can a mage with `mana` afford a Fireball? (Mana never exceeds MAX_MANA.)
pub fn can_afford(mana: u32) -> bool {
    mana.min(MAX_MANA) >= fireball().1
}
