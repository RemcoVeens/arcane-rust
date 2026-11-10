//! Our hero.

use crate::error::BattleError;
use crate::spells::Spell;

pub const MAX_MANA: u32 = 100;
pub const MAX_HP: u32 = 100;

#[derive(Debug, Clone, PartialEq)]
pub struct Mage {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub mana: u32,
    pub level: u32,
}

impl Mage {
    pub fn new(name: &str, level: u32) -> Self {
        Mage { name: name.to_string(), hp: MAX_HP, max_hp: MAX_HP, mana: MAX_MANA, level }
    }

    /// Tiberius, level 5: Arcane Blast yes, Meteor not yet.
    pub fn tiberius() -> Self {
        Mage::new("Tiberius", 5)
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Lose `amount` hp (never below 0). Returns the hp left.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        self.hp = self.hp.saturating_sub(amount);
        self.hp
    }

    /// Pay for `spell`: checks level, then mana, then spends the mana.
    /// Returns the spell's base power.
    pub fn cast(&mut self, spell: &Spell) -> Result<u32, BattleError> {
        if self.level < spell.level {
            return Err(BattleError::LevelTooLow {
                spell: spell.name.clone(),
                required: spell.level,
                level: self.level,
            });
        }
        if self.mana < spell.cost {
            return Err(BattleError::NotEnoughMana {
                spell: spell.name.clone(),
                needed: spell.cost,
                available: self.mana,
            });
        }
        self.mana -= spell.cost;
        Ok(spell.power)
    }
}
