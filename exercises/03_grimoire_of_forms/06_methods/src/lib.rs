// ⚔️ Quest 3.6 — Methods: Breathing Life into Forms
// 📖 Read: book/src/03_grimoire_of_forms/06_methods.md
//
// Replace each `todo!()`. Pay attention to the receiver of every method:
// `&self` reads, `&mut self` changes, `self` consumes.

pub const STARTING_MANA: u32 = 100;

#[derive(Debug, Clone, PartialEq)]
pub struct Mage {
    pub name: String,
    pub level: u32,
    pub mana: u32,
    pub max_mana: u32,
}

impl Mage {
    /// Associated function (no `self`): a level-1 mage with
    /// STARTING_MANA out of STARTING_MANA. Called as `Mage::new("Tiberius")`.
    pub fn new(name: &str) -> Self {
        todo!()
    }

    /// Is there enough mana for a spell of this cost?
    pub fn can_cast(&self, cost: u32) -> bool {
        todo!()
    }

    /// Spend the mana if possible (reuse `can_cast`!). Returns whether
    /// the spell was cast.
    pub fn cast(&mut self, cost: u32) -> bool {
        todo!()
    }

    /// Refill mana to max_mana.
    pub fn rest(&mut self) {
        todo!()
    }

    /// One level up: +1 level, +10 max_mana, mana refilled.
    pub fn level_up(&mut self) {
        todo!()
    }

    /// "Tiberius the Apprentice" (level 1–4), "… the Adept" (5–9),
    /// "… the Archmage" (10+).
    pub fn title(&self) -> String {
        todo!()
    }

    /// The mage retires — this consumes the mage, nobody can use it after.
    /// Returns "Tiberius retires at level 7."
    pub fn retire(self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
}

impl Element {
    /// All elements, in order. An associated function.
    pub fn all() -> [Element; 3] {
        todo!()
    }

    /// Fire beats Frost, Frost beats Arcane, Arcane beats Fire.
    /// Element is Copy, so taking `self` by value is fine (and idiomatic).
    pub fn beats(self, other: Element) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summoning_a_new_mage() {
        let mage = Mage::new("Tiberius");
        assert_eq!(mage, Mage { name: String::from("Tiberius"), level: 1, mana: 100, max_mana: 100 });
    }

    #[test]
    fn casting_spends_mana() {
        let mut mage = Mage::new("Tiberius");
        assert!(mage.can_cast(90));
        assert!(mage.cast(90));
        assert_eq!(mage.mana, 10);
        assert!(!mage.can_cast(30));
        assert!(!mage.cast(30), "not enough mana");
        assert_eq!(mage.mana, 10, "a failed cast spends nothing");
        mage.rest();
        assert_eq!(mage.mana, 100);
    }

    #[test]
    fn leveling_up() {
        let mut mage = Mage::new("Tiberius");
        mage.cast(50);
        mage.level_up();
        assert_eq!((mage.level, mage.mana, mage.max_mana), (2, 110, 110));
    }

    #[test]
    fn titles() {
        let mut mage = Mage::new("Tiberius");
        assert_eq!(mage.title(), "Tiberius the Apprentice");
        for _ in 0..4 {
            mage.level_up();
        }
        assert_eq!(mage.title(), "Tiberius the Adept");
        for _ in 0..5 {
            mage.level_up();
        }
        assert_eq!(mage.title(), "Tiberius the Archmage");
    }

    #[test]
    fn retiring_consumes_the_mage() {
        let mut mage = Mage::new("Tiberius");
        for _ in 0..6 {
            mage.level_up();
        }
        assert_eq!(mage.retire(), "Tiberius retires at level 7.");
    }

    #[test]
    fn the_wheel_of_elements() {
        assert_eq!(Element::all(), [Element::Fire, Element::Frost, Element::Arcane]);
        assert!(Element::Fire.beats(Element::Frost));
        assert!(Element::Frost.beats(Element::Arcane));
        assert!(Element::Arcane.beats(Element::Fire));
        assert!(!Element::Frost.beats(Element::Fire));
        assert!(!Element::Fire.beats(Element::Fire));
    }
}
