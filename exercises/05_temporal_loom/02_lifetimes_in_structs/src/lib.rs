// ⚔️ Quest 5.2 — Lifetimes in Structs: The Crystal Ball
// 📖 Read: book/src/05_temporal_loom/02_lifetimes_in_structs.md
//
// ❌ This file does not compile yet: the structs hold references,
// but don't say how long those references live.
// Add the lifetime parameters, then replace every `todo!()`.

/// Reads an ancient scroll line by line, *without copying it*.
/// It borrows the scroll's text and remembers how far it has read.
pub struct ScrollReader {
    text: &str,
    /// Byte position of the next unread character.
    pos: usize,
}

impl ScrollReader {
    /// Start reading at the very beginning of `text`.
    pub fn new(text: &str) -> Self {
        todo!()
    }

    /// The next line of the scroll (without the trailing `'\n'`),
    /// or `None` once everything has been read.
    ///
    /// ⚠️ The returned line borrows from the *scroll*, not from the reader:
    /// you must be able to hold on to a line while reading the next one.
    /// Hint: `str::find('\n')` gives the byte offset of the next newline.
    pub fn next_line(&mut self) -> Option<&str> {
        todo!()
    }

    /// Everything that hasn't been read yet.
    pub fn remaining(&self) -> &str {
        todo!()
    }
}

/// A place in a scroll where a word of power appears.
#[derive(Debug, PartialEq)]
pub struct Highlight {
    /// The line the word appears on (borrowed from the scroll).
    pub line: &str,
    /// The line number, starting at 1.
    pub line_number: usize,
}

/// Find every line of `scroll` that contains `word`.
/// The highlights borrow from `scroll` — but never from `word`.
pub fn find_highlights(scroll: &str, word: &str) -> Vec<Highlight> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCROLL: &str = "In the beginning there was mana\nAnd the mana was IGNIS\nIGNIS became fire";

    #[test]
    fn reading_line_by_line() {
        let mut reader = ScrollReader::new(SCROLL);
        assert_eq!(reader.next_line(), Some("In the beginning there was mana"));
        assert_eq!(reader.next_line(), Some("And the mana was IGNIS"));
        assert_eq!(reader.next_line(), Some("IGNIS became fire"));
        assert_eq!(reader.next_line(), None);
        assert_eq!(reader.next_line(), None, "an empty scroll stays empty");
    }

    #[test]
    fn lines_outlive_the_next_read() {
        let mut reader = ScrollReader::new(SCROLL);
        let first = reader.next_line();
        let second = reader.next_line(); // `first` must still be usable here!
        assert_eq!(first, Some("In the beginning there was mana"));
        assert_eq!(second, Some("And the mana was IGNIS"));
    }

    #[test]
    fn lines_outlive_the_reader() {
        let line;
        {
            let mut reader = ScrollReader::new(SCROLL);
            line = reader.next_line();
        } // the reader is gone, the scroll is not
        assert_eq!(line, Some("In the beginning there was mana"));
    }

    #[test]
    fn what_remains() {
        let mut reader = ScrollReader::new("one\ntwo\nthree");
        assert_eq!(reader.remaining(), "one\ntwo\nthree");
        reader.next_line();
        assert_eq!(reader.remaining(), "two\nthree");
        reader.next_line();
        reader.next_line();
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn highlighting_words_of_power() {
        let highlights = find_highlights(SCROLL, "IGNIS");
        assert_eq!(
            highlights,
            vec![
                Highlight { line: "And the mana was IGNIS", line_number: 2 },
                Highlight { line: "IGNIS became fire", line_number: 3 },
            ]
        );
        assert!(find_highlights(SCROLL, "GLACIUS").is_empty());
    }

    #[test]
    fn highlights_outlive_the_search_word() {
        let scroll = String::from("frost\nfire\nfrost again");
        let found;
        {
            let word = String::from("frost");
            found = find_highlights(&scroll, &word);
        }
        assert_eq!(found.len(), 2);
        assert_eq!(found[1].line, "frost again");
    }
}
