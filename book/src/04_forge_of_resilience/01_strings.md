# ⚔️ 4.1 — Strings: The Weave of Incantations

> *"An incantation is a string of syllables; a true name is a string of runes. The Academy's oldest scrolls are written in runes no Latin alphabet ever knew — and Rust reads them all."*

## ⚡ The Concept

### `String` vs `&str` — the owner and the view

You met both in Chapter 2. A quick recap:

| | `String` | `&str` |
|---|---|---|
| Owns its bytes? | yes (heap) | no — borrows from somewhere |
| Can grow? | yes (`push_str`) | no |
| Literals | — | `"Fireball"` is a `&'static str` |
| Like | Go `strings.Builder` / Python `str` | Go `string` / a Python slice view |

```rust
let literal: &str = "ignis";              // baked into the binary
let owned: String = String::from("ignis"); // or "ignis".to_string() / .to_owned()
let view: &str = &owned;                    // &String → &str for free
```

**Rule of thumb:** parameters take `&str` (they accept both), return values and struct fields use `String`.

### Building strings

```rust
let mut scroll = String::new();
scroll.push_str("ignis");     // append a &str
scroll.push(' ');             // append one char
scroll += "magna";            // also push_str

let mage = "Tiberius";
let line = format!("Hail, {mage}! You know {} spells.", 5); // like Python f-strings / Go Sprintf
```

### UTF-8: bytes ≠ characters

A Rust string is **always valid UTF-8**. ASCII letters take 1 byte, `é` takes 2, runes like `ᚠ` take 3, emoji take 4.

```rust
let rune = "ᚠᚢᚦ";
rune.len();             // 9  — BYTES
rune.chars().count();   // 3  — characters (Unicode scalar values)
```

<div class="compare">

🐍 Python's `len("ᚠᚢᚦ")` is 3 (it counts code points). · 🐹 Go's `len("ᚠᚢᚦ")` is 9 (bytes) and you range over **runes** to get characters. · 🦀 Rust is like Go: `.len()` is bytes, `.chars()` gives you `char`s — Rust's word for a Go rune.

</div>

### No indexing — iterate instead

```rust,compile_fail
let spell = "ᚠᚢᚦ";
let c = spell[0];   // ❌ `str` cannot be indexed by an integer
```

What would `spell[0]` even be — the first byte (half a rune)? Rust refuses to guess. Instead:

```rust
let spell = "ignis";
let first: Option<char> = spell.chars().next();   // Some('i'), None for ""
for c in spell.chars() { /* each char */ }
for c in spell.chars().rev() { /* backwards */ }
for c in spell.chars().take(3) { /* just the first 3 chars */ }
for b in spell.bytes() { /* each u8 */ }
```

Byte-range slicing `&s[0..3]` exists, but it **panics** if the range cuts a character in half. Safe for ASCII you control; dangerous for user input.

### Everyday methods

```rust
"  ignis \n".trim()           // "ignis"
"ignis".to_uppercase()        // "IGNIS" (a new String)
"ignis magna".split_whitespace() // iterator over "ignis", "magna"
"cast fireball".starts_with("cast") // true
"Fireball".contains("ball")   // true
"a,b".replace(",", " ")       // "a b"
s.is_empty()
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`.len()` counts bytes.** `"café".len()` is 5. Use `.chars().count()` for "how many letters".
- **`&s[..n]` can panic.** `&"Ætherblast"[..1]` panics: `Æ` is 2 bytes. Take `n` chars instead.
- **`+` takes ownership of the left side:** `let c = a + &b;` moves `a`. Prefer `format!` or `push_str`.
- **Comparisons just work:** `String == &str` compares contents — no `.as_str()` needed in `assert_eq!`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/01_strings`

Greet mages, inscribe scrolls, count runes vs bytes, reverse incantations and truncate names — all Unicode-safe.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `String::new()`, `String::from(s)`, `s.to_string()`, `format!(...)`
- `push_str(&str)`, `push(char)`, `+=`
- `.len()` bytes · `.chars().count()` chars · `.chars().next()` first char
- `.chars().rev()`, `.chars().take(n)` · build with `let mut out = String::new(); out.push(c);`
- `trim`, `to_uppercase`, `to_lowercase`, `split_whitespace`, `starts_with`, `contains`
- Parameters `&str`, owned results `String`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — inscribe</summary>

Check `scroll.is_empty()` first; only push a `' '` if there is already something on it. Then `scroll.push_str(word)`.

</details>
<details class="hint"><summary>Hint 2 — reverse and truncate</summary>

Both are the same pattern: start with `let mut out = String::new();`, loop over an iterator of chars, `out.push(c)`, return `out`. The iterators are `spell.chars().rev()` and `spell.chars().take(n)` — `take` stops early on its own if the string is shorter.

</details>
<details class="hint"><summary>Hint 3 — the rest</summary>

`greet` is one `format!`. `rune_count` is `word.chars().count()`, `byte_weight` is `word.len()`, `first_rune` is `word.chars().next()`, and `battle_cry` chains `words.trim().to_uppercase()`.

</details>
