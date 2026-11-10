# ⚔️ 4.0 — Vectors: The Dynamic Grimoire

> *"A mage's power grows. New spells are learned, old ones are forgotten. A fixed-length array cannot hold a growing grimoire. You need a `Vec`."*

## ⚡ The Concept

A `Vec<T>` is a **growable, heap-allocated list** of values of one type `T`.

```rust
let mut spells: Vec<String> = Vec::new();   // empty, type annotated
let costs = vec![30, 25, 40];               // the vec! macro, type inferred: Vec<i32>

spells.push(String::from("Fireball"));      // add to the end
spells.push("Frostbolt".to_string());
let last = spells.pop();                    // Option<String>: Some("Frostbolt")
spells.insert(0, "Heal".to_string());       // insert at index (shifts the rest)
let removed = spells.remove(0);             // remove at index → String (shifts back)
```

<div class="compare">

🐍 Python `list` · 🐹 Go slice `[]T` · 🦀 `Vec<T>` — same idea: pointer + length + capacity. Unlike Python, every element has the **same type**. Unlike Go, there is no `append` that may or may not alias: a `Vec` owns its elements, full stop.

</div>

### Reading: indexing vs `.get`

```rust
let book = vec!["Fireball", "Frostbolt"];
let first = book[0];          // "Fireball" — panics if out of range!
let maybe = book.get(7);      // Option<&&str>: None, never panics
```

Use `[i]` when an out-of-range index would be a **bug**. Use `.get(i)` when it is a normal possibility (user input, unknown length).

### Iterating

```rust
let mut costs = vec![30, 25, 40];

for c in &costs { println!("{c}"); }       // borrow each element (&i32)
for c in &mut costs { *c += 5; }            // mutate in place (*c dereferences)
for c in costs { println!("{c}"); }         // consumes the vector — costs is gone afterwards
```

### Handy methods

```rust
let powers = vec![45, 35, 60];
powers.len();                       // 3
powers.is_empty();                  // false
powers.contains(&60);               // true
powers.first();                     // Some(&45)
powers.last();                      // Some(&60)
powers.iter().sum::<i32>();         // 140
powers.iter().max();                // Some(&60) — an Option<&i32>
powers.iter().max().copied();       // Some(60)  — copy the value out of the reference
```

### `&Vec<T>` or `&[T]`?

Remember slices from Chapter 2? A `&Vec<T>` turns into a `&[T]` automatically, so functions that only **read** should take a slice — they then accept vectors, arrays and sub-slices alike:

```rust
fn total(costs: &[u32]) -> u32 { costs.iter().sum() }

total(&vec![30, 25]);  // a Vec
total(&[30, 25, 40]);  // an array
```

Only take `&mut Vec<T>` when you need to **grow or shrink** it (`push`, `pop`, `remove`).

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`for x in vec` moves the vector.** Write `for x in &vec` unless you really are done with it.
- **No pushing while you hold a reference into it.** A `push` may reallocate and move every element; the borrow checker refuses:

  ```rust,compile_fail
  let mut v = vec![1, 2, 3];
  let first = &v[0];
  v.push(4);            // ❌ cannot borrow `v` as mutable
  println!("{first}");
  ```
- **`remove(i)` is O(n)** — it shifts everything after `i`. Fine for a spellbook, not for a million particles.
- **Comparing a `Vec<String>` with `&str`s works**: `assert_eq!(book, vec!["Fireball"])` compiles, because `String == &str` is defined.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/00_vectors`

Implement the spellbook helpers: create a book, learn spells (no duplicates!), forget spells, look up pages safely, tear out the last page, and compute totals and maxima.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Create: `Vec::new()`, `vec![a, b, c]`
- Grow/shrink: `push`, `pop` (→ `Option<T>`), `insert(i, x)`, `remove(i)` (→ `T`)
- Read: `v[i]` (panics) · `v.get(i)` (→ `Option<&T>`) · `first`, `last`, `len`, `is_empty`, `contains(&x)`
- Loop: `for x in &v` · `for x in &mut v` · `for x in v` (consumes)
- `v.iter().sum()`, `v.iter().max()`, `.copied()` turns `Option<&T>` into `Option<T>`
- Read-only parameters: `&[T]`; growing parameters: `&mut Vec<T>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — creating and learning</summary>

`vec!["Fireball", "Frostbolt"]` would be a `Vec<&str>`. You need owned `String`s: call `.to_string()` on each. For `learn`, loop over the book with `for known in book.iter()` and `return` early if `known == spell`; otherwise `push`.

</details>
<details class="hint"><summary>Hint 2 — forgetting</summary>

You need the *index* to call `remove`. Loop with `for i in 0..book.len()`, compare `book[i] == spell`, and `remove(i)` + `return true` as soon as you find it. Returning right away also avoids the borrow problem of mutating while iterating.

</details>
<details class="hint"><summary>Hint 3 — the one-liners</summary>

`page` is `book.get(page)`, `tear_last_page` is `book.pop()`, `total_cost` is `costs.iter().sum()`, and `strongest` is `powers.iter().max()` followed by `.copied()`.

</details>
