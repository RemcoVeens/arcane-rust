# ⚔️ 4.5 — The `?` Operator: The Path of Least Resistance

> *"A mage's spell chain should flow like water. If one link fails, the chain breaks cleanly and the failure flows back to whoever started it. One rune does this: `?`."*

## ⚡ The Concept

### The problem

Last quest you wrote this by hand:

```rust,ignore
let a = match parse_cost(a) {
    Ok(v) => v,
    Err(e) => return Err(e),
};
```

### The solution

```rust,ignore
let a = parse_cost(a)?;
```

`expr?` means: *if this is `Ok(v)`, give me `v`; if it is `Err(e)`, return `Err(e)` from the current function right now.*

```rust
use std::num::ParseIntError;

fn parse_cost(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()          // already the right Result type — just return it
}

fn combined_cost(a: &str, b: &str) -> Result<u32, ParseIntError> {
    Ok(parse_cost(a)? + parse_cost(b)?)
}
```

<div class="compare">

🐹 Go: `v, err := f(); if err != nil { return 0, err }` — three lines, every time. 🦀 `let v = f()?;` — same control flow, one character. 🐍 Python: like an exception propagating up, except it is **visible** at every call site and in every signature.

</div>

### `?` works on `Option` too

In a function returning `Option`, `?` on a `None` returns `None` early:

```rust
fn first_word_len(text: &str) -> Option<usize> {
    let word = text.split_whitespace().next()?;   // None if there are no words
    Some(word.len())
}
```

### Mixing: Result inside an Option function

`?` won't silently turn an `Err` into a `None` — you convert explicitly with `.ok()`:

```rust
fn cost_of(line: &str) -> Option<u32> {
    let mut parts = line.split_whitespace();
    let _name = parts.next()?;                   // Option → ?
    let cost: u32 = parts.next()?.parse().ok()?; // Result → .ok() → Option → ?
    Some(cost)
}
```

And the other way round: `.ok_or(err)` turns an `Option` into a `Result`, so you can `?` it in a `Result` function.

Some standard methods return an `Option` exactly where things can go wrong: `a.checked_div(b)` is `None` when `b == 0`.

### `Box<dyn Error>`: one error type to hold them all

What if a function can fail with a `ParseIntError` *and* with your own "missing part" message? Use `Box<dyn std::error::Error>` — a box that can hold **any** error type. (`dyn` is explained properly in Quest 4.9; for now, read it as "any error".)

```rust
use std::error::Error;

fn parse_level(line: &str) -> Result<u32, Box<dyn Error>> {
    let word = line.split_whitespace().nth(1).ok_or("missing level")?; // &str → Box<dyn Error>
    let level = word.parse::<u32>()?;                                   // ParseIntError → Box<dyn Error>
    Ok(level)
}
```

`?` doesn't just return the error, it **converts** it into the function's error type (using the `From` trait — Quest 4.6). Both `&str` and every standard error convert into `Box<dyn Error>`.

### `?` in `main`

`main` can return a `Result` too, so quick programs can use `?` everywhere:

```rust,ignore
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let level: u32 = "7".parse()?;
    println!("Level {level}");
    Ok(())
}
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`?` only works in functions returning `Result` / `Option`.** Using it in a function returning `u32` or `()` is a compile error ("the `?` operator can only be used in a function that returns `Result` or `Option`").
- **`?` on a `Result` in an `Option` function** (or vice versa) doesn't compile — convert with `.ok()` / `.ok_or(...)`.
- **The last expression still needs wrapping:** `Ok(a? + b?)`, not `a? + b?`.
- **`.ok()` discards the error.** Fine when you return `Option` anyway; a loss of information otherwise.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/05_question_mark`

Parse spell costs and whole spell lines like `"Fireball 30 45"`. Every function can be written without a single `match` — let `?` carry the failures.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `x?` on `Result`: `Ok(v)` → `v`, `Err(e)` → `return Err(e.into())`
- `x?` on `Option`: `Some(v)` → `v`, `None` → `return None`
- `result.ok()` → `Option` · `option.ok_or(err)` → `Result`
- `a.checked_div(b)` → `None` on division by zero
- `Result<T, Box<dyn std::error::Error>>`: accepts any error and `&str` messages via `?`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the small ones</summary>

`parse_cost` is `input.trim().parse()` — the return type tells `parse` what to produce. `combined_cost` is `Ok(parse_cost(a)? + parse_cost(b)?)`. `first_word_shouted`: grab the word with `split_whitespace().next()?`, return `Some(word.to_uppercase())`.

</details>
<details class="hint"><summary>Hint 2 — power_per_mana</summary>

`let mut parts = line.split_whitespace();` then three `parts.next()?` calls (name, cost, power). For the numbers: `parts.next()?.parse().ok()?` with a type annotation `let cost: u32 = ...`. Finish with `power.checked_div(cost)` — it already returns an `Option<u32>`.

</details>
<details class="hint"><summary>Hint 3 — parse_spell</summary>

Same skeleton, but with `parts.next().ok_or("missing cost")?` for each part, and `.parse()?` for the numbers. End with `Ok(Spell { name: name.to_string(), cost, power })`.

</details>
