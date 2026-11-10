//! The shared battlefield: one state behind `Arc<Mutex<…>>`, many threads.

use std::sync::{Arc, Mutex};

use super::events::Outcome;
use crate::creature::Creature;
use crate::error::BattleError;
use crate::mage::Mage;
use crate::spells::Spell;

/// Everything on the battlefield at one moment.
#[derive(Debug, Clone, PartialEq)]
pub struct BattleState {
    pub mage: Mage,
    pub creatures: Vec<Creature>,
}

/// A cheap-to-clone handle to the shared battlefield.
#[derive(Debug, Clone)]
pub struct Battlefield {
    state: Arc<Mutex<BattleState>>,
}

impl Battlefield {
    pub fn new(mage: Mage, creatures: Vec<Creature>) -> Self {
        Battlefield { state: Arc::new(Mutex::new(BattleState { mage, creatures })) }
    }

    /// A copy of the current state.
    pub fn snapshot(&self) -> BattleState {
        self.state.lock().unwrap().clone()
    }

    /// Damage creature `index`; `true` only for the blow that fells it.
    pub fn strike_creature(&self, index: usize, damage: u32) -> bool {
        let mut state = self.state.lock().unwrap();
        match state.creatures.get_mut(index) {
            Some(c) if c.is_alive() => c.take_damage(damage) == 0,
            _ => false,
        }
    }

    /// Damage the mage; `true` only for the blow that fells him.
    pub fn strike_mage(&self, damage: u32) -> bool {
        let mut state = self.state.lock().unwrap();
        state.mage.is_alive() && state.mage.take_damage(damage) == 0
    }

    /// The mage pays for `spell` on the shared state.
    pub fn cast(&self, spell: &Spell) -> Result<u32, BattleError> {
        self.state.lock().unwrap().mage.cast(spell)
    }

    /// `Defeat` if the mage is down, `Victory` if every creature is, else `None`.
    pub fn outcome(&self) -> Option<Outcome> {
        let state = self.state.lock().unwrap();
        if !state.mage.is_alive() {
            Some(Outcome::Defeat)
        } else if state.creatures.iter().all(|c| !c.is_alive()) {
            Some(Outcome::Victory)
        } else {
            None
        }
    }
}
