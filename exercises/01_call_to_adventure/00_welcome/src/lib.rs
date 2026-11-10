// ⚔️ Quest 1.0 — The Call
// 📖 Read: book/src/01_call_to_adventure/00_welcome.md
//
// Every Rust Mage starts by speaking the Oath.
// The tests at the bottom of this file are the Gatekeepers. They pass only
// when the oath is spoken *exactly* right.
//
// Run `wr` from the root of the repository to check your work.

/// Returns the oath of the Arcane Academy.
pub fn oath() -> &'static str {
    // TODO: return exactly "I seek the Arcane"
    "I seek... uh... snacks?"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_oath_is_spoken() {
        assert_eq!(
            oath(),
            "I seek the Arcane",
            "🚪 The Gatekeeper frowns: that is not the oath."
        );
    }
}
