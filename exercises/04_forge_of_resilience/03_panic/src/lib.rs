// ⚔️ Quest 4.3 — Panic!: When Magic Goes Critical
// 📖 Read: book/src/04_forge_of_resilience/03_panic.md
//
// Replace each `todo!()` with real code, then fix the test marked TODO.

/// Split the loot evenly over the party (integer division).
///
/// A party of 0 is a *bug in the caller* — panic with a message containing
/// "empty party".
pub fn split_loot(gold: u32, party_size: u32) -> u32 {
    todo!()
}

/// Read the spell on `page`. The caller promises the page exists, so plain
/// indexing is fine — an out-of-range page panics on its own.
pub fn read_page(book: &[String], page: usize) -> &str {
    todo!()
}

/// Channel `cost` mana out of `mana`.
///
/// Our game logic guarantees there is always enough mana when this is called.
/// If that guarantee is ever broken, the spell backfires: panic with a message
/// containing "backfire". Hint: `checked_sub` + `expect`.
pub fn channel(mana: u32, cost: u32) -> u32 {
    todo!()
}

/// A mana crystal holds a charge between 0 and 100 (inclusive).
/// Charging it past 100 makes it explode: use `assert!` so it panics with
/// a message containing "overcharged".
/// Returns the new charge.
pub fn charge_crystal(charge: u32, extra: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loot_is_shared() {
        assert_eq!(split_loot(100, 3), 33);
    }

    #[test]
    #[should_panic(expected = "empty party")]
    fn nobody_to_share_with() {
        split_loot(100, 0);
    }

    #[test]
    fn reading_a_page() {
        let book = vec!["Fireball".to_string(), "Heal".to_string()];
        assert_eq!(read_page(&book, 1), "Heal");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn reading_a_page_that_does_not_exist() {
        let book = vec!["Fireball".to_string(), "Heal".to_string()];
        read_page(&book, 7);
    }

    #[test]
    fn channelling_mana() {
        assert_eq!(channel(100, 30), 70);
    }

    #[test]
    #[should_panic(expected = "backfire")]
    fn channelling_too_much_backfires() {
        channel(10, 30);
    }

    #[test]
    fn charging_a_crystal() {
        assert_eq!(charge_crystal(60, 40), 100);
    }

    // TODO: this test is *supposed* to panic. Tell the test harness so,
    //       and make sure it only accepts the "overcharged" panic.
    #[test]
    fn overcharged_crystal_explodes() {
        charge_crystal(90, 20);
    }
}
