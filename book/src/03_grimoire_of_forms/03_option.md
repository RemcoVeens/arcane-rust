# ⚔️ 3.3 — Option: The Absence of Null

> *"A chest may contain treasure — or it may be empty. A spell may be in your grimoire — or it may not. These are not errors. They are possibilities, and Rust writes them into the type."*

## ⚡ The Concept

### There is no null

Tony Hoare called null his "billion-dollar mistake": any reference *might* be nothing, and nothing in the type tells you. Rust has no null. Instead, the standard library has this enum:

```rust,ignore
enum Option<T> {
    Some(T),
    None,
}
```

`Option<u32>` is "maybe a `u32`". `T` is a type parameter (generics are Chapter 4): `Option<String>`, `Option<Spell>`, anything. `Some` and `None` are so common they're in the prelude — no `Option::` prefix needed.

```rust
let staff: Option<u32> = Some(12);
let no_staff: Option<u32> = None;
```

The crucial part: an `Option<u32>` is **not** a `u32`. You can't add it, compare it with a number or print it as one until you've dealt with the `None` case. The compiler won't let you forget.

<div class="compare">

🐍 **Python:** `def find(name) -> Spell | None` — and nothing stops `find("x").cost` from raising `AttributeError` at runtime.

🐹 **Go:** `spell, ok := book[name]` or a `nil` pointer — the check is a convention; skip it and you panic.

🦀 **Rust:** `fn find(name: &str) -> Option<Spell>` — the *type* forces the check before you can touch the spell.

</div>

### Getting the value out

```rust
let staff = Some(12);

match staff {                          // the full treatment
    Some(bonus) => println!("+{bonus}"),
    None => println!("no staff"),
}

let bonus = staff.unwrap_or(0);        // default for None
let bonus = staff.unwrap();            // panics on None — only when you're *sure*
```

### Transforming without unwrapping

```rust
let power: Option<u32> = Some(45);
let doubled = power.map(|p| p * 2);    // Some(90); None stays None
```

`|p| p * 2` is a **closure** — a tiny inline function (`lambda p: p * 2`). They get their own chapter; for `map` this is all you need.

### Producing an Option

```rust
fn share_loot(gold: u32, mages: u32) -> Option<u32> {
    if mages == 0 { None } else { Some(gold / mages) }
}
```

Many standard methods already return `Option`: `text.find(' ')`, `vec.first()`, `vec.pop()`, `a.checked_div(b)`, `a.checked_sub(b)`…

### The `?` operator

Inside a function that returns `Option`, `?` means "if this is `None`, return `None` right now; otherwise give me the value":

```rust,ignore
fn combo_cost(first: &str, second: &str) -> Option<u32> {
    let a = spell_cost(first)?;      // None? → the whole function returns None
    let b = spell_cost(second)?;
    Some(a + b)
}
```

It's like Go's `if err != nil { return nil, err }` — compressed into one character. You'll use it constantly with `Result` in Chapter 4.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`.unwrap()` everywhere.** It turns a compile-time guarantee back into a runtime crash. Fine in tests and prototypes; in real code prefer `match`, `unwrap_or`, `?`, or `if let` (Quest 3.5).
- **`?` only works in functions that return `Option`** (or `Result`). In a function returning `u32` it's a compile error.
- **`Some(x)` vs `x`.** When a function returns `Option<u32>`, the happy path must return `Some(value)`, not `value`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/03_option`

Look up spell costs that might not exist, default a missing staff to zero, share loot without dividing by zero, empower maybe-spells with `map`, combine two lookups with `?`, and call a familiar that may or may not answer.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Option<T>` = `Some(T)` | `None` — no null in Rust
- `match opt { Some(x) => …, None => … }`
- `.unwrap_or(d)` · `.unwrap()` (panics) · `.is_some()` / `.is_none()`
- `.map(|x| …)` transforms the inside, keeps `None`
- `value?` in an `Option`-returning fn: early-return `None`
- `checked_div`, `checked_sub`, `find`, `first`, `pop` return Options

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — spell_cost & staff_bonus</summary>

Loop over `SPELLS` with `for (spell, cost) in SPELLS`, compare `to_lowercase()`s, `return Some(cost)` on a hit, and end the function with `None`. `staff_bonus` is a single `unwrap_or`.

</details>
<details class="hint"><summary>Hint 2 — share_loot & empowered</summary>

An `if mages == 0 { None } else { Some(...) }` works — and `gold.checked_div(mages)` does exactly that in one call. `empowered` is `power.map(|p| p * 2)`.

</details>
<details class="hint"><summary>Hint 3 — combo_cost & call_familiar</summary>

`Some(spell_cost(first)? + spell_cost(second)?)`. For the familiar, `match familiar { Some(name) => format!("{name} hoots!"), None => … }`.

</details>
