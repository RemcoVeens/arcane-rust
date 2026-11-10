// ⚔️ Quest 7.0 — Threads: The Summoning of Familiars
// 📖 Read: book/src/07_conclave_of_threads/00_threads.md
//
// Replace each `todo!()` with real code. Every function must do its work
// on spawned threads (`std::thread::spawn` or `std::thread::scope`).

use std::thread;

/// Summon one familiar (thread) per scroll. Each familiar counts the words
/// in its scroll (`split_whitespace`).
///
/// Return the counts in the SAME order as the scrolls were given — no matter
/// which familiar finishes first. (Hint: keep the `JoinHandle`s in order.)
pub fn count_words_in_parallel(scrolls: Vec<String>) -> Vec<usize> {
    todo!()
}

/// Summon `familiars` familiars. Familiar number `i` (counting from 0)
/// gathers `(i + 1) * 10` mana and returns it from its thread.
///
/// Return the total mana gathered by all familiars.
/// `gather_mana(3)` → 10 + 20 + 30 = 60
pub fn gather_mana(familiars: u32) -> u32 {
    todo!()
}

/// Split `powers` in two halves (`split_at(len / 2)`). One scoped thread sums
/// the left half, another sums the right half; return the grand total.
///
/// Use `thread::scope` — it lets the threads *borrow* `powers`
/// instead of needing ownership.
pub fn total_power_scoped(powers: &[u32]) -> u32 {
    todo!()
}

/// A risky ritual: a familiar computes `100 / divisor` on its own thread.
/// Dividing by zero panics *inside the familiar*.
///
/// Return `Some(result)` when the familiar survives, `None` when it panicked.
/// The calling thread must NOT panic.
pub fn risky_ritual(divisor: u32) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn familiars_count_words_in_order() {
        let scrolls = vec![
            "fire burns bright".to_string(),
            "frost".to_string(),
            "the arcane is vast and deep".to_string(),
        ];
        assert_eq!(
            count_words_in_parallel(scrolls),
            vec![3, 1, 6],
            "📜 results must come back in scroll order"
        );
    }

    #[test]
    fn no_scrolls_no_familiars() {
        assert_eq!(count_words_in_parallel(vec![]), Vec::<usize>::new());
    }

    #[test]
    fn three_familiars_gather_sixty_mana() {
        assert_eq!(gather_mana(3), 60);
    }

    #[test]
    fn ten_familiars_gather_a_lot() {
        assert_eq!(gather_mana(10), 550);
    }

    #[test]
    fn scoped_threads_sum_all_powers() {
        let powers = [45, 35, 60, 0, 150];
        assert_eq!(total_power_scoped(&powers), 290);
        // `powers` is still ours — scoped threads only borrowed it.
        assert_eq!(powers.len(), 5);
    }

    #[test]
    fn scoped_threads_handle_tiny_slices() {
        assert_eq!(total_power_scoped(&[]), 0);
        assert_eq!(total_power_scoped(&[7]), 7);
    }

    #[test]
    fn a_safe_ritual_returns_its_result() {
        assert_eq!(risky_ritual(4), Some(25));
    }

    #[test]
    fn a_panicking_familiar_does_not_take_the_mage_down() {
        assert_eq!(risky_ritual(0), None, "💥 join() returns Err when the thread panicked");
    }
}
