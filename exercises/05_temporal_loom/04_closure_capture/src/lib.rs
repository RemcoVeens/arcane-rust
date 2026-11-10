// ⚔️ Quest 5.4 — Capturing: Borrow, Mutate, or Consume
// 📖 Read: book/src/05_temporal_loom/04_closure_capture.md
//
// ❌ This file does not compile yet.
// Three signatures promise the wrong closure trait (`Fn` / `FnMut` /
// `FnOnce`) or forget `move`. The tests are correct — fix the code.
// Then replace the `todo!()`s.

/// Channel a ritual `n` times.
/// The ritual is allowed to change the world around it (e.g. a counter).
pub fn channel<F: Fn()>(n: u32, ritual: F) {
    for _ in 0..n {
        ritual();
    }
}

/// Unseal a scroll: the closure *gives away* what it captured.
/// It will only ever be called once.
pub fn unseal<F: Fn() -> String>(seal: F) -> String {
    seal()
}

/// Return a closure that echoes the spell name, e.g. "Fireball! Fireball!"
pub fn spell_echo(name: String) -> impl Fn() -> String {
    || format!("{name}! {name}!")
}

/// A mana well that starts with `mana`. Each call draws `amount` from it
/// and returns how much was *actually* drawn (never more than what's left).
///
/// ```text
/// let mut well = mana_well(50);
/// well(30) → 30   (20 left)
/// well(30) → 20   (0 left)
/// well(30) → 0
/// ```
pub fn mana_well(mana: u32) -> impl FnMut(u32) -> u32 {
    todo!()
}

/// A spell cooldown. The returned closure is called once per turn and
/// returns `true` when the spell is ready. After returning `true` the
/// cooldown starts again.
///
/// With `turns = 2`:  false, true, false, true, ...
/// With `turns = 3`:  false, false, true, false, false, true, ...
pub fn cooldown(turns: u32) -> impl FnMut() -> bool {
    todo!()
}

/// A rune modifier: a boxed closure, so different closures can live in
/// one `Vec`.
pub type Rune = Box<dyn Fn(u32) -> u32>;

/// Three runes, in this order:
///   1. doubles the power
///   2. adds `bonus`
///   3. halves the power (integer division)
pub fn carve_runes(bonus: u32) -> Vec<Rune> {
    todo!()
}

/// Apply every rune in order to `power`.
pub fn apply_runes(runes: &[Rune], power: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channeling_a_ritual_that_counts() {
        let mut chants = 0;
        channel(3, || chants += 1);
        assert_eq!(chants, 3);
    }

    #[test]
    fn unsealing_gives_away_the_scroll() {
        let scroll = String::from("The Lost Incantation");
        let opened = unseal(move || scroll); // `scroll` moves OUT of the closure
        assert_eq!(opened, "The Lost Incantation");
    }

    #[test]
    fn echoes_outlive_their_name() {
        let echo = {
            let name = String::from("Fireball");
            spell_echo(name)
        };
        assert_eq!(echo(), "Fireball! Fireball!");
        assert_eq!(echo(), "Fireball! Fireball!", "a Fn closure can be called again");
    }

    #[test]
    fn drawing_from_the_mana_well() {
        let mut well = mana_well(50);
        assert_eq!(well(30), 30);
        assert_eq!(well(30), 20);
        assert_eq!(well(30), 0, "🕳️ the well is dry");
    }

    #[test]
    fn cooldowns_tick() {
        let mut ready = cooldown(3);
        let turns: Vec<bool> = (0..6).map(|_| ready()).collect();
        assert_eq!(turns, vec![false, false, true, false, false, true]);

        let mut quick = cooldown(1);
        assert!(quick() && quick(), "a 1-turn cooldown is always ready");
    }

    #[test]
    fn runes_apply_in_order() {
        let runes = carve_runes(10);
        assert_eq!(runes.len(), 3);
        assert_eq!(runes[0](7), 14);
        assert_eq!(runes[1](7), 17);
        assert_eq!(runes[2](7), 3);
        // (45 * 2 + 10) / 2 = 50
        assert_eq!(apply_runes(&runes, 45), 50);
        assert_eq!(apply_runes(&[], 45), 45, "no runes, no change");
    }
}
