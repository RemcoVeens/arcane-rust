// ⚔️ Quest 6.5 — Documentation: The Archivist's Scrolls
// 📖 Read: book/src/06_grand_library/05_documentation.md
//
// ❌ This crate does not compile yet: `#![deny(missing_docs)]` turns every
// undocumented public item into an error. The Archivist demands:
//
//   1. A crate-level doc comment (`//!`) right below the `#![deny]` line.
//   2. A `///` doc comment on EVERY public item: types, fields, variants,
//      functions, methods, constants.
//   3. The existing example on `Scroll::new` is broken — fix it.
//   4. The intra-doc link on `readable_by` points at nothing — fix it.
//   5. At least 3 doc tests in total (examples in ``` blocks).
//
// `wr` runs `cargo test`, `cargo test --doc` and
// `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps`.
// When it passes, admire your work: `cargo doc -p ch06_05_documentation --open`

#![deny(missing_docs)]

pub const MAX_LEVEL: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum School {
    Evocation,
    Abjuration,
    Necromancy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scroll {
    pub title: String,
    pub level: u32,
    pub school: School,
}

impl Scroll {
    /// Write a new scroll. Levels above [`MAX_LEVEL`] are capped.
    ///
    /// ```
    /// use ch06_05_documentation::Scroll;
    ///
    /// let scroll = Scroll::new("Fireball", 1);
    /// assert_eq!(scroll.level, 2);
    /// ```
    pub fn new(title: &str, level: u32, school: School) -> Scroll {
        Scroll {
            title: title.to_string(),
            level: level.min(MAX_LEVEL),
            school,
        }
    }

    pub fn is_forbidden(&self) -> bool {
        self.school == School::Necromancy || self.level == MAX_LEVEL
    }
}

/// The scrolls a mage of `mage_level` may read: level at most `mage_level`
/// and not forbidden (see [`Scroll::is_forbiden`]).
pub fn readable_by(scrolls: &[Scroll], mage_level: u32) -> Vec<&Scroll> {
    scrolls
        .iter()
        .filter(|s| s.level <= mage_level && !s.is_forbidden())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_capped() {
        assert_eq!(Scroll::new("Meteor", 99, School::Evocation).level, MAX_LEVEL);
    }

    #[test]
    fn necromancy_is_forbidden() {
        assert!(Scroll::new("Raise Dead", 3, School::Necromancy).is_forbidden());
        assert!(!Scroll::new("Shield", 3, School::Abjuration).is_forbidden());
    }

    #[test]
    fn apprentices_read_simple_scrolls() {
        let library = vec![
            Scroll::new("Fireball", 1, School::Evocation),
            Scroll::new("Arcane Blast", 5, School::Evocation),
            Scroll::new("Raise Dead", 1, School::Necromancy),
        ];
        let titles: Vec<&str> = readable_by(&library, 3).iter().map(|s| s.title.as_str()).collect();
        assert_eq!(titles, vec!["Fireball"]);
    }
}
