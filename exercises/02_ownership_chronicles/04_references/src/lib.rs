// ⚔️ Quest 2.4 — References: Borrowing the Grimoire
// 📖 Read: book/src/02_ownership_chronicles/04_references.md
//
// Replace each `todo!()`. Read through `&T`, write through `&mut T`.

pub const MAX_MANA: u32 = 100;

/// Count the characters in the grimoire without taking it.
/// (Yes, `&String` — Quest 2.6 shows why `&str` is better.)
#[allow(clippy::ptr_arg)] // deliberately `&String`, for now
pub fn rune_count(grimoire: &String) -> usize {
    todo!()
}

/// Return a NEW string: the spell in uppercase, followed by "!".
/// `shout(&String::from("Fireball"))` → "FIREBALL!"
#[allow(clippy::ptr_arg)]
pub fn shout(spell: &String) -> String {
    todo!()
}

/// Restore `amount` mana, never exceeding MAX_MANA.
/// You need to dereference: `*mana`.
pub fn recharge(mana: &mut u32, amount: u32) {
    todo!()
}

/// Write a rune at the end of the scroll, in place.
pub fn add_rune(scroll: &mut String, rune: char) {
    todo!()
}

/// Two mages swap mana pools.
pub fn swap_mana(a: &mut u32, b: &mut u32) {
    todo!()
}

/// Sum the circle's rune costs. The array is borrowed, not copied.
pub fn circle_cost(circle: &[u32; 5]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_does_not_take_the_grimoire() {
        let grimoire = String::from("ᚠᚢᚦ runes");
        assert_eq!(rune_count(&grimoire), 9);
        assert_eq!(grimoire, "ᚠᚢᚦ runes", "📖 still ours after lending it out");
    }

    #[test]
    fn shouting_a_spell() {
        let spell = String::from("Fireball");
        assert_eq!(shout(&spell), "FIREBALL!");
        assert_eq!(spell, "Fireball");
    }

    #[test]
    fn recharging_through_a_mutable_reference() {
        let mut mana = 40;
        recharge(&mut mana, 25);
        assert_eq!(mana, 65);
        recharge(&mut mana, 50);
        assert_eq!(mana, MAX_MANA, "✦ never above MAX_MANA");
    }

    #[test]
    fn writing_runes_in_place() {
        let mut scroll = String::from("ᚠᚢ");
        add_rune(&mut scroll, 'ᚦ');
        add_rune(&mut scroll, 'ᚨ');
        assert_eq!(scroll, "ᚠᚢᚦᚨ");
    }

    #[test]
    fn swapping_mana_pools() {
        let (mut tiberius, mut morgana) = (100, 15);
        swap_mana(&mut tiberius, &mut morgana);
        assert_eq!((tiberius, morgana), (15, 100));
    }

    #[test]
    fn summing_a_borrowed_circle() {
        let circle = [30, 25, 40, 20, 90];
        assert_eq!(circle_cost(&circle), 205);
    }
}
