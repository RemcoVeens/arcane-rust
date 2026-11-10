# ⚔️ 2.6 — Slices: A Window Into the Grimoire

> *"A crystal ball does not show the whole future — only a window into it. A slice is a window into a sequence: you see part of it, you own none of it."*

## ⚡ The Concept

### String slices: `&str`

A **slice** is a borrowed view of a *contiguous part* of some data:

```rust
let grimoire = String::from("Fireball Frostbolt");
let first: &str = &grimoire[0..8];   // "Fireball"
let second: &str = &grimoire[9..];   // "Frostbolt"
let all: &str = &grimoire[..];       // the whole thing
```

Ranges work like Python's `s[start:end]` — start inclusive, end exclusive — but the indices are **byte offsets**.

```text
grimoire: String ──▶ F i r e b a l l ␣ F r o s t b o l t
                     ▲─────────────▲   ▲─────────────────▲
               &grimoire[0..8]       &grimoire[9..]
               (ptr, len 8)          (ptr+9, len 9)
```

A `&str` is a **fat pointer**: start address + length. No copying, no allocation. And a string literal `"Fireball"` is *also* a `&str` — a view into text stored in your binary.

### Why `&str` beats `&String` in parameters

```rust
fn first_word(incantation: &str) -> &str {
    let end = incantation.find(' ').unwrap_or(incantation.len());
    &incantation[..end]
}

let owned = String::from("Heal thyself");
first_word(&owned);          // &String → &str automatically (deref coercion)
first_word("Fireball Ice");  // a literal is already a &str
first_word(&owned[5..]);     // a slice of a slice — fine too
```

`&String` would only accept the first call. `&str` accepts all three. **Rule of thumb: take `&str`, return `String` when you build something new, return `&str` when you return a part of the input.**

Notice the return type: the result points *into the argument*. With exactly one reference parameter, Rust infers "the output borrows from the input" for you.

`.find(pattern)` returns `Some(byte_index)` or `None`; `.unwrap_or(default)` picks the default for `None` (you saw it with `parse` in Chapter 1). `.split_once(' ')` gives `Some((before, after))`.

### Array and Vec slices: `&[T]`

The same idea works for any sequence:

```rust
let costs = [30, 25, 40, 20, 90];
let middle: &[u32] = &costs[1..4];        // [25, 40, 20]

fn strongest(powers: &[u32]) -> u32 { /* ... */ 0 }
strongest(&costs);                        // &[u32; 5] → &[u32]
strongest(&vec![1, 2, 3]);                // &Vec<u32> → &[u32]
strongest(&costs[..2]);                   // part of an array
```

Like `&str` vs `&String`: **take `&[T]`, not `&Vec<T>` or `&[T; N]`.**

### Mutable slices: `&mut [T]`

```rust
let mut damage = [10, 20, 30, 40, 50];
let buffed = &mut damage[1..4];
for d in buffed.iter_mut() {   // yields &mut u32
    *d *= 2;
}
assert_eq!(damage, [10, 40, 60, 80, 50]);
```

<div class="compare">

🐍 **Python:** `s[1:4]` *copies* into a new list/string.

🐹 **Go:** `s[1:4]` shares the backing array — and `append` on it can overwrite the original's elements. Surprise!

🦀 **Rust:** `&s[1..4]` shares, like Go, but the borrow rules guarantee nobody can modify the original while you look, and a `&mut [T]` slice can't grow.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Byte indices, not characters.** `&"🔥fire"[0..1]` **panics**: byte 1 is in the middle of the 4-byte 🔥. Use indices from `.find()` (always on a char boundary), or count with `.chars()`.
- **Out of range panics.** `&costs[..99]` on a 5-element array crashes at runtime. Clamp first: `n.min(costs.len())`.
- **The slice keeps the owner borrowed.** While `first` (a slice of `grimoire`) is alive, `grimoire.push_str(...)` is a borrow conflict — same rule as the previous quest.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/06_slices`

Find first words and schools of magic, count runes vs bytes, find the strongest power, take the first `n` spells of a book (as a slice!), total the cost of a slice of spells, and empower part of an array in place.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `&s[a..b]`, `&s[a..]`, `&s[..b]`, `&s[..]` — byte ranges, end exclusive
- `&str` = string slice · `&[T]` = slice of any sequence · `&mut [T]` = writable view
- Parameters: `&str` over `&String`, `&[T]` over `&Vec<T>`
- `.find(p)` → `Option<usize>` · `.unwrap_or(d)` · `.split_once(c)`
- `.chars().count()` runes · `.len()` bytes / elements · `.is_empty()`
- `for x in slice` (`&T`) · `for x in slice.iter_mut()` (`&mut T`) · `for &x in slice` (copies)

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — first_word & school</summary>

Find the end index, then slice from the start: `let end = text.find(' ').unwrap_or(text.len());`. For `school`, what default index gives you an empty slice `&full_name[..?]`?

</details>
<details class="hint"><summary>Hint 2 — strongest & total_cost</summary>

Loop with a running variable. `for &p in powers` copies each `u32` out of the slice. For spells, destructure in the loop header: `for (_, cost, _) in spells { total += cost; }`.

</details>
<details class="hint"><summary>Hint 3 — first_spells & empower</summary>

`&spellbook[..n.min(spellbook.len())]` — one line, no allocation. For `empower`: `for d in damage.iter_mut() { *d *= 2; }`.

</details>
