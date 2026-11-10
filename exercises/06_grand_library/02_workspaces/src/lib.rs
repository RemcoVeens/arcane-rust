// ⚔️ Quest 6.2 — Workspaces: The Citadel of Many Towers
// 📖 Read: book/src/06_grand_library/02_workspaces.md
//
// ❌ This crate does not compile yet.
//
// It depends on a second crate of the workspace: `arcana_core/` (right next
// to this file's `src/`). That crate's public API is incomplete, and its
// manifest ignores the workspace settings. Fix `arcana_core` so that:
//   1. `arcana_core::Element` can be imported (hint: `pub use`),
//   2. `ManaCrystal::drain` can be called from this crate,
//   3. `arcana_core::VERSION` equals the workspace version (0.1.0),
// then implement the `todo!()`s below. Don't edit the tests.

use arcana_core::{Element, ManaCrystal};

/// Cast a spell powered by a crystal. Drain `cost` mana from it; the cast
/// succeeds only when the crystal could pay the full cost.
/// (A failed cast still drains what was there — crystals are unforgiving.)
pub fn cast_from_crystal(crystal: &mut ManaCrystal, cost: u32) -> bool {
    crystal.drain(cost) == cost
}

/// Which element should you cast against an enemy of `enemy` element?
/// The one that is strong against it: e.g. Frost enemies → Fire.
pub fn counter_element(enemy: Element) -> Element {
    todo!()
}

/// Report which version of the core this tower was built with:
/// "arcana_core v<VERSION>"
pub fn core_banner() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crystals_pay_for_spells() {
        let mut crystal = ManaCrystal::new(50);
        assert!(cast_from_crystal(&mut crystal, 30));
        assert_eq!(crystal.charge(), 20);
        assert!(!cast_from_crystal(&mut crystal, 30), "💎 only 20 left");
        assert_eq!(crystal.charge(), 0);
    }

    #[test]
    fn countering_elements() {
        assert_eq!(counter_element(Element::Frost), Element::Fire);
        assert_eq!(counter_element(Element::Arcane), Element::Frost);
        assert_eq!(counter_element(Element::Fire), Element::Arcane);
    }

    #[test]
    fn the_core_shares_the_workspace_version() {
        assert_eq!(
            arcana_core::VERSION,
            "0.1.0",
            "🏰 arcana_core should inherit `version` from [workspace.package]"
        );
        assert_eq!(core_banner(), "arcana_core v0.1.0");
    }
}
