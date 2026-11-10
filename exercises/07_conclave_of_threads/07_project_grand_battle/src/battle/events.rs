//! Everything that happens in battle is a `BattleEvent`. (Provided.)

use std::fmt;

/// One thing that happened on the battlefield. Events are *sent* through a
/// channel by the combatant threads and *received* by the chronicler.
#[derive(Debug, Clone, PartialEq)]
pub enum BattleEvent {
    /// A creature claws at the mage. `damage` is the creature's attack value.
    Attack { attacker: String, target: String, damage: u32 },
    /// The mage's spell hits a creature. `damage` includes the weakness bonus.
    SpellCast { caster: String, spell: String, target: String, damage: u32 },
    /// The mage tried to cast but couldn't (`reason` is the error message).
    Fizzle { caster: String, reason: String },
    /// Someone dropped to 0 hp.
    Defeated { name: String },
}

impl fmt::Display for BattleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BattleEvent::Attack { attacker, target, damage } => {
                write!(f, "🗡️  {attacker} attacks {target} for {damage}")
            }
            BattleEvent::SpellCast { caster, spell, target, damage } => {
                write!(f, "✨ {caster} casts {spell} at {target} for {damage}")
            }
            BattleEvent::Fizzle { caster, reason } => {
                write!(f, "💨 {caster}'s spell fizzles: {reason}")
            }
            BattleEvent::Defeated { name } => write!(f, "💀 {name} is defeated!"),
        }
    }
}

/// How a battle ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Outcome {
    /// Every creature is down (and the mage still stands).
    Victory,
    /// The mage is down.
    Defeat,
    /// The battle plan ran out before either side fell.
    #[default]
    Retreat,
}
