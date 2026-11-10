# ⚔️ 1.7 — Constants & Shadowing: The Unchanging Laws

> *"Some laws are written in the stone of the Academy itself: no mage may hold more than one hundred mana. Other truths change form — the word becomes the number, the name becomes its length."*

## ⚡ The Concept

### Constants

```rust
pub const MAX_MANA: u32 = 100;
const SECONDS_PER_TURN: u64 = 6 * 10;   // evaluated at compile time
```

- **Type annotation is required.** No inference for `const`.
- `SCREAMING_SNAKE_CASE`.
- Can live outside functions (module level). Inlined wherever used.
- Must be computable at compile time.

`static` is similar but has one fixed memory address — you'll rarely need it.

<div class="compare">

🐍 `MAX_MANA = 100` is a convention, nothing stops you changing it. · 🐹 Go `const MaxMana = 100` (untyped constant allowed). · 🦀 Rust demands the type.

</div>

### Shadowing

You can declare a **new** variable with the same name. The old one is hidden:

```rust
let input = "  42\n";
let input = input.trim();            // &str, trimmed
let input: u32 = input.parse().unwrap_or(0); // now a u32!
```

This is **not** mutation:

| | `let mut x` | shadowing `let x` again |
|---|---|---|
| Changes the value | ✅ | ✅ (new variable) |
| Can change the type | ❌ | ✅ |
| Old value afterwards | gone | still exists, just hidden |

It's the idiomatic way to "transform" a value step by step without inventing names like `input_str`, `input_trimmed`, `input_num`.

### A peek at `parse`

`"42".parse::<u32>()` returns a `Result` — either `Ok(42)` or an `Err` (Chapter 4). For now: `.unwrap_or(0)` means "the number, or 0 if it failed". The annotation `let x: u32 = …` tells `parse` which type to produce.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`const X = 100;` doesn't compile** — `const` always needs `: Type`.
- **`parse` needs to know the target type.** Annotate the variable or use the turbofish `parse::<u32>()`.
- **Shadowing in an inner block** only lasts until the block ends.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/07_constants_shadowing`

Fix the constant, then use it and shadowing to implement the three functions.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `const NAME: Type = value;` — typed, compile-time, SCREAMING_CASE
- `let x = …; let x = f(x);` — shadowing: new binding, may change type
- `text.trim()`, `text.parse().unwrap_or(default)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

`pub const MAX_MANA: u32 = 100;` — and `fill` is `(mana + amount).min(MAX_MANA)`.

</details>
<details class="hint"><summary>Hint 2</summary>

`parse_mana`: two shadowing lines straight from the concept section, then return `input`.

</details>
