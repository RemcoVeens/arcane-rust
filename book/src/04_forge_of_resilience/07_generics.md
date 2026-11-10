# ⚔️ 4.7 — Generics: The Universal Incantations

> *"A healing spell works on humans, elves and beasts alike. A pouch holds gems as well as scrolls. The formula stays the same — only the subject changes. That is a generic."*

## ⚡ The Concept

### The problem

```rust,ignore
fn strongest_u32(items: &[u32]) -> Option<u32> { /* ... */ }
fn strongest_f64(items: &[f64]) -> Option<f64> { /* same code */ }
fn strongest_char(items: &[char]) -> Option<char> { /* same code again */ }
```

### The solution: a type parameter

```rust,compile_fail
fn strongest<T>(items: &[T]) -> Option<T> {
    let mut best = items[0];
    for &item in items {
        if item > best { best = item; }   // ❌ binary operation `>` cannot be applied to type `T`
    }
    Some(best)
}
```

`T` can be *any* type — and that's the problem: not every type can be compared with `>`, or copied out of a slice. You must **promise** what `T` can do, with **trait bounds**:

```rust
fn strongest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut best = *items.first()?;
    for &item in items {
        if item > best { best = item; }
    }
    Some(best)
}

strongest(&[45, 150, 35]);     // T = i32
strongest(&[1.5, 2.0]);        // T = f64
strongest(&['ᚠ', 'ᚱ']);        // T = char
```

A few standard traits make useful bounds (Quest 4.8 dives into traits):

| Bound | Means `T` can… |
|---|---|
| `PartialOrd` | be compared with `<`, `>` |
| `PartialEq` | be compared with `==` |
| `Copy` | be copied implicitly (numbers, `char`, `bool`, `&T`) |
| `Clone` | be duplicated with `.clone()` (`String`, `Vec`, …) |
| `std::fmt::Display` | be printed with `{}` |

<div class="compare">

🐹 Go 1.18+: `func Strongest[T cmp.Ordered](items []T) T` — the same idea, with *constraints*. · 🐍 Python: `def strongest(items: list[T]) -> T` — type hints only, nothing checked at runtime. · 🦀 Bounds are checked at compile time, and each use is compiled to specialised code ("monomorphization"): zero runtime cost.

</div>

### Generic structs

```rust
struct Pouch<T> {
    items: Vec<T>,
}

impl<T> Pouch<T> {                         // "for every T, Pouch<T> has these methods"
    fn new() -> Self { Pouch { items: Vec::new() } }
    fn stash(&mut self, item: T) { self.items.push(item); }
}

let mut gems = Pouch::new();
gems.stash("ruby");                        // now it's a Pouch<&str>
let mut scrolls: Pouch<String> = Pouch::new();
```

You've been using generic types all along: `Vec<T>`, `Option<T>`, `Result<T, E>`, `HashMap<K, V>`.

### Methods that only exist for some `T`

An `impl` block can carry its own bounds. The methods inside only exist when `T` meets them:

```rust,ignore
struct Duel<T> { left: T, right: T }

impl<T> Duel<T> {                 // every Duel
    fn swap(self) -> Duel<T> { Duel { left: self.right, right: self.left } }
}

impl<T: PartialOrd> Duel<T> {     // only Duels of comparable things
    fn winner(&self) -> &T { /* ... */ }
}
```

### `where` clauses

When bounds get long, move them after the signature:

```rust,ignore
fn report<T, U>(a: T, b: U) -> String
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug,
{ /* ... */ }
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **No bound, no abilities.** With a bare `T` you can only move, store and return it. Read the compiler's suggestion: it usually names the exact bound to add (`consider restricting type parameter T: help: T: PartialOrd`).
- **`impl<T>` needs the `<T>` twice**: `impl<T> Pouch<T>` — the first declares `T`, the second uses it.
- **Moving out of a slice:** `let best = items[0];` only works for `Copy` types. For others, return a reference (`Option<&T>`) or `.clone()`.
- **`Copy` vs `Clone`:** `Copy` is implicit and cheap; `Clone` is explicit. `String` is `Clone` but not `Copy`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/07_generics`

Write a generic `strongest` and `echo`, a generic `Pouch<T>`, and a `Duel<T>` whose `winner` method only exists for comparable `T`. Some signatures are missing bounds on purpose.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `fn f<T: Bound1 + Bound2>(x: T)` · or `where T: Bound`
- `struct S<T> { x: T }` · `impl<T> S<T> { ... }` · `impl<T: Bound> S<T> { ... }`
- Common bounds: `PartialOrd`, `PartialEq`, `Copy`, `Clone`, `Display`, `Debug`
- Turbofish when inference fails: `strongest::<u32>(&[])`
- Generics are monomorphized: no runtime cost

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — strongest and echo</summary>

Write the body first, then add the bounds the compiler asks for. `strongest` needs `>` and copying values out of the slice: `T: PartialOrd + Copy`. `echo` needs `.clone()`: `T: Clone`. In `echo`, loop `for _ in 0..n` and push `item.clone()`.

</details>
<details class="hint"><summary>Hint 2 — Pouch</summary>

Every method is one line on `self.items`: `Vec::new()`, `push`, `pop`, `last`, `len`, `is_empty`. (`last()` returns exactly the `Option<&T>` that `peek` needs.)

</details>
<details class="hint"><summary>Hint 3 — Duel</summary>

`swap` builds a new `Duel { left: self.right, right: self.left }` (it takes `self` by value, so moving the fields out is fine). For `winner`, add `impl<T: PartialOrd> Duel<T> { pub fn winner(&self) -> &T { ... } }` returning `&self.right` if it is strictly greater, otherwise `&self.left`.

</details>
