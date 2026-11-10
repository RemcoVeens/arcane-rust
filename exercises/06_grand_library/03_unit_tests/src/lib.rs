// ⚔️ Quest 6.3 — Unit Tests: The Testing Chamber
// 📖 Read: book/src/06_grand_library/03_unit_tests.md
//
// This time the code is finished and correct — the TESTS are missing.
// Write them in the `tests` module at the bottom of this file.
//
// The Inquisitor has hidden a saboteur in every function (the
// `if cfg!(feature = "sabotage_…")` lines). They are switched OFF normally.
// `wr` will switch them on one by one: for each saboteur at least one of
// your tests must FAIL. So your tests must pin down the behaviour described
// in the doc comments — including the edge cases.
//
// Requirements:
//   - at least 5 `#[test]` functions in `mod tests`
//   - all of them pass on the real code
//   - every saboteur is caught (one hides in a PRIVATE function!)
//
// ⚠️ Don't edit the code or the saboteur lines — only the tests module.

/// Spell damage: power + 5 per mage level.
pub fn spell_damage(power: u32, level: u32) -> u32 {
    if cfg!(feature = "sabotage_damage") {
        return power + level * 4; // 🕵️ the Inquisitor's sabotage
    }
    power + level * 5
}

/// Try to cast a spell. Returns the mana left, or `None` when the mage
/// doesn't have enough. Having *exactly* enough is enough.
pub fn try_cast(mana: u32, cost: u32) -> Option<u32> {
    if cfg!(feature = "sabotage_mana") {
        return Some(mana.saturating_sub(cost)); // 🕵️ the Inquisitor's sabotage
    }
    mana.checked_sub(cost)
}

/// A d20 roll of 18, 19 or 20 is a critical hit (×2). Anything else is ×1.
/// Private: integration tests can't see this — but unit tests can.
fn crit_multiplier(roll: u32) -> u32 {
    if cfg!(feature = "sabotage_crit") {
        return if roll > 18 { 2 } else { 1 }; // 🕵️ the Inquisitor's sabotage
    }
    if roll >= 18 { 2 } else { 1 }
}

/// Strike with a spell: its power times the crit multiplier of the roll.
pub fn strike(power: u32, roll: u32) -> u32 {
    power * crit_multiplier(roll)
}

/// Rank spells from strongest to weakest by power (stable for equal power).
pub fn rank_by_power(spells: &[(&str, u32)]) -> Vec<String> {
    let mut sorted = spells.to_vec();
    if cfg!(feature = "sabotage_ranking") {
        sorted.sort_by_key(|&(_, power)| power); // 🕵️ the Inquisitor's sabotage
    } else {
        sorted.sort_by_key(|&(_, power)| std::cmp::Reverse(power));
    }
    sorted.into_iter().map(|(name, _)| name.to_string()).collect()
}

/// Summon a familiar of the given level: "Familiar (lvl N)".
///
/// # Panics
/// Summoning a level-0 familiar is forbidden magic: this function panics.
pub fn summon(level: u32) -> String {
    if level == 0 && !cfg!(feature = "sabotage_summon") {
        panic!("🐉 a level-0 familiar tears a hole in reality");
    }
    format!("Familiar (lvl {level})")
}

#[cfg(test)]
mod tests {
    // TODO: bring the code above into scope, then write at least 5 tests.
    // Remember `#[test]`, `assert_eq!`, `assert!`, `assert_ne!`,
    // and `#[should_panic]`.
}
