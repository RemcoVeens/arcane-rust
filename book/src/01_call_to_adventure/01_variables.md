# ⚔️ 1.1 — Variables: The Immutable Truth

> *"In the Arcane Academy, the first lesson is this: a variable holds a truth. If you need a different truth, you must say so out loud."*

## ⚡ The Concept

### `let` and `mut`

```rust
let mana = 100;          // immutable — cannot change
let mut mana = 100;      // mutable — can change
mana = 75;               // OK, because of `mut`
```

<div class="compare">

🐍 Python: everything is mutable. · 🐹 Go: everything is mutable, `const` opts out. · 🦀 Rust: everything is **immutable**, `mut` opts in.

</div>

This is not an annoyance. It is a **shield**: when you read `let x = ...`, you *know* `x` never changes, no matter how long the function is.

### Type inference

```rust
let spell_power = 42;    // i32 (default integer)
let mana = 100.5;        // f64 (default float)
let name = "Merlin";     // &str (string slice)
let level: u32 = 7;      // or say it explicitly
```

Like Go's `:=`, but the compiler can also infer from *later* use — e.g. returning `mana` from a function declared `-> u32` makes `mana` a `u32`.

### Scope

Variables live until the end of the `{ }` block they were declared in:

```rust,compile_fail
fn main() {
    let x = 5;
    {
        let y = 10;
        println!("{x} {y}"); // OK
    }
    println!("{y}");         // ❌ y is gone
}
```

### Silencing "unused" warnings

```rust
let _not_yet = "a rune for later"; // a leading `_` means "I know"
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`mut` is not the default.** Python fingers will write `let x = 5; x = 6;`. The compiler error literally tells you to add `mut` — read it!
- **`+=` is also a mutation.** It needs `mut` too.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/01_variables`

This file **does not compile**. Fix the three functions so it builds and the tests pass.

</div>

This is the first time `wr` greets you with a *compile error* instead of a failed test. Same deal: read the error, fix, repeat.

## 📜 The Grimoire

<div class="grimoire">

- `let x = v;` immutable · `let mut x = v;` mutable
- Types are inferred at compile time; annotate with `let x: u32 = 7;`
- Variables are dropped at the end of their `{ }` block
- Prefix unused variables with `_`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — rest_and_meditate</summary>

`error[E0384]: cannot assign twice to immutable variable`. The `help:` line underneath tells you exactly what to add.

</details>
<details class="hint"><summary>Hint 2 — mage_stats</summary>

`staff_power` is declared inside an inner `{ }` block, so it is dropped before the last line. Remove the extra block (or move the `let` outside it).

</details>
<details class="hint"><summary>Hint 3 — count_scrolls</summary>

This one compiles, but with a warning. Rename `unused_rune` to `_unused_rune`.

</details>
