// ⚔️ Quest 8.2 — Weak: The Ouroboros of Memory
// 📖 Read: book/src/08_final_incantation/02_weak_cycles.md
//
// Replace each `todo!()` with real code.
// Parents own their children (`Rc`). Children only *remember* their parent
// (`Weak`) — otherwise parent and child would keep each other alive forever.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A guild in the great tree of guilds.
#[derive(Debug)]
pub struct Guild {
    pub name: String,
    parent: RefCell<Weak<Guild>>,
    children: RefCell<Vec<Rc<Guild>>>,
}

impl Guild {
    /// A new guild with no parent and no children.
    pub fn new(name: &str) -> Rc<Guild> {
        todo!()
    }

    /// Make `child` a sub-guild of `parent`:
    /// - the child remembers its parent as a `Weak` (`Rc::downgrade`)
    /// - the parent owns the child (`Rc`) in its children list
    pub fn add_child(parent: &Rc<Guild>, child: Rc<Guild>) {
        todo!()
    }

    /// The parent guild, if it still exists (`Weak::upgrade`).
    pub fn parent(&self) -> Option<Rc<Guild>> {
        todo!()
    }

    /// Names of the direct sub-guilds, in the order they were added.
    pub fn child_names(&self) -> Vec<String> {
        todo!()
    }

    /// Names from this guild up to the root:
    /// Pyromancers → ["Pyromancers", "Mages", "Grand Order"]
    pub fn lineage(&self) -> Vec<String> {
        todo!()
    }

    /// This guild plus all guilds below it (recursively).
    pub fn size(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Grand Order
    /// ├── Mages
    /// │   ├── Pyromancers
    /// │   └── Cryomancers
    /// └── Knights
    fn order() -> (Rc<Guild>, Rc<Guild>, Rc<Guild>) {
        let root = Guild::new("Grand Order");
        let mages = Guild::new("Mages");
        let pyro = Guild::new("Pyromancers");
        Guild::add_child(&root, Rc::clone(&mages));
        Guild::add_child(&root, Guild::new("Knights"));
        Guild::add_child(&mages, Rc::clone(&pyro));
        Guild::add_child(&mages, Guild::new("Cryomancers"));
        (root, mages, pyro)
    }

    #[test]
    fn a_lone_guild() {
        let g = Guild::new("Hermits");
        assert!(g.parent().is_none());
        assert!(g.child_names().is_empty());
        assert_eq!(g.size(), 1);
    }

    #[test]
    fn children_know_their_parent() {
        let (_root, mages, pyro) = order();
        assert_eq!(pyro.parent().map(|p| p.name.clone()), Some("Mages".to_string()));
        assert_eq!(mages.child_names(), vec!["Pyromancers", "Cryomancers"]);
    }

    #[test]
    fn lineage_climbs_to_the_root() {
        let (root, _mages, pyro) = order();
        assert_eq!(pyro.lineage(), vec!["Pyromancers", "Mages", "Grand Order"]);
        assert_eq!(root.lineage(), vec!["Grand Order"]);
    }

    #[test]
    fn size_counts_the_whole_subtree() {
        let (root, mages, _pyro) = order();
        assert_eq!(root.size(), 5);
        assert_eq!(mages.size(), 3);
    }

    #[test]
    fn children_do_not_keep_parents_alive() {
        let (root, mages, _pyro) = order();
        assert_eq!(Rc::strong_count(&root), 1, "♻️ only our `root` variable owns the root");
        assert_eq!(Rc::weak_count(&root), 2, "two children point back with Weak");
        assert_eq!(Rc::strong_count(&mages), 2, "our variable + the root's children list");
    }

    #[test]
    fn when_the_root_falls_the_orphans_know() {
        let (root, mages, pyro) = order();
        drop(root);
        assert!(mages.parent().is_none(), "the Grand Order is gone — upgrade() returns None");
        assert_eq!(pyro.lineage(), vec!["Pyromancers", "Mages"]);
    }
}
