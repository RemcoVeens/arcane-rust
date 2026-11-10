# ⚔️ 1.2 — Scalar Types: The Elemental Runes

> *"Fire, Water, Earth, Air — the four elemental planes. In Rust we call them integers, floats, booleans and characters. Master these, and you can build anything."*

## ⚡ The Concept

A **scalar** is a single value. Rust has four kinds.

### 1. Integers

| Bits | Signed | Unsigned |
|---|---|---|
| 8 | `i8` (−128..=127) | `u8` (0..=255) |
| 16 | `i16` | `u16` |
| 32 | `i32` *(default)* | `u32` |
| 64 | `i64` | `u64` |
| CPU word | `isize` | `usize` *(indices, lengths)* |

```rust
let mana: u32 = 1_000;          // underscores are just for readability
let population: u64 = 8_000_000_000;
```

### 2. Floats

```rust
let gravity = 9.81;            // f64 (default)
let ratio: f32 = 0.5;
```

### 3. Booleans and 4. Characters

```rust
let is_mage = true;
let rune = 'ᚱ';                // char: single quotes, any Unicode scalar, 4 bytes
let word = "rune";             // &str: double quotes
```

### No implicit conversions

Rust never converts numbers for you. You convert explicitly with `as`:

```rust
let level: u8 = 7;
let damage: u32 = 45 + level as u32;   // u8 → u32
let pct = 25 as f64 / 200 as f64;      // 0.125
let code = 'A' as u32;                 // 65
```

<div class="compare">

🐍 Python: `5 / 3 == 1.666…`, ints never overflow. · 🐹 Go: `5 / 3 == 1`, ints silently wrap. · 🦀 Rust: `5 / 3 == 1`, and overflow **panics** in debug builds.

</div>

### Overflow: choose your behavior

```rust
let mana: u8 = 10;
// mana - 30          → panic in debug: "attempt to subtract with overflow"
mana.saturating_sub(30);   // 0   — stop at the edge
mana.wrapping_sub(30);     // 236 — wrap around (Go behavior)
mana.checked_sub(30);      // None — "it didn't work" (Option, Chapter 3)
300u32.min(255);           // 255 — clamp, then `as u8` is safe
```

And `as` itself **truncates**: `300u32 as u8 == 44`. Surprise!

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`as` never fails — it silently truncates.** Clamp first if the value might not fit.
- **Integer division truncates.** `100 / 3 == 33`; the remainder is `100 % 3 == 1`.
- **`'a'` is a `char`, `"a"` is a `&str`.** Different types, not interchangeable.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/02_scalar_types`

Replace every `todo!()` with a one-liner. Watch out for the two overflow traps.

</div>

`todo!()` compiles fine (it has every type) but panics when called — so the tests fail until you replace it.

## 📜 The Grimoire

<div class="grimoire">

- Integers: `i8..i128`, `u8..u128`, `isize/usize`; default `i32`. Floats: `f64` default.
- `char` = 4-byte Unicode scalar in `'…'`.
- Convert with `as` (truncates!). Use `saturating_*`, `wrapping_*`, `checked_*` to control overflow.
- `/` on integers truncates, `%` is the remainder.

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — drain & engrave</summary>

Look at the overflow table above. One function is `saturating_sub`; the other clamps with `.min(255)` before the `as u8`.

</details>
<details class="hint"><summary>Hint 2 — percent</summary>

Convert *both* numbers to `f64` before dividing, then multiply by `100.0`.

</details>
<details class="hint"><summary>Hint 3 — runes & gold</summary>

`rune as u32`, `gold / mages`, `gold % mages`.

</details>
