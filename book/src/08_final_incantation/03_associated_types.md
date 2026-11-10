# ⚔️ 8.3 — Associated Types: The Spell's Inner Essence

> *"Every enchantment produces something. Flame yields damage. Frost yields damage and a chill that lingers. Insight yields only words. The Enchanter's Codex does not dictate what — it only demands that each enchantment declare it."*

## ⚡ The Concept

### You've been using one all along

```rust,ignore
pub trait Iterator {
    type Item;                                  // ← associated type
    fn next(&mut self) -> Option<Self::Item>;
    // …70+ provided methods: map, filter, sum, take, …
}
```

`type Item;` is a **placeholder type** that each implementation fills in. `Vec<u32>`'s iterator says `type Item = &u32`; `Chars` says `type Item = char`. Code that uses the trait refers to it as `Self::Item` (inside) or `I::Item` (outside).

### Defining your own

```rust
trait Enchantment {
    type Output;
    fn enchant(&self, power: u32) -> Self::Output;
}

struct Flame;
impl Enchantment for Flame {
    type Output = u32;                           // Flame decides: damage
    fn enchant(&self, power: u32) -> u32 { power * 2 }
}

struct Insight;
impl Enchantment for Insight {
    type Output = String;                        // Insight decides: words
    fn enchant(&self, power: u32) -> String { format!("{power} power") }
}
```

### Associated type vs generic parameter

Why not `trait Enchantment<Output>`? Because a generic trait can be implemented **many times** for one type (`impl Enchantment<u32> for Flame` *and* `impl Enchantment<String> for Flame`), and then every call site must say which one. An associated type means: **exactly one** `Output` per implementor — the implementor chooses, callers just use it.

| | `trait T<X>` (generic) | `trait T { type X; }` (associated) |
|---|---|---|
| impls per type | many (one per `X`) | one |
| who chooses `X` | the caller | the implementor |
| example | `From<T>`, `Add<Rhs>` | `Iterator::Item`, `Deref::Target` |

### Using it in generic code

```rust,ignore
// The return type follows whichever enchantment you pass in.
fn enchant_all<E: Enchantment>(e: &E, powers: &[u32]) -> Vec<E::Output> { … }

// Or pin it down with an equality constraint:
fn total<E: Enchantment<Output = u32>>(e: &E, powers: &[u32]) -> u32 { … }
```

That `Trait<Assoc = Type>` syntax is the same one you write in `impl Iterator<Item = u32>`.

### Implementing `Iterator` — `next` is all you write

```rust
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 { return None; }
        self.0 -= 1;
        Some(self.0 + 1)
    }
}

assert_eq!(Countdown(3).collect::<Vec<_>>(), vec![3, 2, 1]);
assert_eq!(Countdown(4).sum::<u32>(), 10); // all adapters for free
```

<div class="compare">

🐍 Python's iterator protocol: `__next__` raising `StopIteration` ≈ `next` returning `None`. Type hints like `Iterator[int]` are the closest thing to `Item = int` — but unchecked. · 🐹 Go's `iter.Seq[V]` takes the element type as a type parameter — the "generic" column of the table above.

</div>

### A glimpse of GATs

Associated types can themselves be generic — *generic associated types* (GATs), e.g. `type Item<'a> where Self: 'a;` for an iterator that lends out references to its own buffer. Stable since Rust 1.65; you'll recognise them if you meet them, and you won't need them here.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **"not all trait items implemented, missing: `Output`"** — every `impl` must specify every associated type (unless the trait gives a default, which is still unstable for associated types).
- **`Self::Output` vs `E::Output`**: inside the impl use `Self::`; in a generic function over `E: Enchantment`, use `E::Output`.
- **An iterator should keep returning `None`** once it has finished. Make sure your stop condition holds on every later call too.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/08_final_incantation/03_associated_types`

This file **does not compile**: three enchantments and an iterator are missing their associated types. Add them, implement each `enchant`, the generic `enchant_all`, and `ManaRegen::next`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Declare: `trait T { type Out; fn f(&self) -> Self::Out; }`
- Implement: `impl T for X { type Out = u32; … }`
- Use: `fn g<E: T>(e: &E) -> E::Out` · constrain: `E: T<Out = u32>`
- `impl Iterator for X { type Item = …; fn next(&mut self) -> Option<Self::Item> }`
- One impl per type → associated type; many → generic parameter

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the compile errors</summary>

Inside each impl: `type Output = u32;`, `type Output = (u32, u32);`, `type Output = String;`, and for the iterator `type Item = u32;`.

</details>
<details class="hint"><summary>Hint 2 — enchant_all</summary>

`powers.iter().map(|&p| enchantment.enchant(p)).collect()` — the `Vec<E::Output>` return type tells `collect` what to build.

</details>
<details class="hint"><summary>Hint 3 — ManaRegen::next</summary>

If `self.current >= self.max`, return `None`. Otherwise set `self.current` to `(self.current + self.rate).min(self.max)` and return `Some(self.current)`.

</details>
