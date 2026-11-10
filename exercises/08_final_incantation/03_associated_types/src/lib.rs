// ⚔️ Quest 8.3 — Associated Types: The Spell's Inner Essence
// 📖 Read: book/src/08_final_incantation/03_associated_types.md
//
// ❌ This file does not compile yet. Every `impl` below is missing its
// associated type (`type Output = …;` / `type Item = …;`). Add them, then
// replace every `todo!()`.

/// An enchantment turns raw power into *something* — and each enchantment
/// decides for itself what that something is.
pub trait Enchantment {
    /// What this enchantment produces.
    type Output;

    fn name(&self) -> &str;
    fn enchant(&self, power: u32) -> Self::Output;
}

/// Flame: pure damage, doubled. `Output = u32`
pub struct Flame;

/// Frost: `(damage, turns_slowed)` — half the damage (integer division),
/// but slows the target for `power / 10` turns. `Output = (u32, u32)`
pub struct Frost;

/// Insight: no damage at all, just knowledge.
/// `Output = String`, e.g. power 45 → "The runes whisper of 45 power"
pub struct Insight;

impl Enchantment for Flame {
    // TODO: type Output = …;

    fn name(&self) -> &str {
        "Flame"
    }
    fn enchant(&self, power: u32) -> Self::Output {
        todo!()
    }
}

impl Enchantment for Frost {
    // TODO: type Output = …;

    fn name(&self) -> &str {
        "Frost"
    }
    fn enchant(&self, power: u32) -> Self::Output {
        todo!()
    }
}

impl Enchantment for Insight {
    // TODO: type Output = …;

    fn name(&self) -> &str {
        "Insight"
    }
    fn enchant(&self, power: u32) -> Self::Output {
        todo!()
    }
}

/// Enchant every power with the same enchantment. Works for ANY enchantment —
/// the return type follows its `Output`.
pub fn enchant_all<E: Enchantment>(enchantment: &E, powers: &[u32]) -> Vec<E::Output> {
    todo!()
}

/// Mana regenerating over time: starts at `current`, gains `rate` per tick,
/// and stops once it reaches `max`.
///
/// Yields the mana after each tick, capped at `max`; the tick that reaches
/// `max` is the last one. Starting at or above `max` yields nothing.
/// `ManaRegen::new(70, 100, 12)` yields 82, 94, 100 — then `None`.
pub struct ManaRegen {
    current: u32,
    max: u32,
    rate: u32,
}

impl ManaRegen {
    pub fn new(current: u32, max: u32, rate: u32) -> Self {
        ManaRegen { current, max, rate }
    }
}

impl Iterator for ManaRegen {
    // TODO: type Item = …;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flame_doubles_damage() {
        let damage: u32 = Flame.enchant(45);
        assert_eq!(damage, 90);
    }

    #[test]
    fn frost_trades_damage_for_slowness() {
        let (damage, slowed): (u32, u32) = Frost.enchant(35);
        assert_eq!((damage, slowed), (17, 3));
    }

    #[test]
    fn insight_reveals_knowledge() {
        let vision: String = Insight.enchant(45);
        assert_eq!(vision, "The runes whisper of 45 power");
    }

    #[test]
    fn enchant_all_follows_the_output_type() {
        let flames: Vec<u32> = enchant_all(&Flame, &[10, 20]);
        assert_eq!(flames, vec![20, 40]);
        let frosts: Vec<(u32, u32)> = enchant_all(&Frost, &[60]);
        assert_eq!(frosts, vec![(30, 6)]);
        assert_eq!(enchant_all(&Insight, &[]), Vec::<String>::new());
    }

    #[test]
    fn mana_regenerates_up_to_the_cap() {
        let ticks: Vec<u32> = ManaRegen::new(70, 100, 12).collect();
        assert_eq!(ticks, vec![82, 94, 100]);
    }

    #[test]
    fn full_mana_regenerates_nothing() {
        assert_eq!(ManaRegen::new(100, 100, 10).next(), None);
    }

    #[test]
    fn regen_is_a_real_iterator() {
        // You only wrote `next` — every other adapter comes for free.
        let total: u32 = ManaRegen::new(0, 100, 25).take(2).sum();
        assert_eq!(total, 25 + 50);
        assert_eq!(ManaRegen::new(0, 100, 10).count(), 10);
        assert_eq!(ManaRegen::new(0, 100, 30).last(), Some(100));
    }
}
