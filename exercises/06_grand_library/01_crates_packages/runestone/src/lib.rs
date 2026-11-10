//! # Runestone
//!
//! An ancient library for carving and reading runes. It is a separate crate,
//! so everything you use from it must be `pub`.

/// The Elder Futhark, one rune per Latin letter a..=z (approximately!).
const FUTHARK: [char; 26] = [
    'ᚨ', 'ᛒ', 'ᚲ', 'ᛞ', 'ᛖ', 'ᚠ', 'ᚷ', 'ᚺ', 'ᛁ', 'ᛃ', 'ᚴ', 'ᛚ', 'ᛗ', 'ᚾ', 'ᛟ', 'ᛈ', 'ᛩ',
    'ᚱ', 'ᛊ', 'ᛏ', 'ᚢ', 'ᚡ', 'ᚹ', 'ᛪ', 'ᛦ', 'ᛉ',
];

/// A carved stone: an inscription plus the power it holds.
#[derive(Debug, Clone, PartialEq)]
pub struct Runestone {
    inscription: String,
    power: u32,
}

impl Runestone {
    /// Carve `text` into a new stone. Every letter adds 3 power.
    pub fn carve(text: &str) -> Runestone {
        Runestone {
            inscription: text.to_string(),
            power: rune_count(text) as u32 * 3,
        }
    }

    /// The text carved into the stone.
    pub fn inscription(&self) -> &str {
        &self.inscription
    }

    /// The power stored in the stone.
    pub fn power(&self) -> u32 {
        self.power
    }
}

/// The number of letters (alphabetic chars) in `text`.
pub fn rune_count(text: &str) -> usize {
    text.chars().filter(|c| c.is_alphabetic()).count()
}

/// Glyph translation lives in its own module: `runestone::glyphs`.
pub mod glyphs {
    use super::FUTHARK;

    /// Translate one ASCII letter (any case) into its rune.
    /// Anything else (spaces, digits, …) passes through unchanged.
    pub fn to_glyph(c: char) -> char {
        if c.is_ascii_alphabetic() {
            let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            FUTHARK[index]
        } else {
            c
        }
    }
}
