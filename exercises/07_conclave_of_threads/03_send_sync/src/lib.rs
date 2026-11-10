// ⚔️ Quest 7.3 — Send & Sync: The Pacts of Passage
// 📖 Read: book/src/07_conclave_of_threads/03_send_sync.md
//
// ❌ This file does not compile yet. That's the quest!
// Four rituals break the Pacts of Passage. Read each compiler error — look
// for "cannot be sent between threads safely" and "may outlive" — and fix
// the code with the smallest change that keeps the behavior.
// Don't change the function signatures or the tests.

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

/// A familiar carries a message to a distant tower.
#[derive(Debug, Clone)]
pub struct Familiar {
    pub name: Rc<str>,
    pub message: String,
}

impl Familiar {
    pub fn new(name: &str, message: &str) -> Self {
        Familiar { name: name.into(), message: message.to_string() }
    }
}

/// Ritual 1: send the familiar to another thread, which returns
/// "<name> delivers: <message>".
pub fn dispatch(familiar: Familiar) -> String {
    thread::spawn(move || format!("{} delivers: {}", familiar.name, familiar.message))
        .join()
        .unwrap()
}

/// Ritual 2: every apprentice (one thread per budget) reads the SAME shared
/// spell list and counts the spells whose cost is <= its budget.
pub fn affordable_counts(spells: Vec<(String, u32)>, budgets: Vec<u32>) -> Vec<usize> {
    let spells = Rc::new(spells);
    let handles: Vec<_> = budgets
        .into_iter()
        .map(|budget| {
            let spells = Rc::clone(&spells);
            thread::spawn(move || spells.iter().filter(|(_, cost)| *cost <= budget).count())
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

/// Ritual 3: each of `mages` threads adds `amount` to a shared offering.
pub fn pooled_offering(mages: u32, amount: u32) -> u32 {
    let total = Arc::new(0u32);
    let handles: Vec<_> = (0..mages)
        .map(|_| {
            let total = Arc::clone(&total);
            thread::spawn(move || {
                *total += amount;
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    *total
}

/// Ritual 4: a herald thread proclaims the archmage's title.
pub fn herald(name: &str) -> String {
    let title = format!("Archmage {name}");
    let handle = thread::spawn(|| format!("All hail {title}!"));
    handle.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn familiars_may_cross_between_threads() {
        assert_send::<Familiar>();
        assert_sync::<Familiar>();
    }

    #[test]
    fn the_familiar_delivers() {
        let f = Familiar::new("Corvus", "the goblins are coming");
        assert_eq!(dispatch(f), "Corvus delivers: the goblins are coming");
    }

    #[test]
    fn apprentices_share_one_spell_list() {
        let spells = vec![
            ("Fireball".to_string(), 30),
            ("Frostbolt".to_string(), 25),
            ("Arcane Blast".to_string(), 40),
            ("Heal".to_string(), 20),
            ("Meteor".to_string(), 90),
        ];
        assert_eq!(affordable_counts(spells, vec![10, 25, 40, 100]), vec![0, 2, 4, 5]);
    }

    #[test]
    fn the_offering_counts_every_coin() {
        assert_eq!(pooled_offering(10, 7), 70);
    }

    #[test]
    fn the_herald_proclaims() {
        assert_eq!(herald("Tiberius"), "All hail Archmage Tiberius!");
    }
}
