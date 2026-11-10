// ⚔️ Quest 8.5 — Unsafe: The Forbidden Vault
// 📖 Read: book/src/08_final_incantation/05_unsafe.md
//
// Replace each `todo!()` with real code. Every `unsafe { }` block you write
// needs a `// SAFETY:` comment explaining why its contract holds.
// The functions marked "safe" must not be `unsafe fn`: they wrap the
// danger so callers can't misuse it.

/// Read the rune value behind a raw pointer.
///
/// # Safety
///
/// `rune` must be non-null, properly aligned, and point to an initialised
/// `u32` that stays valid for the duration of the call.
pub unsafe fn read_rune(rune: *const u32) -> u32 {
    todo!()
}

/// Engrave `value` into the slot behind a raw pointer.
///
/// # Safety
///
/// `slot` must be non-null, properly aligned, valid for writes, and no
/// other reference to that `u32` may be in use during the call.
pub unsafe fn engrave_rune(slot: *mut u32, value: u32) {
    todo!()
}

/// (safe) Swap two mana values using raw pointers (`std::ptr::swap`).
///
/// Two `&mut` can never alias, so the raw-pointer contract always holds.
pub fn swap_mana(a: &mut u32, b: &mut u32) {
    todo!()
}

/// (safe) The first rune, or `None` for an empty slice.
///
/// Check the length yourself, then use the unchecked accessor
/// `get_unchecked(0)` inside `unsafe`. (Yes, `runes.first()` would do — this
/// is practice for the "check, then trust" pattern.)
pub fn first_rune(runes: &[u32]) -> Option<u32> {
    todo!()
}

/// (safe) Split the ranks into two mutable halves `[0, mid)` and `[mid, len)`,
/// like the standard library's `split_at_mut` — but build it yourself from
/// `as_mut_ptr`, `ptr.add(mid)` and `std::slice::from_raw_parts_mut`.
///
/// # Panics
///
/// Panics if `mid > ranks.len()` (the message must contain "mid out of bounds").
pub fn split_ranks(ranks: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_a_rune_through_a_raw_pointer() {
        let rune: u32 = 5809;
        let ptr = &rune as *const u32;
        // SAFETY: `ptr` comes from a live reference.
        assert_eq!(unsafe { read_rune(ptr) }, 5809);
    }

    #[test]
    fn engraving_a_rune_through_a_raw_pointer() {
        let mut slot: u32 = 0;
        let ptr = &mut slot as *mut u32;
        // SAFETY: `ptr` comes from a live, exclusive reference.
        unsafe { engrave_rune(ptr, 42) };
        assert_eq!(slot, 42);
    }

    #[test]
    fn swapping_mana() {
        let (mut tiberius, mut morgana) = (30, 90);
        swap_mana(&mut tiberius, &mut morgana);
        assert_eq!((tiberius, morgana), (90, 30));
    }

    #[test]
    fn the_first_rune() {
        assert_eq!(first_rune(&[7, 8, 9]), Some(7));
        assert_eq!(first_rune(&[]), None, "an empty slice has no first rune — don't read past it!");
    }

    #[test]
    fn splitting_the_ranks() {
        let mut ranks = [1, 2, 3, 4, 5];
        let (front, back) = split_ranks(&mut ranks, 2);
        assert_eq!(front, &[1, 2]);
        assert_eq!(back, &[3, 4, 5]);
        // Both halves are mutable at the same time — that's the point.
        front[0] = 10;
        back[0] = 30;
        assert_eq!(ranks, [10, 2, 30, 4, 5]);
    }

    #[test]
    fn splitting_at_the_edges() {
        let mut ranks = [1, 2, 3];
        let (front, back) = split_ranks(&mut ranks, 0);
        assert!(front.is_empty());
        assert_eq!(back.len(), 3);
        let (front, back) = split_ranks(&mut ranks, 3);
        assert_eq!(front.len(), 3);
        assert!(back.is_empty());
    }

    #[test]
    #[should_panic(expected = "mid out of bounds")]
    fn splitting_past_the_end_panics() {
        let mut ranks = [1, 2, 3];
        let _ = split_ranks(&mut ranks, 4);
    }
}
