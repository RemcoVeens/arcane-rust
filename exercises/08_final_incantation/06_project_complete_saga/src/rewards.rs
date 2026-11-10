//! 🔨 Phase 3 — Bounties: a trait with an associated type.

use crate::creature::Creature;

/// Something that pays out when conquered. Each implementor decides what
/// kind of `Prize` it gives.
pub trait Bounty {
    type Prize;
    fn bounty(&self) -> Self::Prize;
}

/// Gold plus a trophy for the stash.
#[derive(Debug, Clone, PartialEq)]
pub struct Loot {
    pub gold: u32,
    pub trophy: String,
}

/// A treasure chest. (Provided.)
#[derive(Debug, Clone, PartialEq)]
pub struct Chest {
    pub gold: u32,
}

impl Bounty for Chest {
    type Prize = Loot;
    fn bounty(&self) -> Loot {
        Loot { gold: self.gold, trophy: "Empty Chest".to_string() }
    }
}

/// A creature's bounty: `gold = max_hp / 2`, `trophy = "<name> Trophy"`
/// (e.g. "Goblin Trophy").
impl Bounty for Creature {
    type Prize = Loot;

    fn bounty(&self) -> Self::Prize {
        todo!()
    }
}

/// The total gold of anything whose prize is `Loot`. Note the bound:
/// `B: Bounty<Prize = Loot>` pins the associated type.
pub fn total_gold<B: Bounty<Prize = Loot>>(bounties: &[B]) -> u32 {
    todo!()
}

/// The spoils of battle: only creatures that have FALLEN (hp 0) pay out.
/// Returns their total gold and their trophies (in the given order).
pub fn spoils(creatures: &[Creature]) -> (u32, Vec<String>) {
    todo!()
}
