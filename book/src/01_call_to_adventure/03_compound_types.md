# ⚔️ 1.3 — Compound Types: Tuples & Arrays

> *"A mage's pouch holds a feather, a stone and a vial — different things, together. A spell circle holds five runes — always five, always runes."*

## ⚡ The Concept

### Tuples — the mixed pouch

A fixed number of values of **different** types:

```rust
let spell: (&str, u32, u32) = ("Fireball", 30, 45);

let cost = spell.1;                 // access by position: .0, .1, .2
let (name, cost, power) = spell;    // or destructure
let (name, _, _) = spell;           // `_` ignores a field
```

The empty tuple `()` is called **unit** — it's what functions without a return value return.

<div class="compare">

🐍 Python tuples, but with a fixed type per position. · 🐹 Go has no tuples; you'd return multiple values or use a struct.

</div>

### Arrays — the elemental row

A fixed number of values of the **same** type. The length is part of the type:

```rust
let costs: [u32; 5] = [30, 25, 40, 20, 15];
let first = costs[0];
let zeros = [0; 5];                 // [0, 0, 0, 0, 0]
let grid = [['.'; 3]; 3];           // 3x3 array of arrays
let center = grid[1][1];
```

`[u32; 5]` and `[u32; 6]` are *different types*. Need a growable list? That's `Vec<T>` — coming in Chapter 4.

Out-of-bounds access is checked: `costs[10]` **panics** instead of reading random memory.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Tuple access is `.0`, not `[0]`.** Arrays use `[i]`.
- **Arrays don't grow.** No `push` on `[T; N]`.
- **Array indices are `usize`.** `costs[i]` with `i: u32` doesn't compile — use `costs[i as usize]`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/03_compound_types`

Build, read and destructure tuples and arrays. No loops yet — indexing only.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Tuple: `(a, b, c)`, access `.0`, destructure `let (x, y, z) = t;`
- Array: `[T; N]`, literal `[1, 2, 3]` or `[v; N]`, access `a[i]`
- Nested: `[[T; N]; M]`, `grid[row][col]`
- `()` = unit = "nothing"

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

Return a tuple literal from `fireball()` exactly like the one in the test. `spell_power` is a single `.2`.

</details>
<details class="hint"><summary>Hint 2</summary>

`swap`: `let (n, s) = pair;` then return `(s, n)`. `uniform_circle`: the `[v; N]` syntax.

</details>
