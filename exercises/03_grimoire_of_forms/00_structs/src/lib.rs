// ⚔️ Quest 3.0 — Structs: Crafting Vessels of Magic
// 📖 Read: book/src/03_grimoire_of_forms/00_structs.md
//
// ❌ This file does not compile yet: the `Potion` struct is missing.
// Define it, then replace each `todo!()`.

/// A mage, with named fields instead of an anonymous tuple.
/// `#[derive(...)]` asks the compiler to generate printing (`Debug`),
/// comparing (`PartialEq`) and `.clone()` (`Clone`) for us.
#[derive(Debug, Clone, PartialEq)]
pub struct Mage {
    pub name: String,
    pub level: u32,
    pub mana: u32,
    pub max_mana: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: String,
    pub cost: u32,
    pub power: u32,
}

// TODO: define `pub struct Potion` with two pub fields:
//   - `name`, a String
//   - `strength`, a u32
// and derive Debug, Clone and PartialEq, like the structs above.

/// A fresh apprentice: level 1, 100 mana out of 100.
/// Try the field init shorthand: `Mage { name, ... }`.
pub fn apprentice(name: &str) -> Mage {
    todo!()
}

/// Promotion: one level up, max_mana grows by 10 and the mana is refilled
/// to the new max. The name stays the same.
/// Try struct update syntax: `Mage { level: ..., ..mage }`.
pub fn promote(mage: Mage) -> Mage {
    todo!()
}

/// Brew a potion.
pub fn brew(name: &str, strength: u32) -> Potion {
    todo!()
}

/// Drink a potion: the mage's mana goes up by its strength, but never
/// above their own max_mana. The potion is only borrowed.
pub fn drink(mage: &mut Mage, potion: &Potion) {
    todo!()
}

/// "Tiberius (level 7) — mana 40/100"
pub fn describe(mage: &Mage) -> String {
    todo!()
}

/// Power per mana point. Try destructuring: `let Spell { cost, power, .. } = spell;`
/// A spell that costs 0 mana has an efficiency of 0.0.
pub fn efficiency(spell: &Spell) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiberius() -> Mage {
        Mage { name: String::from("Tiberius"), level: 7, mana: 40, max_mana: 100 }
    }

    #[test]
    fn a_new_apprentice() {
        let mage = apprentice("Morgana");
        assert_eq!(mage, Mage { name: String::from("Morgana"), level: 1, mana: 100, max_mana: 100 });
    }

    #[test]
    fn promotion_keeps_the_name() {
        let mage = promote(tiberius());
        assert_eq!(mage.name, "Tiberius");
        assert_eq!(mage.level, 8);
        assert_eq!(mage.max_mana, 110);
        assert_eq!(mage.mana, 110, "✦ promotion refills mana to the new max");
    }

    #[test]
    fn brewing_potions() {
        let potion = brew("Elixir of Focus", 50);
        assert_eq!(potion.name, "Elixir of Focus");
        assert_eq!(potion.strength, 50);
    }

    #[test]
    fn drinking_is_capped_at_max_mana() {
        let mut mage = tiberius();
        let potion = brew("Elixir of Focus", 50);
        drink(&mut mage, &potion);
        assert_eq!(mage.mana, 90);
        drink(&mut mage, &potion);
        assert_eq!(mage.mana, 100, "never above max_mana");
        assert_eq!(potion.strength, 50, "the potion was only borrowed");
    }

    #[test]
    fn describing_a_mage() {
        assert_eq!(describe(&tiberius()), "Tiberius (level 7) — mana 40/100");
    }

    #[test]
    fn spell_efficiency() {
        let fireball = Spell { name: String::from("Fireball"), cost: 30, power: 45 };
        assert_eq!(efficiency(&fireball), 1.5);
        let wink = Spell { name: String::from("Wink"), cost: 0, power: 1 };
        assert_eq!(efficiency(&wink), 0.0);
    }
}
