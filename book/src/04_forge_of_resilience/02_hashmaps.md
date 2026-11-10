# ⚔️ 4.2 — HashMaps: The Lexicon of Magic

> *"A spellbook indexed by name. A bestiary keyed by creature. A bag where every item knows how many of it you carry. The lexicons of the magical world are maps."*

## ⚡ The Concept

`HashMap<K, V>` maps keys to values. It is not in the prelude, so import it:

```rust
use std::collections::HashMap;

let mut bag: HashMap<String, u32> = HashMap::new();
bag.insert("mana potion".to_string(), 2);
bag.insert("scroll".to_string(), 5);
bag.insert("scroll".to_string(), 1);   // overwrites: scroll is now 1
```

<div class="compare">

🐍 `dict` · 🐹 `map[string]int` · 🦀 `HashMap<String, u32>`. Like Go (and unlike Python 3.7+), **iteration order is arbitrary** — and Rust even randomises it per run.

</div>

### Reading

```rust
let n: Option<&u32> = bag.get("scroll");        // a &str works as lookup key
let n: u32 = bag.get("scroll").copied().unwrap_or(0); // "0 if missing"
bag.contains_key("scroll");                     // true
bag.len();
```

`get` never panics; `bag["scroll"]` does if the key is missing. Missing keys are *normal* in a map, so prefer `get`.

<div class="compare">

🐹 Go's `n := bag["x"]` silently returns the zero value. 🐍 Python's `bag["x"]` raises `KeyError`, `bag.get("x", 0)` gives a default. Rust's `get(..)` gives an `Option` and you choose: `.unwrap_or(0)`, `match`, `if let`.

</div>

### The `entry` API — insert-or-update in one step

The classic "count things" pattern:

```rust
let mut freq: HashMap<String, u32> = HashMap::new();
for word in "ignis magna ignis".split_whitespace() {
    *freq.entry(word.to_string()).or_insert(0) += 1;
}
// {"ignis": 2, "magna": 1}
```

`entry(key)` looks the key up once; `or_insert(0)` inserts `0` if it was absent and returns a `&mut u32` to the value either way; `*` dereferences it so you can add. It's Python's `defaultdict(int)` in one line.

### Changing or removing values

```rust
if let Some(qty) = bag.get_mut("scroll") {
    *qty -= 1;              // modify in place through &mut
}
bag.remove("scroll");       // → Option<u32> (the old value)
```

### Iterating

```rust
for (item, qty) in &bag {        // (&String, &u32), arbitrary order!
    println!("{item}: {qty}");
}
let total: u32 = bag.values().sum();
```

Need a stable order (for tests, for display)? Collect into a `Vec` and `sort()` it — tuples sort by their first element, then the second.

### Ownership

`insert` **moves** owned keys and values into the map. Inserting a `String` key means the map owns it now; you look it up later with a `&str`.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Never rely on iteration order.** A test that works today will fail tomorrow. Sort first, or use `BTreeMap` (sorted keys).
- **`get_mut` + `remove` in the same scope:** you can't call `bag.remove(..)` while you still hold the `&mut` from `get_mut`. Finish using the reference first (the borrow ends at its last use), then remove.
- **`entry` needs an owned key** (`String`), even if the key already exists — so `entry(item.to_string())`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/02_hashmaps`

Build the mage's inventory: add (stacking piles), count, use items (removing empty piles), total and list them alphabetically, and count words in an incantation.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `use std::collections::HashMap;` · `HashMap::new()`
- `insert(k, v)` (overwrites) · `get(&k)` → `Option<&V>` · `get_mut` → `Option<&mut V>` · `remove` · `contains_key`
- `*map.entry(k).or_insert(default) += 1;`
- `for (k, v) in &map` · `map.values().sum()` · `map.len()`, `is_empty()`
- `.copied().unwrap_or(0)` turns `Option<&u32>` into a `u32` with a default

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — add and count</summary>

`add_item` is the entry pattern with `+= qty`. `count` is `inv.get(item)` → `Option<&u32>`; `.copied()` → `Option<u32>`; `.unwrap_or(0)`.

</details>
<details class="hint"><summary>Hint 2 — use_item</summary>

`let Some(qty) = inv.get_mut(item) else { return false; };` (let-else from Chapter 3). Then decrement through the reference. If it hit zero, `inv.remove(item)` — the `&mut` is no longer used at that point, so the borrow checker is happy.

</details>
<details class="hint"><summary>Hint 3 — sorting and words</summary>

`sorted_items`: create an empty `Vec`, loop `for (name, qty) in inv`, push `(name.clone(), *qty)`, then `items.sort()`. `word_frequency`: loop over `text.split_whitespace()` and use the entry pattern with `word.to_lowercase()` as the key.

</details>
