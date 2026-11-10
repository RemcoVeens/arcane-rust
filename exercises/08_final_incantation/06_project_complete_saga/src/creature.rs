//! The horde.

use crate::spells::{Element, Spell};

#[derive(Debug, Clone, PartialEq)]
pub struct Creature {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    /// Damage dealt to the mage each round.
    pub attack: u32,
    /// Spells of this element deal double damage.
    pub weakness: Option<Element>,
}

impl Creature {
    pub fn new(name: &str, hp: u32, attack: u32, weakness: Option<Element>) -> Self {
        Creature { name: name.to_string(), hp, max_hp: hp, attack, weakness }
    }

    pub fn goblin() -> Self {
        Creature::new("Goblin", 30, 8, Some(Element::Fire))
    }

    pub fn wraith() -> Self {
        Creature::new("Wraith", 50, 12, Some(Element::Arcane))
    }

    pub fn ice_golem() -> Self {
        Creature::new("Ice Golem", 80, 15, Some(Element::Fire))
    }

    pub fn dragon() -> Self {
        Creature::new("Dragon", 300, 40, Some(Element::Frost))
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Lose `amount` hp (never below 0). Returns the hp left.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        self.hp = self.hp.saturating_sub(amount);
        self.hp
    }

    /// How much damage `spell` does to this creature: double against its weakness.
    pub fn damage_from(&self, spell: &Spell) -> u32 {
        if self.weakness == Some(spell.element) {
            spell.power * 2
        } else {
            spell.power
        }
    }
}
