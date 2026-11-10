# ⚔️ 5.5 — Iterators: The River of Values

> *"A river flows. Each drop follows the next, and the river never hands you the whole ocean at once. An iterator is that river: one value, then the next, until it runs dry."*

## ⚡ The Concept

### The `Iterator` trait

The whole iterator machinery rests on one small trait:

```rust,ignore
pub trait Iterator {
    type Item;                                  // what it yields
    fn next(&mut self) -> Option<Self::Item>;   // Some(value) … then None
    // + ~75 provided methods: map, filter, sum, collect, take, …
}
```

Implement `next`, and you get every adapter in the standard library for free — the same "required + provided methods" pattern you saw with traits in Chapter 4.

`type Item` is an **associated type**: each iterator has exactly one item type, so you write `Iterator<Item = u32>` instead of a generic parameter.

### Your own iterator

```rust
struct Countdown {
    remaining: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.remaining == 0 {
            return None;
        }
        let current = self.remaining;
        self.remaining -= 1;
        Some(current)
    }
}

let mut c = Countdown { remaining: 2 };
assert_eq!(c.next(), Some(2));
assert_eq!(c.next(), Some(1));
assert_eq!(c.next(), None);

for n in (Countdown { remaining: 3 }) {   // `for` calls next() until None
    print!("{n}… ");
}
```

The iterator **is** its state. Each `next` advances it; once it returns `None`, keep returning `None`.

### Iterators that borrow

An iterator can yield references into data it borrows — lifetimes from 5.2, back again:

```rust
struct Each<'a> {
    items: &'a [String],
    index: usize,
}

impl<'a> Iterator for Each<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<&'a String> {
        let item = self.items.get(self.index)?;  // `?` works on Option too
        self.index += 1;
        Some(item)
    }
}
```

### Three ways to iterate a collection

| Call | `for` sugar | Yields | Collection afterwards |
|---|---|---|---|
| `v.iter()` | `for x in &v` | `&T` | untouched |
| `v.iter_mut()` | `for x in &mut v` | `&mut T` | modified in place |
| `v.into_iter()` | `for x in v` | `T` | **consumed** (moved) |

### Lazy by nature

Iterators do nothing until something pulls values out of them. `(1..).map(|x| x * 2)` is an *infinite* sequence that costs nothing — until you `take(3)` and `collect()`:

```rust
let firsts: Vec<u32> = (1..).map(|x| x * 2).take(3).collect();
assert_eq!(firsts, vec![2, 4, 6]);
```

### Returning iterators: `impl Iterator`

```rust
fn odd_turns(last: u32) -> impl Iterator<Item = u32> {
    (1..=last).step_by(2)
}
```

The caller decides whether to collect, sum, or loop — no intermediate `Vec` is built.

<div class="compare">

🐍 Python: `__next__` + `StopIteration`; generators with `yield`. Rust has no `yield` (on stable) — you store the state in a struct instead. · 🐹 Go: `for range` over slices/maps/channels, and since 1.23 range-over-func iterators. · 🦀 Rust: `Option` replaces `StopIteration`, and adapters compile down to the same machine code as a hand-written loop.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`next` needs `&mut self`** — so the iterator variable must be `let mut it = ...`.
- **Struct literal in a `for` head** needs parentheses: `for n in (Countdown { remaining: 3 })`.
- **Infinite iterators + `collect()`** = hang forever. Always `take(n)` first.
- **`for x in v` moves `v`.** Use `&v` if you still need it after the loop.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/05_iterators`

1. `impl Iterator` for `Countdown`, `ManaRegen` and the borrowing `Rotation<'a>` (which cycles forever).
2. Return a lazy iterator from `even_turns`.
3. Loop with `iter_mut` in `empower_all` and with ownership in `into_names`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `impl Iterator for T { type Item = X; fn next(&mut self) -> Option<X> { ... } }`
- Borrowing: `impl<'a> Iterator for T<'a> { type Item = &'a X; ... }`
- `iter()` → `&T` · `iter_mut()` → `&mut T` · `into_iter()` → `T`
- `fn f() -> impl Iterator<Item = u32>` · `(a..=b).step_by(n)` · `.take(n)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

`ManaRegen::next`: if `mana >= max` → `None`. Otherwise add `regen`, cap at `max` with `.min(self.max)`, store it back, and return `Some(self.mana)`.

</details>
<details class="hint"><summary>Hint 2</summary>

`Rotation`: `type Item = &'a Spell;`. If `spells` is empty → `None`. Otherwise take `&self.spells[self.next_index]` and advance with `(self.next_index + 1) % self.spells.len()`.

</details>
<details class="hint"><summary>Hint 3</summary>

`even_turns`: `(2..=last_turn).step_by(2)`. `empower_all`: `for spell in spells.iter_mut() { spell.power += 10; }`. `into_names`: `for spell in spells { names.push(spell.name); }` — moving the `String` out is fine because you own each spell.

</details>
