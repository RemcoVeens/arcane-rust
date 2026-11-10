# ⚔️ 1.4 — Functions: The Incantation Forms

> *"An incantation has a name, it takes ingredients, and it produces an effect. Speak it wrongly — add one syllable too many — and nothing happens at all."*

## ⚡ The Concept

```rust
fn spell_damage(base: u32, level: u32) -> u32 {
    base + level * 5
}
```

- Parameters **always** have types. No inference across function boundaries.
- `-> Type` is the return type. No arrow means it returns `()`.
- Naming is `snake_case`.

### Statements vs expressions — the one syllable too many

Almost everything in Rust is an **expression** that produces a value. A `;` turns an expression into a **statement**, which throws the value away.

```rust,compile_fail
fn mana_after_cast(mana: u32, cost: u32) -> u32 {
    mana - cost;   // ← statement: value discarded, function returns ()
}                  // ❌ expected `u32`, found `()`
```

Remove the semicolon and the last expression becomes the return value. `return x;` exists for **early** returns:

```rust
fn safe_divide(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 0;
    }
    a / b
}
```

### Blocks are expressions too

```rust
let empowered = {
    let doubled = 21 * 2;
    doubled + 1           // value of the block
};
```

### Building strings: `format!`

```rust
let name = "Fireball";
let cost = 30;
let text: String = format!("{name} costs {cost} mana");   // like println!, but returns a String
```

<div class="compare">

🐍 `f"{name} costs {cost} mana"` · 🐹 `fmt.Sprintf("%s costs %d mana", name, cost)` · 🦀 `format!("{name} costs {cost} mana")`

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **The stray semicolon** is the #1 beginner error. `expected X, found ()` ⇒ look for a `;` at the end of your function.
- **`&str` vs `String`:** take text as `&str` (borrowed), return `String` (owned) when you build new text. Chapter 2 explains why.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/04_functions`

This file does not compile: fix `mana_after_cast`, **write** `describe_spell` from scratch, and finish the block in `empowered`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `fn name(p: Type) -> Ret { … }`
- The last expression (no `;`) is the return value; `return` for early exits
- Blocks `{ … }` evaluate to their last expression
- `format!` builds a `String` with the same syntax as `println!`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

The test calls `describe_spell("Fireball", 30)` and compares with a string. So the signature is `pub fn describe_spell(name: &str, cost: u32) -> String`. Don't forget `pub`.

</details>
<details class="hint"><summary>Hint 2</summary>

`empowered`: the block's last line should be `doubled + 1` — no semicolon.

</details>
