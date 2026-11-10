// ⚔️ Quest 4.7 — Generics: The Universal Incantations
// 📖 Read: book/src/04_forge_of_resilience/07_generics.md
//
// Replace each `todo!()`. Some signatures are missing trait bounds on purpose —
// once you write the body, the compiler will tell you which bound to add.

/// The largest item in the slice, or `None` if it is empty.
/// Works for mana (`u32`), damage multipliers (`f64`), runes (`char`)…
pub fn strongest<T>(items: &[T]) -> Option<T> {
    todo!()
}

/// A copy of `item`, `n` times: `echo(&"ignis", 3)` → `["ignis", "ignis", "ignis"]`.
pub fn echo<T>(item: &T, n: usize) -> Vec<T> {
    todo!()
}

/// A pouch that can hold any kind of thing — gems, scrolls, spells.
/// Last in, first out.
#[derive(Debug)]
pub struct Pouch<T> {
    items: Vec<T>,
}

impl<T> Pouch<T> {
    /// An empty pouch.
    pub fn new() -> Self {
        todo!()
    }

    /// Put an item in the pouch.
    pub fn stash(&mut self, item: T) {
        todo!()
    }

    /// Take the most recently stashed item out.
    pub fn take(&mut self) -> Option<T> {
        todo!()
    }

    /// Look at the most recently stashed item without taking it.
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    /// How many items are in the pouch?
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Is the pouch empty?
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

/// A duel between two contenders of the same type.
#[derive(Debug, PartialEq)]
pub struct Duel<T> {
    pub left: T,
    pub right: T,
}

impl<T> Duel<T> {
    /// Swap sides: the left contender goes right and vice versa.
    pub fn swap(self) -> Duel<T> {
        todo!()
    }
}

// TODO: add a SECOND impl block that only exists when `T` can be compared,
//       with a method `pub fn winner(&self) -> &T` returning the larger side
//       (the left side wins ties).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strongest_works_for_any_comparable_type() {
        assert_eq!(strongest(&[45, 150, 35]), Some(150));
        assert_eq!(strongest(&[1.5, 2.0, 0.5]), Some(2.0));
        assert_eq!(strongest(&['ᚠ', 'ᚱ', 'ᚢ']), Some('ᚱ'));
        assert_eq!(strongest::<u32>(&[]), None);
    }

    #[test]
    fn echoing_anything() {
        assert_eq!(echo(&"ignis", 3), vec!["ignis", "ignis", "ignis"]);
        assert_eq!(echo(&String::from("ᚱ"), 2), vec!["ᚱ".to_string(), "ᚱ".to_string()]);
        assert!(echo(&7, 0).is_empty());
    }

    #[test]
    fn a_pouch_of_gems() {
        let mut pouch = Pouch::new();
        assert!(pouch.is_empty());
        pouch.stash("ruby");
        pouch.stash("sapphire");
        assert_eq!(pouch.len(), 2);
        assert_eq!(pouch.peek(), Some(&"sapphire"));
        assert_eq!(pouch.take(), Some("sapphire"));
        assert_eq!(pouch.take(), Some("ruby"));
        assert_eq!(pouch.take(), None);
    }

    #[test]
    fn a_pouch_of_scrolls() {
        let mut pouch: Pouch<String> = Pouch::new();
        pouch.stash("Fireball".to_string());
        assert_eq!(pouch.peek(), Some(&"Fireball".to_string()));
        assert_eq!(pouch.len(), 1);
    }

    #[test]
    fn swapping_sides() {
        let duel = Duel { left: "Tiberius", right: "Goblin" };
        assert_eq!(duel.swap(), Duel { left: "Goblin", right: "Tiberius" });
    }

    #[test]
    fn the_stronger_side_wins() {
        assert_eq!(*Duel { left: 45, right: 60 }.winner(), 60);
        assert_eq!(*Duel { left: 2.5, right: 1.0 }.winner(), 2.5);
        assert_eq!(*Duel { left: 7, right: 7 }.winner(), 7);
    }
}
