// ⚔️ Quest 5.3 — Closures: Incantations on the Fly
// 📖 Read: book/src/05_temporal_loom/03_closures.md
//
// Replace each `todo!()`. Every solution here is short — most are a
// single line with a closure `|x| ...` in it.

#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub power: u32,
    pub level: u32,
}

impl Spell {
    pub fn new(name: &str, cost: u32, power: u32, level: u32) -> Self {
        Spell { name: name.to_string(), cost, power, level }
    }
}

/// The Academy's standard spellbook.
pub fn spellbook() -> Vec<Spell> {
    vec![
        Spell::new("Fireball", 30, 45, 1),
        Spell::new("Frostbolt", 25, 35, 2),
        Spell::new("Arcane Blast", 40, 60, 5),
        Spell::new("Heal", 20, 0, 1),
        Spell::new("Meteor", 90, 150, 10),
    ]
}

/// Apply the enchantment `f` to `power` twice: `f(f(power))`.
pub fn apply_twice<F: Fn(u32) -> u32>(f: F, power: u32) -> u32 {
    todo!()
}

/// Cast `spell` and let `modifier` decide the damage.
/// The modifier receives the spell and returns the damage dealt.
pub fn cast_with<F>(spell: &Spell, modifier: F) -> u32
where
    F: Fn(&Spell) -> u32,
{
    todo!()
}

/// Return a closure that multiplies its input by `factor`.
/// (The closure must own `factor` — look up `move`.)
pub fn make_amplifier(factor: u32) -> impl Fn(u32) -> u32 {
    todo!()
}

/// Sort spells from the cheapest to the most expensive (by cost).
/// Hint: `sort_by_key` takes a closure that picks the key.
pub fn sort_by_cost(spells: &mut [Spell]) {
    todo!()
}

/// Remove every spell the mage cannot cast: cost above `mana`, or level
/// above `level`.
/// Hint: `Vec::retain` keeps the elements for which the closure returns `true`.
pub fn forget_uncastable(spells: &mut Vec<Spell>, mana: u32, level: u32) {
    todo!()
}

/// The power of the spell named `name`, doubled — or 0 if there is no such
/// spell. Use `Option::map` and `unwrap_or` instead of `match`.
pub fn doubled_power(spells: &[Spell], name: &str) -> u32 {
    let found: Option<&Spell> = spells.iter().find(|s| s.name == name);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enchanting_twice() {
        assert_eq!(apply_twice(|p| p + 10, 45), 65);
        assert_eq!(apply_twice(|p| p * 3, 2), 18);
    }

    #[test]
    fn casting_with_modifiers() {
        let fireball = Spell::new("Fireball", 30, 45, 1);
        assert_eq!(cast_with(&fireball, |s| s.power), 45);
        assert_eq!(cast_with(&fireball, |s| s.power * 2 + s.level), 91);

        // closures can borrow from their surroundings
        let full_moon_bonus = 15;
        assert_eq!(cast_with(&fireball, |s| s.power + full_moon_bonus), 60);
    }

    #[test]
    fn amplifiers_remember_their_factor() {
        let triple = make_amplifier(3);
        let tenfold = make_amplifier(10);
        assert_eq!(triple(45), 135);
        assert_eq!(tenfold(6), 60);
        assert_eq!(apply_twice(triple, 2), 18, "amplifiers are closures too");
    }

    #[test]
    fn sorting_by_cost() {
        let mut spells = spellbook();
        sort_by_cost(&mut spells);
        let costs: Vec<u32> = spells.iter().map(|s| s.cost).collect();
        assert_eq!(costs, vec![20, 25, 30, 40, 90]);
    }

    #[test]
    fn forgetting_spells_you_cannot_cast() {
        let mut spells = spellbook();
        forget_uncastable(&mut spells, 35, 3);
        let names: Vec<&str> = spells.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["Fireball", "Frostbolt", "Heal"]);
    }

    #[test]
    fn doubling_power_or_fizzling() {
        let spells = spellbook();
        assert_eq!(doubled_power(&spells, "Arcane Blast"), 120);
        assert_eq!(doubled_power(&spells, "Summon Pizza"), 0, "🫧 the spell fizzles");
    }
}
