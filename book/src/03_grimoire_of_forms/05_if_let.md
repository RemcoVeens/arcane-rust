# ⚔️ 3.5 — If Let & Let-Else: The Single-Glance Divination

> *"Sometimes you need not see every future — only whether the chest is open. A seer who stares into the ball for every trivial question never gets anything done."*

## ⚡ The Concept

### `if let` — care about one pattern

A full `match` with a do-nothing `_` arm is noise:

```rust,ignore
match loot {
    Loot::Gold(n) => total += n,
    _ => {}
}
```

`if let` says the same thing in one breath:

```rust,ignore
if let Loot::Gold(n) = loot {
    total += n;
}
```

Read it as "if `loot` matches the pattern `Loot::Gold(n)`, bind `n` and run the block". It can have an `else`, and since it's an expression, it can produce a value:

```rust,ignore
let value = if let Loot::Gold(n) = loot { *n } else { 0 };
```

### `while let` — loop while the pattern matches

```rust
let mut pile = vec![String::from("Fireball"), String::from("Heal")];
while let Some(scroll) = pile.pop() {      // pop() returns Option<String>
    println!("Reading {scroll}");           // Heal, then Fireball
}
```

The loop ends the first time the pattern fails — here, when `pop()` returns `None`. `scroll` is the owned `String`: `pop` moves it out of the Vec.

### `let ... else` — bail out early

The opposite situation: you *need* the value, and if it's missing there's nothing sensible to do except return.

```rust,ignore
fn parse_cast(input: &str) -> String {
    let Some(name) = input.strip_prefix("cast ") else {
        return String::from("That is not a cast command.");
    };
    // `name` is available here, as a plain &str — no extra nesting!
    format!("Casting {name}")
}
```

The `else` block **must diverge**: `return`, `break`, `continue` or panic. The compiler checks that. In exchange, the bound variables are usable for the rest of the function. This keeps the happy path at the left margin — like Go's guard clauses:

<div class="compare">

🐍 **Python:** `if (m := re.match(...)) is None: return` — the walrus gives you something similar.

🐹 **Go:** `spell, ok := book[name]; if !ok { return }` — the same "guard clause" style, by convention.

🦀 **Rust:** `let Some(spell) = find(name) else { return … };` — the same shape, but the compiler guarantees `spell` is only usable when it exists.

</div>

### Which one when?

| Situation | Tool |
|---|---|
| Every case matters | `match` |
| One case matters, the rest is "do nothing / default" | `if let … else` |
| Keep going while something is there | `while let` |
| Need the value; otherwise leave the function | `let … else` |
| Just need a yes/no | `matches!` |

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`=` not `==`.** It's `if let Some(x) = opt`, a pattern binding, not a comparison.
- **`let-else` must diverge.** `let Some(x) = opt else { println!("oops") };` is an error — the else block has to `return`/`break`/`continue`/panic.
- **Shadowed names.** In `if let Loot::Potion { mana } = loot`, the new `mana` shadows any outer `mana` (like your `&mut u32` parameter!). Rename it in the pattern: `Loot::Potion { mana: extra }`.
- **`while let` + `pop` goes from the end.** A Vec pops its *last* element first — that's what makes it a stack.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/05_if_let`

Value some loot, drink potions out of a chest, collect scroll names, read a pile of scrolls from the top with `while let`, and parse cast commands with two `let ... else` guards.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `if let PATTERN = value { … } else { … }`
- `while let PATTERN = expr { … }` — e.g. `while let Some(x) = v.pop()`
- `let PATTERN = value else { return …; };` — else must diverge
- Rename bindings in patterns: `Loot::Potion { mana: extra }`
- `s.strip_prefix("cast ")` → `Option<&str>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — gold_value & drink_if_potion</summary>

`if let Loot::Gold(n) = loot { *n } else { 0 }`. For the potion, rename the field in the pattern so it doesn't shadow the parameter: `if let Loot::Potion { mana: extra } = loot { … }`.

</details>
<details class="hint"><summary>Hint 2 — scroll_names & read_pile</summary>

`for loot in chest { if let Loot::Scroll(name) = loot { names.push(name.as_str()); } }`. For the pile: make it mutable (`let mut pile = pile;`), and inside the `while let` add `", "` before every scroll except the first (`if !read.is_empty()`).

</details>
<details class="hint"><summary>Hint 3 — parse_cast</summary>

```rust,ignore
let Some(name) = input.strip_prefix("cast ") else {
    return String::from("That is not a cast command.");
};
let Some(cost) = spell_cost(name) else { /* … */ };
```

</details>
