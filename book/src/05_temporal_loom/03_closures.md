# ⚔️ 5.3 — Closures: Incantations on the Fly

> *"A one-off spell. A quick calculation. A transformation that only matters once. Closures are incantations you write on the fly, use immediately, and forget."*

## ⚡ The Concept

### Closure syntax

A closure is an anonymous function you can store in a variable or pass around:

```rust
let double = |x: u32| x * 2;          // types can be annotated…
let add = |a, b| a + b;               // …or inferred from use
let shout = || println!("IGNIS!");    // no arguments
let complex = |x: u32| -> u32 {       // block body, explicit return type
    let y = x + 1;
    y * y
};

assert_eq!(double(21), 42);
assert_eq!(add(2, 3), 5);
```

<div class="compare">

🐍 Python: `lambda x: x * 2` (single expression only). · 🐹 Go: `func(x int) int { return x * 2 }`. · 🦀 Rust: `|x| x * 2` — any body, types inferred, and the compiler knows *exactly* what it captures.

</div>

### Capturing the environment

Unlike a `fn`, a closure can use variables from where it was defined:

```rust
let full_moon_bonus = 15;
let empowered = |power: u32| power + full_moon_bonus;   // borrows the bonus
assert_eq!(empowered(45), 60);
```

### Accepting closures: generics + `Fn`

Every closure has its own unique, unnameable type. To accept one you use a generic bounded by a closure trait:

```rust
fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(x)
}

// or with a `where` clause — same thing, easier to read when long
fn cast_with<F>(power: u32, modifier: F) -> u32
where
    F: Fn(u32) -> u32,
{
    modifier(power)
}

assert_eq!(apply(|x| x + 1, 1), 2);
```

`Fn(u32) -> u32` reads "something callable that takes a `u32` and returns a `u32`". Plain functions satisfy it too: `apply(u32::isqrt, 49)`.

### Returning closures: `impl Fn`

```rust
fn make_adder(n: u32) -> impl Fn(u32) -> u32 {
    move |x| x + n
}

let add_ten = make_adder(10);
assert_eq!(add_ten(5), 15);
```

`impl Fn(u32) -> u32` means "some concrete closure type — I won't name it". The `move` keyword makes the closure **own** `n`; without it, the closure would borrow `n`, a local that dies when `make_adder` returns. (More on `move` in the next quest.)

### Closures in the standard library

You'll use closures constantly with the standard library:

```rust
let mut costs = vec![30, 25, 40];
costs.sort_by_key(|c| *c);                 // sort by a key
costs.retain(|c| *c < 35);                 // keep matching elements
let maybe: Option<u32> = Some(45);
let doubled = maybe.map(|p| p * 2).unwrap_or(0);   // transform an Option
let fallback = None.unwrap_or_else(|| 0u32);       // compute default lazily
assert_eq!((costs, doubled, fallback), (vec![25, 30], 90, 0));
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Closure parameter types are fixed on first use.** `let id = |x| x; id(5); id("five");` ❌ — the first call made `x: i32`.
- **Closures with `|s|` over `&Spell` get references.** In `sort_by_key(|s| s.cost)`, `s` is `&Spell`; field access auto-derefs, but comparisons with literals may need `*`.
- **Returning a closure that borrows a local** → "closure may outlive the current function". Add `move`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/03_closures`

Implement the six `todo!()`s: call closures you're given (`apply_twice`, `cast_with`), return one (`make_amplifier`), and hand closures to the standard library (`sort_by_cost`, `forget_uncastable`, `doubled_power`).

</div>

## 📜 The Grimoire

<div class="grimoire">

- `|a, b| a + b` · `|x: u32| -> u32 { ... }` · `|| ...`
- Accept: `fn f<F: Fn(T) -> U>(f: F)` or `where F: Fn(T) -> U`
- Return: `fn f() -> impl Fn(T) -> U { move |x| ... }`
- `v.sort_by_key(|x| key)` · `v.retain(|x| keep?)` · `opt.map(|x| ...).unwrap_or(d)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

A closure parameter is called like any function: `f(power)`. Calling it twice is `f(f(power))`.

</details>
<details class="hint"><summary>Hint 2</summary>

`make_amplifier`: `move |power| power * factor`. `sort_by_cost`: `spells.sort_by_key(|s| s.cost)`.

</details>
<details class="hint"><summary>Hint 3</summary>

`forget_uncastable`: `retain` keeps what the closure approves — so write the condition for *castable*: `s.cost <= mana && s.level <= level`. `doubled_power`: `found.map(|s| s.power * 2).unwrap_or(0)`.

</details>
