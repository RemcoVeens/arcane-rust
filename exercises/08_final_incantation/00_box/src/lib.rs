// ⚔️ Quest 8.0 — Box: The Enchanted Chest
// 📖 Read: book/src/08_final_incantation/00_box.md
//
// ❌ This file does not compile yet. That's the first half of the quest:
// the `Combo` enum contains itself, so it would be infinitely large.
// Put the recursion in a `Box`. Then replace every `todo!()`.

/// A spell combo: a tree of spells.
#[derive(Debug, Clone, PartialEq)]
pub enum Combo {
    /// A single spell.
    Spell { name: String, power: u32 },
    /// Cast the left combo, then the right one: powers add up.
    Chain(Combo, Combo),
    /// Amplify a combo: its power is multiplied by the factor.
    Amplify(Combo, u32),
}

impl Combo {
    /// A single spell.
    pub fn spell(name: &str, power: u32) -> Combo {
        todo!()
    }

    /// `first` chained with `second`.
    pub fn chain(first: Combo, second: Combo) -> Combo {
        todo!()
    }

    /// `combo` amplified by `factor`.
    pub fn amplify(combo: Combo, factor: u32) -> Combo {
        todo!()
    }

    /// Total power of the combo tree (recursive!).
    pub fn power(&self) -> u32 {
        todo!()
    }

    /// How many single spells are in the tree.
    pub fn spell_count(&self) -> usize {
        todo!()
    }

    /// A readable formula:
    /// - Spell          → "Fireball"
    /// - Chain(a, b)    → "(a + b)"
    /// - Amplify(c, 3)  → "c x3"
    pub fn describe(&self) -> String {
        todo!()
    }
}

/// A protective ward absorbs part of incoming damage.
pub trait Ward {
    fn name(&self) -> &str;
    /// Damage left over after the ward absorbs its share.
    fn absorb(&self, damage: u32) -> u32;
}

/// Absorbs a flat 10 damage.
pub struct IceWard;

/// Absorbs half of the damage (rounding the remainder up: 45 → 23 left).
pub struct ArcaneShield;

impl Ward for IceWard {
    fn name(&self) -> &str {
        "Ice Ward"
    }
    fn absorb(&self, damage: u32) -> u32 {
        damage.saturating_sub(10)
    }
}

impl Ward for ArcaneShield {
    fn name(&self) -> &str {
        "Arcane Shield"
    }
    fn absorb(&self, damage: u32) -> u32 {
        damage.div_ceil(2)
    }
}

/// Forge a ward by name: "ice" → `IceWard`, "arcane" → `ArcaneShield`,
/// anything else → `None`. The caller doesn't know (or care) which
/// concrete type it gets back: it's a `Box<dyn Ward>`.
pub fn forge_ward(kind: &str) -> Option<Box<dyn Ward>> {
    todo!()
}

/// Damage passes through every ward in order.
pub fn damage_through(wards: &[Box<dyn Ward>], damage: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inferno() -> Combo {
        // ((Fireball + Frostbolt) x2 + Arcane Blast)
        Combo::chain(
            Combo::amplify(Combo::chain(Combo::spell("Fireball", 45), Combo::spell("Frostbolt", 35)), 2),
            Combo::spell("Arcane Blast", 60),
        )
    }

    #[test]
    fn a_single_spell_is_a_combo() {
        let fb = Combo::spell("Fireball", 45);
        assert_eq!(fb.power(), 45);
        assert_eq!(fb.spell_count(), 1);
        assert_eq!(fb.describe(), "Fireball");
    }

    #[test]
    fn combos_nest() {
        assert_eq!(inferno().power(), (45 + 35) * 2 + 60);
        assert_eq!(inferno().spell_count(), 3);
    }

    #[test]
    fn combos_describe_themselves() {
        assert_eq!(inferno().describe(), "((Fireball + Frostbolt) x2 + Arcane Blast)");
    }

    #[test]
    fn a_hundred_sparks_chain_together() {
        let mut combo = Combo::spell("Spark", 1);
        for _ in 0..100 {
            combo = Combo::chain(combo, Combo::spell("Spark", 1));
        }
        assert_eq!(combo.power(), 101);
        assert_eq!(combo.spell_count(), 101);
    }

    #[test]
    fn wards_are_forged_by_name() {
        assert_eq!(forge_ward("ice").map(|w| w.name().to_string()), Some("Ice Ward".to_string()));
        assert_eq!(forge_ward("arcane").map(|w| w.absorb(45)), Some(23));
        assert!(forge_ward("cheese").is_none(), "🧀 no such ward");
    }

    #[test]
    fn damage_passes_through_every_ward() {
        let wards: Vec<Box<dyn Ward>> = vec![forge_ward("ice").unwrap(), forge_ward("arcane").unwrap()];
        assert_eq!(damage_through(&wards, 90), 40, "90 → ice 80 → arcane 40");
        assert_eq!(damage_through(&[], 90), 90);
    }
}
