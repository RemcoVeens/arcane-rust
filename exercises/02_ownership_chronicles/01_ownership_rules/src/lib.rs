// ⚔️ Quest 2.1 — The Ownership Rules: Three Laws of Magic
// 📖 Read: book/src/02_ownership_chronicles/01_ownership_rules.md
//
// Replace each `todo!()`. No references (`&String`, `&mut`) yet —
// in this quest, values are *given* and *given back*.

/// Forge a new staff and hand it to the caller: "Staff of <element>".
/// The `String` is created here, and returning it moves ownership out.
pub fn forge_staff(element: &str) -> String {
    todo!()
}

/// Take ownership of a scroll, write a rune at its end, and give it back.
/// `inscribe(String::from("ᚠᚢ"), 'ᚦ')` → "ᚠᚢᚦ"
/// Hint: a parameter can be declared `mut scroll: String`.
pub fn inscribe(scroll: String, rune: char) -> String {
    todo!()
}

/// Weigh a scroll (its length in bytes) and give it back, so the caller
/// still owns it: return `(scroll, weight)`.
pub fn weigh(scroll: String) -> (String, usize) {
    todo!()
}

/// Two mages merge their incantations: "<first> <second>".
/// Both Strings are moved in; one new String comes out.
pub fn merge(first: String, second: String) -> String {
    todo!()
}

/// A treasure chest. When the chest (the function) ends, everything it
/// owns is dropped. Build the String "gold", then return only its length —
/// the String itself is dropped when the function returns.
pub fn count_and_drop() -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forging_a_staff() {
        assert_eq!(forge_staff("Embers"), "Staff of Embers");
    }

    #[test]
    fn inscribing_returns_the_scroll() {
        let scroll = String::from("ᚠᚢ");
        let scroll = inscribe(scroll, 'ᚦ');
        assert_eq!(scroll, "ᚠᚢᚦ");
    }

    #[test]
    fn weighing_gives_the_scroll_back() {
        let scroll = String::from("Scroll of Mending");
        let (scroll, weight) = weigh(scroll);
        assert_eq!(weight, 17);
        assert_eq!(scroll, "Scroll of Mending", "📜 you should get your scroll back intact");
    }

    #[test]
    fn merging_incantations() {
        let a = String::from("Ignis");
        let b = String::from("Maxima");
        assert_eq!(merge(a, b), "Ignis Maxima");
    }

    #[test]
    fn the_chest_counts_its_gold() {
        assert_eq!(count_and_drop(), 4);
    }
}
