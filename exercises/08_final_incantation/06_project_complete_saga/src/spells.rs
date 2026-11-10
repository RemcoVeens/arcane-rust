//! 🔨 Phase 1 — The grimoire, written in its own little language.

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

/// The spell-table DSL. Entries are separated by `;` (trailing `;` allowed):
///
/// ```text
/// grimoire! {
///     "Fireball"  => Fire,  cost 30, power 45, level 1;
///     "Frostbolt" => Frost, cost 25, power 35, level 2;
/// }
/// ```
///
/// - the name is a string literal (`$name:literal`), so it may contain spaces
/// - the element is a bare `Element` variant name (`$element:ident`)
/// - `cost`, `power` and `level` are literal keywords in the pattern,
///   each followed by an expression
///
/// Expands to a `Vec<Spell>` built with `Spell::new`. Use full
/// `$crate::spells::…` paths so the macro works from anywhere (the tests call
/// it from another crate!).
#[macro_export]
macro_rules! grimoire {
    // TODO: replace this placeholder rule with the real one.
    ($($anything:tt)*) => {{
        let placeholder: Vec<$crate::spells::Spell> = todo!();
        placeholder
    }};
}

/// All known spells — defined with the DSL above.
pub fn grimoire() -> Vec<Spell> {
    grimoire! {
        "Fireball"     => Fire,   cost 30, power 45,  level 1;
        "Frostbolt"    => Frost,  cost 25, power 35,  level 2;
        "Arcane Blast" => Arcane, cost 40, power 60,  level 5;
        "Heal"         => Holy,   cost 20, power 0,   level 1;
        "Meteor"       => Fire,   cost 90, power 150, level 10;
    }
}

/// Find a spell by name, ignoring case.
pub fn find_spell(name: &str) -> Result<Spell, BattleError> {
    grimoire()
        .into_iter()
        .find(|s| s.name.eq_ignore_ascii_case(name.trim()))
        .ok_or_else(|| BattleError::UnknownSpell(name.trim().to_string()))
}
