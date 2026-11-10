# ⚔️ 1.6 — Loops: The Cyclical Incantations

> *"The goblin's hide is thick. One Fireball will not do. Two will not do. The apprentice asks: how many? The master answers: cast until it falls — and count."*

## ⚡ The Concept

Rust has three loops.

### `loop` — forever, until `break`

```rust
let mut count = 0;
let turns = loop {
    count += 1;
    if count == 3 {
        break count * 10;     // `loop` can return a value through `break`
    }
};
// turns == 30
```

### `while` — while a condition holds

```rust
let mut hp = 100;
while hp > 0 {
    hp = hp.saturating_sub(30);
}
```

### `for` — over anything iterable

```rust
let costs = [30, 25, 40];
for cost in costs {
    println!("{cost}");
}
for i in 0..3 { }          // 0, 1, 2     (exclusive range)
for i in 1..=3 { }         // 1, 2, 3     (inclusive range)
for i in (1..=3).rev() { } // 3, 2, 1
for (i, c) in costs.iter().enumerate() { } // with index
```

<div class="compare">

🐍 `for i in range(3)` ≈ `for i in 0..3`. · 🐹 Go has only `for`; Rust splits it into `loop`/`while`/`for`, and `for` never uses index arithmetic by default.

</div>

### Labels: breaking out of nested loops

```rust
let grid = [[1, 2], [3, 4]];
'search: for row in grid {
    for cell in row {
        if cell == 3 {
            break 'search;      // leaves BOTH loops
        }
    }
}
```

`continue` skips to the next iteration (also works with labels).

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Underflow in loops.** `hp -= 30` with `hp = 10u32` panics. Use `saturating_sub`.
- **`0..n` excludes `n`.** Use `0..=n` when you need it included.
- **Off-by-one in counting** — write the test cases out on paper: 100 HP, 30 damage → 70, 40, 10, 0 → 4 casts.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/06_loops`

Use each kind of loop at least once: `while` for combat, `for` for sums and countdowns, `loop` with `break value` for meditation, and a labeled loop to search the rune grid.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `loop { … break v; }` — infinite loop, can return a value
- `while cond { … }`
- `for x in iterable { … }`; ranges `a..b`, `a..=b`, `.rev()`, `.enumerate()`
- `'label: for …` + `break 'label;` / `continue 'label;`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — countdown</summary>

Start with `let mut s = String::new();`, add parts with `s.push_str(&format!("{i}... "));`, finally `format!("{s}{spell}!")`.

</details>
<details class="hint"><summary>Hint 2 — turns_to_full</summary>

Shadow the parameter: `let mut mana = mana;`. In the `loop`, first check `if mana >= max { break count; }`, then regenerate with `.min(max)` and `count += 1`.

</details>
<details class="hint"><summary>Hint 3 — find_rune</summary>

`let mut found = (99, 99);` then two nested `for r in 0..3` / `for c in 0..3` loops; on a match set `found = (r, c)` and `break 'search;`.

</details>
