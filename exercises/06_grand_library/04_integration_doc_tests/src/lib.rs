// ⚔️ Quest 6.4 — Integration & Doc Tests: Trials from the Outside
// 📖 Read: book/src/06_grand_library/04_integration_doc_tests.md
//
// The arena code below is finished. Your job is to test it from the OUTSIDE:
//
//   1. ❌ `tests/arena.rs` does not compile: it was written as if it lived
//      inside this crate. Fix it so it uses the crate like any outsider would.
//   2. Grow `tests/arena.rs` to at least 5 integration tests.
//   3. Add at least 2 doc tests: ```` ``` ```` examples in the `///` comments
//      below (e.g. on `Duelist::new` and `duel`). One may be `should_panic`.
//
// `wr` counts your tests, so don't stop at "it compiles"!

/// A combatant in the arena.
#[derive(Debug, Clone, PartialEq)]
pub struct Duelist {
    pub name: String,
    pub hp: u32,
    pub armor: u32,
}

impl Duelist {
    /// A new duelist with `hp` hit points and no armor.
    ///
    /// # Panics
    /// A duelist must enter the arena alive: panics if `hp` is 0.
    pub fn new(name: &str, hp: u32) -> Duelist {
        assert!(hp > 0, "{name} cannot enter the arena with 0 HP");
        Duelist { name: name.to_string(), hp, armor: 0 }
    }

    /// The same duelist wearing `armor` points of armor.
    pub fn with_armor(mut self, armor: u32) -> Duelist {
        self.armor = armor;
        self
    }

    /// Take a hit. Armor absorbs part of it (see `damage_after_armor`);
    /// HP never drops below 0. Returns the damage actually taken.
    pub fn take_hit(&mut self, power: u32) -> u32 {
        let damage = damage_after_armor(power, self.armor).min(self.hp);
        self.hp -= damage;
        damage
    }

    /// Has this duelist fallen?
    pub fn is_defeated(&self) -> bool {
        self.hp == 0
    }
}

/// Armor absorbs damage point for point, but a hit always deals at least 1.
fn damage_after_armor(power: u32, armor: u32) -> u32 {
    power.saturating_sub(armor).max(1)
}

/// Fight until one duelist falls. `a` strikes first with `a_power`, then `b`
/// strikes back with `b_power`, and so on. Returns the winner's name.
pub fn duel(a: &mut Duelist, b: &mut Duelist, a_power: u32, b_power: u32) -> String {
    loop {
        b.take_hit(a_power);
        if b.is_defeated() {
            return a.name.clone();
        }
        a.take_hit(b_power);
        if a.is_defeated() {
            return b.name.clone();
        }
    }
}
