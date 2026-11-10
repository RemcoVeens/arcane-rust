// ⚔️ Quest 4.9 — Trait Objects: The Shapeshifter's Art
// 📖 Read: book/src/04_forge_of_resilience/09_trait_objects.md
//
// Replace each `todo!()`. `Goblin` is done for you as an example.

pub trait Creature {
    fn name(&self) -> &str;
    fn hp(&self) -> u32;
    /// How hard this creature hits.
    fn attack(&self) -> u32;
    /// Lose `amount` hp (never below 0).
    fn take_damage(&mut self, amount: u32);

    fn is_alive(&self) -> bool {
        self.hp() > 0
    }
}

pub struct Goblin {
    pub hp: u32,
}

impl Creature for Goblin {
    fn name(&self) -> &str {
        "Goblin"
    }
    fn hp(&self) -> u32 {
        self.hp
    }
    fn attack(&self) -> u32 {
        5
    }
    fn take_damage(&mut self, amount: u32) {
        self.hp = self.hp.saturating_sub(amount);
    }
}

/// A Wraith is half made of shadow: it only takes HALF the damage (rounded down).
/// Name "Wraith", attack 12.
pub struct Wraith {
    pub hp: u32,
}

impl Creature for Wraith {
    fn name(&self) -> &str {
        todo!()
    }
    fn hp(&self) -> u32 {
        todo!()
    }
    fn attack(&self) -> u32 {
        todo!()
    }
    fn take_damage(&mut self, amount: u32) {
        todo!()
    }
}

/// A Dragon attacks for 20 — or 40 once it is enraged (below 100 hp).
/// Name "Dragon".
pub struct Dragon {
    pub hp: u32,
}

impl Creature for Dragon {
    fn name(&self) -> &str {
        todo!()
    }
    fn hp(&self) -> u32 {
        todo!()
    }
    fn attack(&self) -> u32 {
        todo!()
    }
    fn take_damage(&mut self, amount: u32) {
        todo!()
    }
}

/// The horde, in this order: Goblin (30 hp), Wraith (40 hp), Dragon (200 hp).
pub fn spawn_horde() -> Vec<Box<dyn Creature>> {
    todo!()
}

/// `"Goblin (30 hp)"`, or `"Goblin (defeated)"` once it has 0 hp.
pub fn describe(creature: &dyn Creature) -> String {
    todo!()
}

/// The total attack of every creature that is still alive.
pub fn total_threat(horde: &[Box<dyn Creature>]) -> u32 {
    todo!()
}

/// A fireball hits every creature in the horde for `damage`.
pub fn fireball_all(horde: &mut [Box<dyn Creature>], damage: u32) {
    todo!()
}

/// The names of the creatures that are still alive, in horde order.
pub fn survivors(horde: &[Box<dyn Creature>]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraiths_take_half_damage() {
        let mut wraith = Wraith { hp: 40 };
        wraith.take_damage(45);
        assert_eq!(wraith.hp(), 18, "👻 45 / 2 = 22 damage, 40 - 22 = 18");
        wraith.take_damage(100);
        assert_eq!(wraith.hp(), 0);
        assert!(!wraith.is_alive());
        assert_eq!((wraith.name(), wraith.attack()), ("Wraith", 12));
    }

    #[test]
    fn dragons_become_enraged() {
        let mut dragon = Dragon { hp: 200 };
        assert_eq!((dragon.name(), dragon.attack()), ("Dragon", 20));
        dragon.take_damage(150);
        assert_eq!(dragon.hp(), 50);
        assert_eq!(dragon.attack(), 40, "🐉 below 100 hp the dragon is enraged");
    }

    #[test]
    fn spawning_a_mixed_horde() {
        let horde = spawn_horde();
        assert_eq!(horde.len(), 3);
        assert_eq!(horde[0].name(), "Goblin");
        assert_eq!(horde[1].name(), "Wraith");
        assert_eq!((horde[2].name(), horde[2].hp()), ("Dragon", 200));
    }

    #[test]
    fn describing_any_creature() {
        assert_eq!(describe(&Goblin { hp: 30 }), "Goblin (30 hp)");
        assert_eq!(describe(&Wraith { hp: 0 }), "Wraith (defeated)");
    }

    #[test]
    fn the_threat_of_the_horde() {
        let horde = spawn_horde();
        assert_eq!(total_threat(&horde), 5 + 12 + 20);
    }

    #[test]
    fn a_fireball_hits_everyone() {
        let mut horde = spawn_horde();
        fireball_all(&mut horde, 45);
        assert_eq!(horde[0].hp(), 0, "goblin");
        assert_eq!(horde[1].hp(), 18, "wraith takes half");
        assert_eq!(horde[2].hp(), 155, "dragon");
        assert_eq!(survivors(&horde), vec!["Wraith", "Dragon"]);
        assert_eq!(total_threat(&horde), 12 + 20, "dead goblins don't bite");
    }
}
