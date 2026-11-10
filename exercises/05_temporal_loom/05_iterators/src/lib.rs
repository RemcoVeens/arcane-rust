// ⚔️ Quest 5.5 — Iterators: The River of Values
// 📖 Read: book/src/05_temporal_loom/05_iterators.md
//
// ❌ This file does not compile yet: the `Iterator` impls are missing.
// Implement the trait for each struct below (only `next` is required!),
// then replace the `todo!()`s.

#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub power: u32,
}

impl Spell {
    pub fn new(name: &str, cost: u32, power: u32) -> Self {
        Spell { name: name.to_string(), cost, power }
    }
}

/// A ritual countdown: yields `from`, `from - 1`, ..., `1`, then stops.
pub struct Countdown {
    pub remaining: u32,
}

// TODO: impl Iterator for Countdown { type Item = u32; fn next(...) ... }

/// Meditation: every turn the mage regains `regen` mana, never above `max`.
/// Yields the mana *after* each turn, and stops once the mana is full.
///
/// `ManaRegen { mana: 50, regen: 20, max: 100 }` yields 70, 90, 100.
/// A mage who is already full yields nothing.
pub struct ManaRegen {
    pub mana: u32,
    pub regen: u32,
    pub max: u32,
}

// TODO: impl Iterator for ManaRegen

/// Cycles through a borrowed spell rotation forever:
/// A, B, C, A, B, C, ... (an empty rotation yields nothing).
/// The items are `&'a Spell` — borrowed from the rotation, not cloned.
pub struct Rotation<'a> {
    pub spells: &'a [Spell],
    pub next_index: usize,
}

impl<'a> Rotation<'a> {
    pub fn new(spells: &'a [Spell]) -> Self {
        Rotation { spells, next_index: 0 }
    }
}

// TODO: impl<'a> Iterator for Rotation<'a>

/// The even-numbered turns from 2 up to and including `last_turn`.
/// Return an iterator — not a Vec! Ranges are iterators too; look up `step_by`.
pub fn even_turns(last_turn: u32) -> impl Iterator<Item = u32> {
    todo!()
}

/// Every spell in the book gets +10 power.
/// Use a `for` loop over mutable references (`iter_mut()` or `&mut`).
pub fn empower_all(spells: &mut [Spell]) {
    todo!()
}

/// Consume the spellbook and return the names only.
/// Use a `for` loop that takes ownership (`into_iter()` or plain `for x in vec`).
pub fn into_names(spells: Vec<Spell>) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotation() -> Vec<Spell> {
        vec![
            Spell::new("Fireball", 30, 45),
            Spell::new("Frostbolt", 25, 35),
            Spell::new("Heal", 20, 0),
        ]
    }

    #[test]
    fn counting_down_by_hand() {
        let mut countdown = Countdown { remaining: 3 };
        assert_eq!(countdown.next(), Some(3));
        assert_eq!(countdown.next(), Some(2));
        assert_eq!(countdown.next(), Some(1));
        assert_eq!(countdown.next(), None);
        assert_eq!(countdown.next(), None, "a finished countdown stays finished");
    }

    #[test]
    fn countdowns_work_in_for_loops() {
        let mut chant = String::new();
        for n in (Countdown { remaining: 4 }) {
            chant.push_str(&n.to_string());
        }
        assert_eq!(chant, "4321");
    }

    #[test]
    fn regenerating_mana() {
        let turns: Vec<u32> = ManaRegen { mana: 50, regen: 20, max: 100 }.collect();
        assert_eq!(turns, vec![70, 90, 100]);
        let full: Vec<u32> = ManaRegen { mana: 100, regen: 20, max: 100 }.collect();
        assert!(full.is_empty(), "a full mage doesn't meditate");
    }

    #[test]
    fn rotations_cycle_forever() {
        let spells = rotation();
        let names: Vec<&str> = Rotation::new(&spells)
            .take(7)
            .map(|s| s.name.as_str())
            .collect();
        assert_eq!(
            names,
            vec!["Fireball", "Frostbolt", "Heal", "Fireball", "Frostbolt", "Heal", "Fireball"]
        );
        assert_eq!(Rotation::new(&[]).next(), None);
    }

    #[test]
    fn even_turns_are_lazy() {
        let turns: Vec<u32> = even_turns(9).collect();
        assert_eq!(turns, vec![2, 4, 6, 8]);
        assert_eq!(even_turns(10).last(), Some(10));
    }

    #[test]
    fn empowering_in_place() {
        let mut spells = rotation();
        empower_all(&mut spells);
        assert_eq!(spells[0].power, 55);
        assert_eq!(spells[2].power, 10);
    }

    #[test]
    fn consuming_the_book() {
        assert_eq!(into_names(rotation()), vec!["Fireball", "Frostbolt", "Heal"]);
    }
}
