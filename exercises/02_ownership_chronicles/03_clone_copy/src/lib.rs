// ⚔️ Quest 2.3 — Clone & Copy: The Scribe's Duplicates
// 📖 Read: book/src/02_ownership_chronicles/03_clone_copy.md
//
// Replace each `todo!()`. This time cloning is allowed — where it's needed.

/// Make a deep copy of the scroll and return both: (original, copy).
pub fn duplicate(scroll: String) -> (String, String) {
    let dub = (scroll.clone(), scroll);
    dub
}

/// The copy is enchanted, the original stays pristine.
/// `enchant_copy(String::from("Fireball"), " +1")` → ("Fireball", "Fireball +1")
pub fn enchant_copy(original: String, enchantment: &str) -> (String, String) {
    let com = format!("{original}{enchantment}");
    (original, com)
}

/// Conjure `n` mirror images of a spell name.
/// `mirror_images(String::from("Blink"), 3)` → ["Blink", "Blink", "Blink"]
/// Start with `let mut images = Vec::new();` and `images.push(...)`.
/// Remember: a value moved inside a loop is gone on the next iteration.
pub fn mirror_images(spell: String, n: usize) -> Vec<String> {
    let mut res = Vec::new();
    for _ in 0..n {
        res.push(spell.clone());
    }
    res
}

/// A rune is a `char` — a Copy type. Return it twice. No `.clone()` needed!
pub fn rune_pair(rune: char) -> (char, char) {
    (rune, rune)
}

/// The Oracle's Quiz: for each type, is it `Copy`? Return your answers
/// as an array of bools, in this order:
///
///   0. i32
///   1. String
///   2. (u8, bool)
///   3. [u32; 3]
///   4. (String, u32)
///   5. &str
///   6. Vec<u32>
pub fn is_copy_quiz() -> [bool; 7] {
    [true, false, true, true, false, true, false]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicating_a_scroll() {
        let (a, b) = duplicate(String::from("Scroll of Light"));
        assert_eq!(a, "Scroll of Light");
        assert_eq!(b, "Scroll of Light");
    }

    #[test]
    fn copies_are_independent() {
        let (original, copy) = enchant_copy(String::from("Fireball"), " +1");
        assert_eq!(original, "Fireball", "🪞 the original must stay pristine");
        assert_eq!(copy, "Fireball +1");
    }

    #[test]
    fn mirror_images_are_clones() {
        assert_eq!(
            mirror_images(String::from("Blink"), 3),
            vec!["Blink", "Blink", "Blink"]
        );
        assert_eq!(
            mirror_images(String::from("Blink"), 0),
            Vec::<String>::new()
        );
    }

    #[test]
    fn runes_are_copy() {
        assert_eq!(rune_pair('ᚱ'), ('ᚱ', 'ᚱ'));
    }

    #[test]
    fn the_copy_quiz() {
        let answers = is_copy_quiz();
        let truth = [true, false, true, true, false, true, false];
        let names = [
            "i32",
            "String",
            "(u8, bool)",
            "[u32; 3]",
            "(String, u32)",
            "&str",
            "Vec<u32>",
        ];
        for i in 0..7 {
            assert_eq!(
                answers[i], truth[i],
                "🔮 is {} Copy? Think again.",
                names[i]
            );
        }
    }
}
