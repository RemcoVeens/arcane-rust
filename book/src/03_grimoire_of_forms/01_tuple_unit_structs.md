# ⚔️ 3.1 — Tuple & Unit Structs: Minimal Forms

> *"Not every vessel needs labelled compartments. Sometimes a single pouch suffices — as long as everyone knows it holds gold, not gunpowder. And sometimes an empty box is all the meaning you need."*

## ⚡ The Concept

### Tuple structs

A struct with unnamed fields, accessed by position:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub i32, pub i32);

let tower = Position(3, -2);
println!("x = {}, y = {}", tower.0, tower.1);
let Position(x, y) = tower;        // destructuring
```

Useful when the meaning is obvious from the type name and naming fields would just be noise.

### The newtype pattern

A tuple struct with **one** field wraps an existing type to give it a new identity:

```rust,compile_fail
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mana(pub u32);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health(pub u32);

fn heal(health: Health, amount: u32) -> Health { Health(health.0 + amount) }

let mana = Mana(30);
heal(mana, 10);        // ❌ expected `Health`, found `Mana`
```

With plain `u32`s, mixing up mana and health compiles and corrupts your game. With newtypes, it's a type error. And it's free: `size_of::<Mana>() == size_of::<u32>()` — the wrapper disappears at compile time.

<div class="compare">

🐍 **Python:** `Mana = NewType("Mana", int)` — checked by mypy only, ignored at runtime.

🐹 **Go:** `type Mana uint32` — a distinct type, very similar! But Go lets untyped constants slip in (`var m Mana = 5`), and you inherit all arithmetic.

🦀 **Rust:** `struct Mana(u32)` — distinct, zero-cost, and you decide which operations exist. No `+` until you implement it (Chapter 4). For now: unwrap with `.0`, compute, wrap again.

</div>

### Working with newtypes

```rust
fn drain(mana: Mana, cost: u32) -> Mana {
    Mana(mana.0.saturating_sub(cost))     // unwrap → compute → wrap
}
```

Because we derived `Copy`, passing a `Mana` around is as cheap as passing the `u32`.

### Unit structs

A struct with **no fields at all**:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ward;

let w = Ward;                       // the type's only value is its own name
assert_eq!(size_of::<Ward>(), 0);  // takes zero bytes
```

Why would you want that? As a **marker**: a value that *exists* means something happened ("a ward has been raised"), or as a type to hang behaviour on (traits, Chapter 4). You'll meet them again as zero-cost "proof tokens" and type-state markers.

### Overview

| Form | Syntax | Use for |
|---|---|---|
| Named struct | `struct Mage { name: String, … }` | most data |
| Tuple struct | `struct Position(i32, i32);` | small, obvious groupings |
| Newtype | `struct Mana(u32);` | type safety for primitives |
| Unit struct | `struct Ward;` | markers, behaviour without data |

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Field visibility.** In `pub struct Mana(u32)` the *field* is private — outside the module you can't write `Mana(5)` or read `.0`. Write `pub struct Mana(pub u32)` when callers need it.
- **Semicolons.** Tuple and unit structs end with `;` — `struct Ward;`, `struct Position(i32, i32);`. Named structs don't.
- **`abs` and unsigned results.** `(x1 - x2).abs()` is still an `i32`. `x1.abs_diff(x2)` returns a `u32` directly and can't overflow.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/01_tuple_unit_structs`

❌ The file **does not compile** until you define `Gold`, `Position` and `Ward`. Then drain mana, heal with a cap, share gold, walk north, measure Manhattan distances and raise a ward.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Tuple struct: `struct P(pub i32, pub i32);` · access `p.0`, `p.1` · `let P(x, y) = p;`
- Newtype: `struct Mana(pub u32);` — distinct type, zero cost
- Unit struct: `struct Ward;` — value is `Ward`, size 0
- Unwrap → compute → wrap: `Mana(m.0.saturating_sub(c))`
- `a.abs_diff(b)` — distance between two integers, as unsigned

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the definitions</summary>

Put `#[derive(Debug, Clone, Copy, PartialEq)]` above each: `pub struct Gold(pub u32);`, `pub struct Position(pub i32, pub i32);`, `pub struct Ward;`.

</details>
<details class="hint"><summary>Hint 2 — drain, heal, share, step_north</summary>

All follow the same shape: `Health((health.0 + amount).min(MAX_HEALTH))`. For `step_north`: `Position(pos.0, pos.1 + 1)`.

</details>
<details class="hint"><summary>Hint 3 — distance & raise_ward</summary>

`let Position(x1, y1) = a; let Position(x2, y2) = b;` then `x1.abs_diff(x2) + y1.abs_diff(y2)`. `raise_ward` returns just `Ward`.

</details>
