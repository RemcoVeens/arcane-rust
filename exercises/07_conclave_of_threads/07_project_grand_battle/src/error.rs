//! Everything that can go wrong in battle. (Provided — nothing to do here.)

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BattleError {
    UnknownSpell(String),
    LevelTooLow { spell: String, required: u32, level: u32 },
    NotEnoughMana { spell: String, needed: u32, available: u32 },
    NoTargets,
}

impl fmt::Display for BattleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BattleError::UnknownSpell(name) => write!(f, "unknown spell '{name}'"),
            BattleError::LevelTooLow { spell, required, level } => {
                write!(f, "{spell} requires level {required} (you are level {level})")
            }
            BattleError::NotEnoughMana { spell, needed, available } => {
                write!(f, "not enough mana for {spell}: need {needed}, have {available}")
            }
            BattleError::NoTargets => write!(f, "no creatures left to target"),
        }
    }
}

impl std::error::Error for BattleError {}
