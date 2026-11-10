// ⚔️ Quest 2.0 — Stack & Heap: The Two Realms of Memory
// 📖 Read: book/src/02_ownership_chronicles/00_stack_heap.md
//
// The Oracle's Quiz. Replace each `todo!()` with a *number literal* —
// your prediction. The tests compare it with what the compiler knows.
// (Assume a 64-bit machine: a pointer is 8 bytes.)
// Wrong guesses are fine: read the failure, learn, fix. That's the point.

/// How many bytes does a `u8` take on the stack?
pub fn bytes_u8() -> usize {
    todo!()
}

/// ...a `u32`?
pub fn bytes_u32() -> usize {
    todo!()
}

/// ...a `char`? (Careful: a Rust char can hold any Unicode rune.)
pub fn bytes_char() -> usize {
    todo!()
}

/// ...a spell circle `[u32; 5]`?
pub fn bytes_circle() -> usize {
    todo!()
}

/// ...a `&str` (a pointer to some text + its length)?
pub fn bytes_str_ref() -> usize {
    todo!()
}

/// ...a `String` (pointer + length + capacity)?
/// Does it matter how long the text is?
pub fn bytes_string() -> usize {
    todo!()
}

/// ...a `Vec<u64>`? (Its elements live on the heap!)
pub fn bytes_vec() -> usize {
    todo!()
}

/// Now some real code. `String::from` copies text onto the heap.
/// Return how many *heap* bytes the incantation uses: that's its length
/// in bytes (`.len()`), not in characters.
pub fn heap_bytes(incantation: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_runes() {
        assert_eq!(bytes_u8(), size_of::<u8>(), "🔮 a u8 is 8 bits");
        assert_eq!(bytes_u32(), size_of::<u32>(), "🔮 a u32 is 32 bits");
        assert_eq!(bytes_char(), size_of::<char>(), "🔮 a char is a Unicode scalar value, not a byte");
    }

    #[test]
    fn arrays_live_entirely_on_the_stack() {
        assert_eq!(bytes_circle(), size_of::<[u32; 5]>(), "🔮 five runes of four bytes each");
    }

    #[test]
    fn a_str_ref_is_a_fat_pointer() {
        assert_eq!(bytes_str_ref(), size_of::<&str>(), "🔮 pointer (8) + length (8)");
    }

    #[test]
    fn string_and_vec_are_three_words() {
        assert_eq!(bytes_string(), size_of::<String>(), "🔮 pointer + length + capacity");
        assert_eq!(bytes_vec(), size_of::<Vec<u64>>(), "🔮 same shape as String");
    }

    #[test]
    fn a_strings_stack_size_never_changes() {
        let short = String::from("Hi");
        let long = String::from("By the seven moons of Arkanor, I summon thee!");
        assert_eq!(size_of_val(&short), size_of_val(&long));
        assert_eq!(bytes_string(), size_of_val(&long));
    }

    #[test]
    fn heap_bytes_count_bytes_not_runes() {
        assert_eq!(heap_bytes(String::from("Fireball")), 8);
        assert_eq!(heap_bytes(String::from("🔥")), 4, "🔥 is one char but four bytes of UTF-8");
    }
}
