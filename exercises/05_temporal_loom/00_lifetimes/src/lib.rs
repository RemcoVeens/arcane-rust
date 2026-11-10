// ⚔️ Quest 5.0 — Lifetimes: The Thread of Existence
// 📖 Read: book/src/05_temporal_loom/00_lifetimes.md
//
// ❌ This file does not compile yet. That's the quest!
// The compiler cannot tell how long the returned references live.
// Tell it — with lifetime annotations — or stop returning references.

/// Two mages argue over whose incantation is mightier. The longer one wins.
/// On a tie, the first incantation wins.
///
/// The returned `&str` points into one of the two inputs.
pub fn longest_incantation(first: &str, second: &str) -> &str {
    if second.len() > first.len() { second } else { first }
}

/// Mark a foe as the target of the next spell.
///
/// Returns the foe's name (with surrounding whitespace trimmed) when the
/// battle cry is not empty, or `""` when the mage stays silent.
///
/// The result only ever points into `foe` (or is a literal) — never into
/// `battle_cry`. Your annotations must say exactly that: one of the tests
/// drops the battle cry long before it uses the result.
pub fn mark_target(foe: &str, battle_cry: &str) -> &str {
    if battle_cry.is_empty() { "" } else { foe.trim() }
}

/// Summon the name of the Academy's champion: "Tiberius".
///
/// This one tries to return a reference to a `String` that dies at the end
/// of the function. No lifetime annotation can save a dangling reference —
/// change what the function returns instead.
pub fn summon_champion() -> &str {
    let name = String::from("Tiberius");
    &name
}

/// The element a rune belongs to. Runes outside 1..=3 are "void".
/// String literals live for the whole program: `&'static str`.
///   1 → "fire", 2 → "frost", 3 → "arcane", anything else → "void"
pub fn rune_element(rune: u8) -> &str {
    match rune {
        1 => "fire",
        2 => "frost",
        3 => "arcane",
        _ => "void",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_longer_incantation_wins() {
        assert_eq!(longest_incantation("Ignis", "Ignis Maxima"), "Ignis Maxima");
        assert_eq!(longest_incantation("Glacius Eternum", "Frost"), "Glacius Eternum");
        assert_eq!(longest_incantation("Lux", "Nox"), "Lux", "on a tie the first one wins");
    }

    #[test]
    fn the_result_lives_as_long_as_both_inputs() {
        let first = String::from("Arcanum");
        let winner;
        {
            let second = String::from("Arcanum Supremus");
            winner = longest_incantation(&first, &second);
            assert_eq!(winner, "Arcanum Supremus");
        } // `second` dies here — and so does `winner`.
    }

    #[test]
    fn marking_a_target() {
        assert_eq!(mark_target("  Goblin  ", "For the Academy!"), "Goblin");
        assert_eq!(mark_target("Wraith", ""), "", "a silent mage marks no one");
    }

    #[test]
    fn the_mark_outlives_the_battle_cry() {
        let foe = String::from("Ancient Dragon");
        let target;
        {
            let cry = String::from("Burn!");
            target = mark_target(&foe, &cry);
        } // the battle cry fades...
        // ...but the mark remains, because it only borrows from `foe`.
        assert_eq!(target, "Ancient Dragon");
    }

    #[test]
    fn the_champion_is_summoned() {
        assert_eq!(summon_champion(), "Tiberius");
    }

    #[test]
    fn rune_elements_live_forever() {
        let element: &'static str = rune_element(1);
        assert_eq!(element, "fire");
        assert_eq!(rune_element(2), "frost");
        assert_eq!(rune_element(3), "arcane");
        assert_eq!(rune_element(42), "void");
    }
}
