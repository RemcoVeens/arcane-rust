// ⚔️ Quest 7.1 — Channels: The Whispering Stones
// 📖 Read: book/src/07_conclave_of_threads/01_channels.md
//
// Replace each `todo!()` with real code using `std::sync::mpsc`.
// ⚠️ If a test hangs forever, a Sender is still alive somewhere — the
// receiver keeps waiting for a message that can never come.

use std::sync::mpsc::{self, Receiver};
use std::thread;

/// A single familiar chants the given words, one message per word,
/// UPPERCASED, through a channel. The mage (calling thread) receives them
/// with a `for` loop over the receiver and returns them.
///
/// One producer → the messages arrive in the order they were sent.
pub fn chant(words: Vec<String>) -> Vec<String> {
    todo!()
}

/// Every scout `(name, goblins_seen)` runs on its own thread and sends its
/// report `(name, goblins_seen)` through ONE shared channel
/// (clone the `Sender` for each scout).
///
/// Collect all reports and return them **sorted by scout name** — threads
/// finish in any order, so sorting makes the result predictable.
pub fn scout_reports(scouts: Vec<(String, u32)>) -> Vec<(String, u32)> {
    todo!()
}

/// Total goblins seen by all scouts. Reuse `scout_reports`, or build a new
/// channel — your call. Iterator methods are welcome.
pub fn total_goblins(scouts: Vec<(String, u32)>) -> u32 {
    todo!()
}

/// Read every message that is *already waiting* in the inbox, without
/// blocking for new ones (`try_recv` / `try_iter`).
pub fn drain_inbox(inbox: &Receiver<String>) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scouts() -> Vec<(String, u32)> {
        vec![
            ("Wren".to_string(), 4),
            ("Aldric".to_string(), 7),
            ("Mira".to_string(), 0),
            ("Bram".to_string(), 12),
        ]
    }

    #[test]
    fn a_single_familiar_chants_in_order() {
        let words = vec!["ignis".to_string(), "glacies".to_string(), "arcanum".to_string()];
        assert_eq!(chant(words), vec!["IGNIS", "GLACIES", "ARCANUM"]);
    }

    #[test]
    fn an_empty_chant_still_ends() {
        assert_eq!(chant(vec![]), Vec::<String>::new());
    }

    #[test]
    fn every_scout_reports_back() {
        assert_eq!(
            scout_reports(scouts()),
            vec![
                ("Aldric".to_string(), 7),
                ("Bram".to_string(), 12),
                ("Mira".to_string(), 0),
                ("Wren".to_string(), 4),
            ],
            "🧭 all four reports, sorted by name"
        );
    }

    #[test]
    fn goblins_are_counted() {
        assert_eq!(total_goblins(scouts()), 23);
    }

    #[test]
    fn draining_reads_what_is_waiting() {
        let (tx, rx) = mpsc::channel();
        tx.send("A goblin approaches".to_string()).unwrap();
        tx.send("Another goblin approaches".to_string()).unwrap();
        // `tx` is still alive here: a blocking `recv` loop would hang forever!
        assert_eq!(
            drain_inbox(&rx),
            vec!["A goblin approaches", "Another goblin approaches"]
        );
        assert_eq!(drain_inbox(&rx), Vec::<String>::new(), "📭 the inbox is empty now");
        drop(tx);
    }

    #[test]
    fn senders_can_live_on_other_threads() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || tx.send("Wraith sighted".to_string()).unwrap())
            .join()
            .unwrap();
        assert_eq!(drain_inbox(&rx), vec!["Wraith sighted"]);
    }
}
