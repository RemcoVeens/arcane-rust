// ⚔️ Quest 5.6 — Iterator Adapters: Spell Analytics
// 📖 Read: book/src/05_temporal_loom/06_adapters.md
//
// The Academy's Battle Chronicler recorded every spell Tiberius cast.
// Answer the Archmage's questions about the log — each with ONE iterator
// chain (no `for` loops, no `mut` accumulators!).

use std::collections::HashMap;

/// One entry of the battle log.
#[derive(Debug, Clone, PartialEq)]
pub struct Cast {
    pub spell: String,
    pub damage: u32,
    pub turn: u32,
    pub critical: bool,
}

impl Cast {
    pub fn new(spell: &str, damage: u32, turn: u32, critical: bool) -> Self {
        Cast { spell: spell.to_string(), damage, turn, critical }
    }
}

/// Total damage of all casts. (`map` + `sum`)
pub fn total_damage(log: &[Cast]) -> u32 {
    todo!()
}

/// How many casts were critical hits? (`filter` + `count`)
pub fn critical_hits(log: &[Cast]) -> usize {
    todo!()
}

/// The single most damaging cast, or `None` for an empty log.
/// (`max_by_key`)
pub fn strongest_cast(log: &[Cast]) -> Option<&Cast> {
    todo!()
}

/// Average damage per cast, or `None` for an empty log.
pub fn average_damage(log: &[Cast]) -> Option<f64> {
    todo!()
}

/// The turn of the first cast that dealt at least `threshold` damage.
/// (`find` + `map`)
pub fn first_turn_with(log: &[Cast], threshold: u32) -> Option<u32> {
    todo!()
}

/// Total damage per spell name. (`fold` into a `HashMap`)
pub fn damage_by_spell(log: &[Cast]) -> HashMap<String, u32> {
    todo!()
}

/// The chronicle, numbered from 1, only for casts that dealt damage:
///   "1. Fireball (45)", "2. Frostbolt (35)!" ← a `!` marks a critical hit
/// (`filter` + `enumerate` + `map` + `collect`)
/// Note: the numbers count the *listed* casts, so filter first.
pub fn chronicle(log: &[Cast]) -> Vec<String> {
    todo!()
}

/// Pair every cast with the creature it hit, in order:
///   (spell name, target). Extra targets or casts are ignored.
/// (`zip`)
pub fn pair_with_targets<'a>(log: &'a [Cast], targets: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    todo!()
}

/// Did every cast in the log deal damage? (`all`)
/// And was there any critical Meteor? (`any`)
/// Returns (every_cast_hit, any_critical_meteor).
pub fn verdicts(log: &[Cast]) -> (bool, bool) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn battle() -> Vec<Cast> {
        vec![
            Cast::new("Fireball", 45, 1, false),
            Cast::new("Frostbolt", 35, 2, true),
            Cast::new("Heal", 0, 3, false),
            Cast::new("Fireball", 90, 4, true),
            Cast::new("Arcane Blast", 60, 5, false),
        ]
    }

    #[test]
    fn totals_and_crits() {
        assert_eq!(total_damage(&battle()), 230);
        assert_eq!(critical_hits(&battle()), 2);
        assert_eq!(total_damage(&[]), 0);
    }

    #[test]
    fn the_strongest_cast() {
        let log = battle();
        assert_eq!(strongest_cast(&log).map(|c| c.turn), Some(4));
        assert_eq!(strongest_cast(&[]), None);
    }

    #[test]
    fn averages() {
        assert_eq!(average_damage(&battle()), Some(46.0));
        assert_eq!(average_damage(&[]), None, "no casts, no average (and no division by zero!)");
    }

    #[test]
    fn first_big_hit() {
        assert_eq!(first_turn_with(&battle(), 50), Some(4));
        assert_eq!(first_turn_with(&battle(), 500), None);
    }

    #[test]
    fn damage_per_spell() {
        let by_spell = damage_by_spell(&battle());
        assert_eq!(by_spell.len(), 4);
        assert_eq!(by_spell["Fireball"], 135);
        assert_eq!(by_spell["Heal"], 0);
        assert_eq!(by_spell["Arcane Blast"], 60);
    }

    #[test]
    fn the_chronicle() {
        assert_eq!(
            chronicle(&battle()),
            vec![
                "1. Fireball (45)",
                "2. Frostbolt (35)!",
                "3. Fireball (90)!",
                "4. Arcane Blast (60)",
            ]
        );
    }

    #[test]
    fn pairing_casts_with_targets() {
        let log = battle();
        let pairs = pair_with_targets(&log, &["Goblin", "Wraith"]);
        assert_eq!(pairs, vec![("Fireball", "Goblin"), ("Frostbolt", "Wraith")]);
    }

    #[test]
    fn the_archmages_verdicts() {
        assert_eq!(verdicts(&battle()), (false, false));
        let meteor_day = vec![
            Cast::new("Fireball", 45, 1, false),
            Cast::new("Meteor", 150, 2, true),
        ];
        assert_eq!(verdicts(&meteor_day), (true, true));
    }
}
