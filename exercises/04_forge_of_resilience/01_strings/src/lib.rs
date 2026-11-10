// ⚔️ Quest 4.1 — Strings: The Weave of Incantations
// 📖 Read: book/src/04_forge_of_resilience/01_strings.md
//
// Replace each `todo!()` with real code. Remember: a `char` is not a byte!

/// Greet a mage by name: `greet("Tiberius")` → `"Hail, Tiberius!"`
pub fn greet(name: &str) -> String {
    todo!()
}

/// Write a word onto a scroll. Words are separated by a single space,
/// but an empty scroll gets no leading space.
///
/// ""             + "ignis" → "ignis"
/// "ignis"        + "magna" → "ignis magna"
pub fn inscribe(scroll: &mut String, word: &str) {
    todo!()
}

/// How many *runes* (characters) does the word have?  `"ᚠᚢᚦ"` → 3
pub fn rune_count(word: &str) -> usize {
    todo!()
}

/// How many *bytes* does the word weigh in memory?  `"ᚠᚢᚦ"` → 9
pub fn byte_weight(word: &str) -> usize {
    todo!()
}

/// The first rune of a word, or `None` for an empty word.
pub fn first_rune(word: &str) -> Option<char> {
    todo!()
}

/// Counter-spells are spoken backwards: `"ignis"` → `"singi"`.
/// Must work for any Unicode text, e.g. `"ᚠᚢᚦ"` → `"ᚦᚢᚠ"`.
pub fn reverse_incantation(spell: &str) -> String {
    todo!()
}

/// Keep only the first `n` runes (characters!) of a spell name.
/// If the name is shorter, return it whole. Must never panic.
///
/// `truncate_runes("Fireball", 4)` → `"Fire"`, `truncate_runes("Ætherblast", 3)` → `"Æth"`
pub fn truncate_runes(spell: &str, n: usize) -> String {
    todo!()
}

/// Battle cry: trim surrounding whitespace and SHOUT it.
/// `"  for the academy! \n"` → `"FOR THE ACADEMY!"`
pub fn battle_cry(words: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_a_mage() {
        assert_eq!(greet("Tiberius"), "Hail, Tiberius!");
    }

    #[test]
    fn inscribing_words_on_a_scroll() {
        let mut scroll = String::new();
        inscribe(&mut scroll, "ignis");
        assert_eq!(scroll, "ignis", "no leading space on an empty scroll");
        inscribe(&mut scroll, "magna");
        assert_eq!(scroll, "ignis magna");
    }

    #[test]
    fn runes_are_not_bytes() {
        assert_eq!(rune_count("ᚠᚢᚦ"), 3);
        assert_eq!(byte_weight("ᚠᚢᚦ"), 9, "each of these runes takes 3 bytes in UTF-8");
        assert_eq!(rune_count("café"), 4);
        assert_eq!(byte_weight("café"), 5);
    }

    #[test]
    fn the_first_rune() {
        assert_eq!(first_rune("ᚱune"), Some('ᚱ'));
        assert_eq!(first_rune(""), None);
    }

    #[test]
    fn speaking_backwards() {
        assert_eq!(reverse_incantation("ignis"), "singi");
        assert_eq!(reverse_incantation("ᚠᚢᚦ"), "ᚦᚢᚠ");
    }

    #[test]
    fn truncating_by_runes_not_bytes() {
        assert_eq!(truncate_runes("Fireball", 4), "Fire");
        assert_eq!(truncate_runes("Ætherblast", 3), "Æth", "💥 slicing bytes would split 'Æ'");
        assert_eq!(truncate_runes("Heal", 10), "Heal");
    }

    #[test]
    fn shouting_battle_cries() {
        assert_eq!(battle_cry("  for the academy! \n"), "FOR THE ACADEMY!");
    }
}
