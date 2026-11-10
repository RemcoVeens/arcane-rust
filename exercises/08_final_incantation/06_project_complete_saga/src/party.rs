//! 🔨 Phase 2 — The Party: shared heroes and a shared stash.
//!
//! The party, the saga and `main` all need to hold the *same* heroes and
//! change them — single-threaded shared ownership with mutation:
//! `Rc<RefCell<…>>`.

use std::cell::RefCell;
use std::rc::Rc;

use crate::mage::{MAX_MANA, Mage};

/// A hero is a mage that several owners can hold and change.
pub type Hero = Rc<RefCell<Mage>>;

/// How much hp one potion restores (capped at `max_hp`).
pub const POTION_HEAL: u32 = 50;

/// The party's shared belongings.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Stash {
    pub gold: u32,
    pub potions: u32,
    pub trophies: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Party {
    members: Vec<Hero>,
    stash: Rc<RefCell<Stash>>,
}

impl Party {
    /// An empty party whose stash holds `potions` potions and no gold.
    pub fn new(potions: u32) -> Self {
        todo!()
    }

    /// Wrap `mage` into a `Hero`, keep one handle in the party and return
    /// another to the caller.
    pub fn recruit(&mut self, mage: Mage) -> Hero {
        todo!()
    }

    /// All heroes, in the order they were recruited.
    pub fn members(&self) -> &[Hero] {
        &self.members
    }

    /// A handle to the shared stash (another `Rc` to the same stash).
    pub fn stash(&self) -> Rc<RefCell<Stash>> {
        todo!()
    }

    /// The first hero (in recruit order) who is still alive.
    pub fn leader(&self) -> Option<Hero> {
        todo!()
    }

    /// The sum of every hero's hp.
    pub fn total_hp(&self) -> u32 {
        todo!()
    }

    /// `hero` drinks a potion from the stash: if there is one, use it up and
    /// restore `POTION_HEAL` hp (capped at the hero's `max_hp`), returning
    /// `true`. No potions left → `false`, nothing changes.
    pub fn drink_potion(&self, hero: &Hero) -> bool {
        todo!()
    }

    /// A night at the inn: every hero — fallen ones too — is restored to
    /// full hp and `MAX_MANA` mana.
    pub fn rest(&self) {
        todo!()
    }
}
