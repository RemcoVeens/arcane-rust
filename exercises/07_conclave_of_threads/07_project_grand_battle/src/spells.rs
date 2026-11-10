//! The grimoire: every spell Tiberius knows. (Provided — nothing to do here.)

use crate::error::BattleError;

/// The four schools of magic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
    Holy,
}

/// A spell from the grimoire.
#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: String,
    pub element: Element,
    pub cost: u32,
    pub power: u32,
    pub level: u32,
}

impl Spell {
    pub fn new(name: &str, element: Element, cost: u32, power: u32, level: u32) -> Self {
        Spell { name: name.to_string(), element, cost, power, level }
    }
}

/// All known spells.
pub fn grimoire() -> Vec<Spell> {
    vec![
        Spell::new("Fireball", Element::Fire, 30, 45, 1),
        Spell::new("Frostbolt", Element::Frost, 25, 35, 2),
        Spell::new("Arcane Blast", Element::Arcane, 40, 60, 5),
        Spell::new("Heal", Element::Holy, 20, 0, 1),
        Spell::new("Meteor", Element::Fire, 90, 150, 10),
    ]
}

/// Find a spell by name, ignoring case.
///
/// ```
/// use ch07_07_project_grand_battle::find_spell;
/// assert_eq!(find_spell("fireball").unwrap().power, 45);
/// assert!(find_spell("Summon Pizza").is_err());
/// ```
pub fn find_spell(name: &str) -> Result<Spell, BattleError> {
    grimoire()
        .into_iter()
        .find(|s| s.name.eq_ignore_ascii_case(name.trim()))
        .ok_or_else(|| BattleError::UnknownSpell(name.trim().to_string()))
}
