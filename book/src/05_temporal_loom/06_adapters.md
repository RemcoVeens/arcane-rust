# ⚔️ 5.6 — Iterator Adapters: Spell Analytics

> *"Take fire. Add ice. Transform to steam. Channel the steam into a turbine. An adapter chain does the same to a river of values — each stage reshapes the flow."*

## ⚡ The Concept

**Adapters** take an iterator and return a new, lazier iterator. **Consumers** finally pull the values and produce a result. A chain is `source → adapters… → consumer`:

```rust
let costs = vec![30, 25, 40, 20, 90];
let cheap_total: u32 = costs
    .iter()                 // source: &u32
    .filter(|&&c| c < 35)   // adapter: keep cheap ones
    .map(|&c| c * 2)        // adapter: transform
    .sum();                 // consumer
assert_eq!(cheap_total, 150);
```

### The adapters you'll use daily

| Adapter | Does | Example |
|---|---|---|
| `map(f)` | transform each item | `.map(\|s\| s.power)` |
| `filter(p)` | keep items where `p` is true | `.filter(\|s\| s.cost < 30)` |
| `enumerate()` | pair with index `(i, x)` | `.enumerate()` |
| `zip(other)` | pair with another iterator | `a.iter().zip(b.iter())` |
| `take(n)` / `skip(n)` | first n / all but first n | `.take(3)` |
| `rev()` | reverse (when possible) | `(1..=3).rev()` |
| `chain(other)` | one after the other | `a.iter().chain(b.iter())` |
| `filter_map(f)` | map to `Option`, keep `Some`s | `.filter_map(\|s\| s.parse().ok())` |

### The consumers

| Consumer | Returns | Example |
|---|---|---|
| `collect()` | `Vec`, `HashMap`, `String`, … | `.collect::<Vec<_>>()` |
| `sum()` / `count()` | number | `.map(\|c\| c.damage).sum()` |
| `max_by_key(f)` / `min_by_key(f)` | `Option<Item>` | `.max_by_key(\|c\| c.damage)` |
| `find(p)` | first match, `Option<Item>` | `.find(\|c\| c.critical)` |
| `position(p)` | index of first match | `.position(\|c\| c.turn == 3)` |
| `any(p)` / `all(p)` | `bool` | `.any(\|c\| c.critical)` |
| `fold(init, f)` | anything | `.fold(0, \|acc, c\| acc + c.damage)` |

### `fold` — the universal consumer

`fold` threads an accumulator through every item. Anything you'd write as "make a variable, loop, update it" can be a fold:

```rust
use std::collections::HashMap;
let casts = [("Fireball", 45), ("Heal", 0), ("Fireball", 90)];
let by_spell: HashMap<&str, u32> = casts.iter().fold(HashMap::new(), |mut acc, &(name, dmg)| {
    *acc.entry(name).or_insert(0) += dmg;
    acc
});
assert_eq!(by_spell["Fireball"], 135);
```

### Order matters

```rust
let log = ["a", "", "b"];
let numbered: Vec<String> = log
    .iter()
    .filter(|s| !s.is_empty())      // filter FIRST…
    .enumerate()                    // …then number: 0, 1
    .map(|(i, s)| format!("{}. {s}", i + 1))
    .collect();
assert_eq!(numbered, ["1. a", "2. b"]);
```

Swap `filter` and `enumerate` and you'd get `1. a`, `3. b`.

### `collect` needs to know what to build

`collect` can produce many types, so tell it — with a variable type or the turbofish:

```rust
let names: Vec<&str> = ["Fireball", "Heal"].into_iter().collect();
let joined = ["Ignis", "Maxima"].iter().map(|s| s.to_uppercase()).collect::<Vec<_>>().join(" ");
assert_eq!(joined, "IGNIS MAXIMA");
```

<div class="compare">

🐍 Python: list comprehensions and generator expressions — `sum(c.damage for c in log if c.critical)`. · 🐹 Go: mostly hand-written `for` loops. · 🦀 Rust: `log.iter().filter(|c| c.critical).map(|c| c.damage).sum()` — lazy like a generator, and compiled to a loop as fast as hand-written code ("zero-cost abstraction").

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Adapters are lazy.** `v.iter().map(|x| println!("{x}"));` does *nothing* (and the compiler warns: "unused `Map` that must be used").
- **Reference layers.** `iter()` yields `&T`; `filter` gives its closure `&&T`. Patterns like `|&&c|` or `|c| **c` peel them off — or just use field access, which auto-derefs.
- **`max_by_key` returns the *last* maximum on ties, `min_by_key` the *first*.**
- **Averages:** check for an empty slice before dividing — `0 / 0` as `f64` is `NaN`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/06_adapters`

Answer the Archmage's nine questions about the battle log. Each function should be **one iterator chain** — no `for`, no `let mut` accumulators.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Source: `.iter()` · Adapters: `map filter enumerate zip take skip rev chain filter_map`
- Consumers: `collect sum count max_by_key min_by_key find position any all fold last`
- `fold(HashMap::new(), |mut acc, x| { ...; acc })`
- `opt.map(|c| c.turn)` turns `Option<&Cast>` into `Option<u32>`
- `.collect::<Vec<_>>()` — turbofish when the type isn't otherwise known

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

`total_damage`: `log.iter().map(|c| c.damage).sum()`. `strongest_cast`: `log.iter().max_by_key(|c| c.damage)`. `first_turn_with`: `find` then `map(|c| c.turn)`.

</details>
<details class="hint"><summary>Hint 2</summary>

`average_damage`: return `None` early if `log.is_empty()`; otherwise `Some(total as f64 / log.len() as f64)`. `damage_by_spell`: fold into a `HashMap`, using `entry(c.spell.clone()).or_insert(0)`.

</details>
<details class="hint"><summary>Hint 3</summary>

`chronicle`: `.filter(|c| c.damage > 0).enumerate().map(|(i, c)| format!("{}. {} ({}){}", i + 1, c.spell, c.damage, if c.critical { "!" } else { "" })).collect()`. `pair_with_targets`: `log.iter().zip(targets).map(|(c, &t)| (c.spell.as_str(), t)).collect()`.

</details>
