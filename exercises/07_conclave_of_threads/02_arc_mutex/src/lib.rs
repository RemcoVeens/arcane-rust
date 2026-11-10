// ⚔️ Quest 7.2 — Arc & Mutex: The Shared Mana Pool
// 📖 Read: book/src/07_conclave_of_threads/02_arc_mutex.md
//
// Replace each `todo!()` with real code.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// `mages` mages channel mana into ONE shared crystal, each on its own thread.
/// Every mage adds 1 mana, `per_mage` times (lock, add, unlock — in a loop).
///
/// Return the crystal's final charge. With correct locking it is always
/// exactly `mages * per_mage`.
pub fn channel_mana(mages: u32, per_mage: u32) -> u32 {
    todo!()
}

/// A mana pool shared by the whole conclave. Cloning a `ManaPool` gives
/// another handle to the SAME pool (that's what the `Arc` is for).
#[derive(Debug, Clone)]
pub struct ManaPool {
    mana: Arc<Mutex<u32>>,
}

impl ManaPool {
    /// A new pool holding `mana`.
    pub fn new(mana: u32) -> Self {
        todo!()
    }

    /// Try to draw `amount` mana. If the pool holds at least `amount`,
    /// subtract it and return `true`; otherwise leave the pool untouched and
    /// return `false`.
    ///
    /// The check and the subtraction must happen under ONE lock, or two mages
    /// could both see "enough" and overdraw the pool.
    pub fn draw(&self, amount: u32) -> bool {
        todo!()
    }

    /// Mana currently in the pool.
    pub fn remaining(&self) -> u32 {
        todo!()
    }
}

/// The guild's ledger: many readers, few writers → `RwLock`.
#[derive(Debug, Clone, Default)]
pub struct GuildLedger {
    gold: Arc<RwLock<HashMap<String, u32>>>,
}

impl GuildLedger {
    /// Add `gold` to `member`'s balance (members start at 0). Needs a write lock.
    pub fn deposit(&self, member: &str, gold: u32) {
        todo!()
    }

    /// The balance of `member` (0 if unknown). A read lock is enough.
    pub fn balance(&self, member: &str) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_crystal_receives_every_drop_of_mana() {
        assert_eq!(channel_mana(8, 1_000), 8_000, "🔮 lost updates? every add must hold the lock");
    }

    #[test]
    fn zero_mages_charge_nothing() {
        assert_eq!(channel_mana(0, 1_000), 0);
    }

    #[test]
    fn drawing_from_the_pool() {
        let pool = ManaPool::new(100);
        assert!(pool.draw(30));
        assert_eq!(pool.remaining(), 70);
        assert!(!pool.draw(90), "can't draw more than the pool holds");
        assert_eq!(pool.remaining(), 70, "a failed draw leaves the pool untouched");
    }

    #[test]
    fn clones_share_the_same_pool() {
        let pool = ManaPool::new(100);
        let other = pool.clone();
        assert!(other.draw(40));
        assert_eq!(pool.remaining(), 60);
    }

    #[test]
    fn ten_mages_rush_the_pool_but_only_three_fireballs_fit() {
        let pool = ManaPool::new(100);
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let pool = pool.clone();
                thread::spawn(move || pool.draw(30))
            })
            .collect();
        let successes = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .filter(|&ok| ok)
            .count();
        assert_eq!(successes, 3, "🔥 100 mana pays for exactly three 30-mana Fireballs");
        assert_eq!(pool.remaining(), 10);
    }

    #[test]
    fn the_ledger_tracks_deposits_from_many_threads() {
        let ledger = GuildLedger::default();
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let ledger = ledger.clone();
                thread::spawn(move || {
                    for _ in 0..25 {
                        ledger.deposit("Tiberius", 2);
                        ledger.deposit("Morgana", 1);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(ledger.balance("Tiberius"), 200);
        assert_eq!(ledger.balance("Morgana"), 100);
        assert_eq!(ledger.balance("Nobody"), 0);
    }
}
