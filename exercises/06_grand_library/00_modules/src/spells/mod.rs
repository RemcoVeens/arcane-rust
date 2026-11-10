//! The spell shelf: `crate::spells`.
//! This file is the root of the `spells` module; its children live next to it.

pub mod fire;
mod frost;

/// The maximum mana any mage can hold.
pub const MAX_MANA: u32 = 100;

/// A spell as (name, mana cost, power).
pub type Spell = (&'static str, u32, u32);

/// Every spell on this shelf, in shelf order (fire first, then frost).
pub fn catalog() -> Vec<Spell> {
    vec![fire::fireball(), frost::frostbolt()]
}
