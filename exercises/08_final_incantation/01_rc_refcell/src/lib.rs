// ⚔️ Quest 8.1 — Rc & RefCell: The Shared Satchel
// 📖 Read: book/src/08_final_incantation/01_rc_refcell.md
//
// Replace each `todo!()` with real code.
// Notice: `Adventurer`'s methods take `&self`, yet they change the satchel.
// That's interior mutability.

use std::cell::RefCell;
use std::rc::Rc;

/// The party's one and only satchel.
#[derive(Debug, Default, PartialEq)]
pub struct Satchel {
    pub items: Vec<String>,
    pub gold: u32,
}

/// Many owners (`Rc`), mutable on demand (`RefCell`).
pub type SharedSatchel = Rc<RefCell<Satchel>>;

/// A new shared satchel holding `gold` and no items.
pub fn new_satchel(gold: u32) -> SharedSatchel {
    todo!()
}

/// An adventurer carries a handle to the party's satchel.
#[derive(Debug)]
pub struct Adventurer {
    pub name: String,
    satchel: SharedSatchel,
}

impl Adventurer {
    /// Join the party: keep a clone of the `Rc` (not of the satchel!).
    pub fn join(name: &str, satchel: &SharedSatchel) -> Self {
        todo!()
    }

    /// Put an item into the shared satchel.
    pub fn loot(&self, item: &str) {
        todo!()
    }

    /// Pay `amount` gold from the shared satchel. Returns `false` (and pays
    /// nothing) if there isn't enough gold.
    pub fn pay(&self, amount: u32) -> bool {
        todo!()
    }

    /// Take the first item called `item` out of the satchel, if it's there.
    pub fn take(&self, item: &str) -> Option<String> {
        todo!()
    }

    /// How many items are in the satchel right now.
    pub fn item_count(&self) -> usize {
        todo!()
    }
}

/// How many handles to this satchel exist (the satchel's own `Rc` plus one
/// per adventurer still holding it).
pub fn holders(satchel: &SharedSatchel) -> usize {
    todo!()
}

/// Peek at the items without panicking: if someone is *currently* holding a
/// mutable borrow, return `None` instead (`try_borrow`).
pub fn peek_items(satchel: &SharedSatchel) -> Option<Vec<String>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_whole_party_shares_one_satchel() {
        let satchel = new_satchel(100);
        let tiberius = Adventurer::join("Tiberius", &satchel);
        let morgana = Adventurer::join("Morgana", &satchel);

        tiberius.loot("Goblin Ear");
        morgana.loot("Wraith Essence");

        assert_eq!(tiberius.item_count(), 2, "🎒 Tiberius sees Morgana's loot too");
        assert_eq!(satchel.borrow().items, vec!["Goblin Ear", "Wraith Essence"]);
    }

    #[test]
    fn paying_spends_shared_gold() {
        let satchel = new_satchel(100);
        let tiberius = Adventurer::join("Tiberius", &satchel);
        let morgana = Adventurer::join("Morgana", &satchel);
        assert!(tiberius.pay(60));
        assert!(!morgana.pay(60), "only 40 gold left");
        assert!(morgana.pay(40));
        assert_eq!(satchel.borrow().gold, 0);
    }

    #[test]
    fn taking_items_out() {
        let satchel = new_satchel(0);
        let bram = Adventurer::join("Bram", &satchel);
        bram.loot("Potion");
        bram.loot("Rope");
        assert_eq!(bram.take("Potion"), Some("Potion".to_string()));
        assert_eq!(bram.take("Potion"), None);
        assert_eq!(bram.item_count(), 1);
    }

    #[test]
    fn counting_holders() {
        let satchel = new_satchel(0);
        assert_eq!(holders(&satchel), 1);
        let a = Adventurer::join("Wren", &satchel);
        {
            let _b = Adventurer::join("Aldric", &satchel);
            assert_eq!(holders(&satchel), 3);
        }
        assert_eq!(holders(&satchel), 2, "Aldric left the party: his handle was dropped");
        drop(a);
        assert_eq!(holders(&satchel), 1);
    }

    #[test]
    fn peeking_is_polite() {
        let satchel = new_satchel(5);
        satchel.borrow_mut().items.push("Map".to_string());
        assert_eq!(peek_items(&satchel), Some(vec!["Map".to_string()]));

        let busy = satchel.borrow_mut(); // someone is rummaging…
        assert_eq!(peek_items(&satchel), None, "🙈 try_borrow fails instead of panicking");
        drop(busy);
        assert!(peek_items(&satchel).is_some());
    }
}
