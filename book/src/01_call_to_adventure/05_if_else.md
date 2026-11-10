# ⚔️ 1.5 — If/Else: The Forking Paths of Fate

> *"At the crossroads stands a signpost with three arms. The wise mage does not wander — she reads the runes and chooses."*

## ⚡ The Concept

```rust
let power = 45;
if power == 0 {
    println!("utility");
} else if power < 30 {
    println!("minor");
} else {
    println!("mighty");
}
```

- **No parentheses** around the condition (the compiler warns if you add them).
- **Braces are mandatory**, even for one line.
- The condition **must be a `bool`**. There is no truthiness:

```rust,compile_fail
let mana = 10;
if mana { }          // ❌ expected `bool`, found integer
if mana > 0 { }      // ✅
```

<div class="compare">

🐍 `if mana:` works on any value. · 🐹 Go also requires `bool`. · 🦀 Rust requires `bool` — and adds expressions.

</div>

### `if` is an expression

No ternary operator needed — `if` itself returns a value:

```rust
let aura = if mana > 50 { "blue" } else { "grey" };
```

Both branches must have **the same type**. `if x { 5 } else { "five" }` does not compile.

### Logic

`&&` (and), `||` (or), `!` (not) — short-circuiting, like Go.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Order of `else if` matters.** Check the most specific case first (`n % 15` before `n % 3`).
- **Branches of an `if` expression must match types**, and a value-producing `if` needs an `else`.
- **`&str` vs `String` in branches:** if a function returns `String`, convert literals with `"Fizz".to_string()`, numbers with `n.to_string()`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/05_if_else`

Classify spells, check casting conditions, read auras and open the Rune Gate (a magical FizzBuzz).

</div>

## 📜 The Grimoire

<div class="grimoire">

- `if cond { } else if cond { } else { }` — no parens, always braces, `bool` only
- `let x = if c { a } else { b };` — branches must have the same type
- `&&`, `||`, `!`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — tiers</summary>

Go from the bottom up: `if power == 0 { … } else if power < 30 { … } else if power < 60 { … } else { … }`.

</details>
<details class="hint"><summary>Hint 2 — aura</summary>

`let color = if mana > 50 { "blue" } else { "grey" };` then `format!` the sentence.

</details>
<details class="hint"><summary>Hint 3 — rune gate</summary>

Check `n % 15 == 0` first. Return `"Fizz".to_string()` etc., and `n.to_string()` for the fallback.

</details>
