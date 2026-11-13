// ⚔️ Quest 1.1 — Variables: The Immutable Truth
// 📖 Read: book/src/01_call_to_adventure/01_variables.md
//
// ❌ This file does not compile yet. That's the quest!
// Read the compiler error carefully — it tells you exactly what's wrong.

/// A mage starts with 50 mana, rests (+25), then meditates (+25).
/// Returns the final mana.
pub fn rest_and_meditate() -> u32 {
    let mut mana = 50;
    mana = mana + 25; // rest
    mana += 25; // meditate
    mana
}

/// Returns the length of the mage's name *and* the power of their staff.
/// Fix the scoping problem — `staff_power` must be visible where it's used.
pub fn mage_stats() -> (usize, u32) {
    let name = "Tiberius";
    let staff_power = 42;
    (name.len(), staff_power)
}

/// `_` silences unused-variable warnings. Keep the warning away,
/// but still return the number of scrolls in the bag (3).
pub fn count_scrolls() -> u32 {
    let _ = 'ᚱ';
    let scrolls = 3;
    scrolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resting_restores_mana_to_one_hundred() {
        assert_eq!(rest_and_meditate(), 100);
    }

    #[test]
    fn mage_stats_are_known() {
        assert_eq!(mage_stats(), (8, 42));
    }

    #[test]
    fn three_scrolls_in_the_bag() {
        assert_eq!(count_scrolls(), 3);
    }
}
