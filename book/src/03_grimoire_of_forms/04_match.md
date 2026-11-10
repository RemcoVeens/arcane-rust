# ⚔️ 3.4 — Match: The Art of Divination

> *"The crystal ball reveals many futures. Some lead to treasure, some to peril. A true seer prepares for every one of them — the compiler will not let you skip a single path."*

## ⚡ The Concept

`match` compares a value against a list of **patterns**, top to bottom, and runs the first arm that fits. You've used it on enums; patterns can do much more.

### Literals, ranges and `|`

```rust
fn title(level: u32) -> &'static str {
    match level {
        0 => "Unawakened",
        1..=4 => "Apprentice",        // inclusive range
        5 | 6 | 7 | 8 | 9 => "Adept", // several patterns, one arm
        10..=19 => "Archmage",
        _ => "Legend",                // everything else
    }
}
```

Integers aren't enums, so the compiler can't list "all" values — you need a `_` (or a binding) at the end. It *does* check your ranges: write `5..=9` after `1..=10` and it warns that the arm is unreachable.

### Matching on tuples

Match several values at once by putting them in a tuple:

```rust
match (attack, defender) {
    (Element::Fire, Element::Frost) => 2.0,
    (Element::Frost, Element::Fire) => 0.5,
    (a, d) if a == d => 1.0,          // bind both, then a guard
    _ => 1.0,
}
```

### Destructuring variants and structs

```rust,ignore
enum Creature {
    Slime,
    Goblin { hp: u32 },
    Dragon { hp: u32, element: Element, flying: bool },
}

match creature {
    Creature::Slime => "harmless",
    Creature::Dragon { flying: true, .. } => "run",       // match a field's value
    Creature::Dragon { hp, .. } => "deadly",              // bind a field, ignore the rest
    Creature::Goblin { hp } => "annoying",
}
```

- `field: value` inside a pattern *checks* that field.
- `field` alone *binds* it to a variable of the same name.
- `..` ignores the remaining fields.
- Patterns nest: `Creature::Wraith { element: Element::Arcane, .. }`.

### Match guards

An `if` after a pattern adds a condition the pattern alone can't express:

```rust,ignore
Creature::Goblin { hp } if *hp < 20 => "weak",
Creature::Goblin { .. } => "annoying",
```

If the guard is false, matching just continues with the next arm. (Matching on `&Creature` makes `hp` a `&u32`, hence `*hp`.)

### `@` bindings

Test against a pattern **and** keep the value:

```rust
fn loot(gold: u32) -> String {
    match gold {
        0 => String::from("Nothing."),
        n @ 1..=9 => format!("A few coins ({n})."),
        n => format!("{n} gold!"),
    }
}
```

### `matches!` — a match that answers yes/no

```rust,ignore
let is_boss = matches!(creature, Creature::Dragon { .. } | Creature::Wraith { hp: 101.., .. });
```

`matches!(value, pattern)` is `true` if the pattern fits. Guards work too: `matches!(x, Some(n) if n > 3)`.

<div class="compare">

🐍 **Python 3.10+:** `match`/`case` with class patterns and guards — similar syntax, but *not* exhaustive: a forgotten case silently does nothing.

🐹 **Go:** `switch` on values or types — no destructuring, no exhaustiveness, `default` optional.

🦀 **Rust:** exhaustive, destructuring, nested, with guards and bindings — and it's an expression, so every arm returns a value of the same type.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Order matters.** Arms are tried top to bottom. Put `Goblin { hp } if *hp < 20` *before* the general `Goblin { .. }`, or it never fires.
- **A bare name is a binding, not a comparison.** `match x { max_level => … }` doesn't compare with a variable `max_level` — it binds a *new* variable that matches everything. Use a guard (`n if n == max_level`) or a `const` (consts in patterns *are* compared).
- **Guards don't count for exhaustiveness.** `Some(n) if n > 0` plus `None` is not exhaustive — the compiler can't prove the guard covers everything.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/04_match`

Divine elemental multipliers with tuple patterns, titles with ranges, threat levels with guards and nested patterns, loot messages with `@` bindings, and spot bosses with `matches!`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Literals `0`, ranges `1..=4`, open ranges `100..`, alternatives `A | B`
- Tuples: `match (a, b) { (X, Y) => … }`
- Fields: `V { f: 3, .. }` checks, `V { f, .. }` binds
- Guards: `pattern if condition => …`
- Bindings: `n @ 10..=99 => …`
- `matches!(value, pattern)` → bool
- `_` catches everything; arms are tried top to bottom

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — multiplier & title</summary>

Three strong pairs → `2.0` (join them with `|`), three reversed pairs → `0.5`, `_ => 1.0`. For `title`, one arm per line of the doc comment; the last one is `_`.

</details>
<details class="hint"><summary>Hint 2 — threat</summary>

Seven arms, specific before general: `Creature::Goblin { hp } if *hp < 20 => "weak"`, then `Creature::Goblin { .. } => "annoying"`. For Wraiths match the element in the pattern: `Creature::Wraith { element: Element::Arcane, .. }`.

</details>
<details class="hint"><summary>Hint 3 — loot_message & is_boss</summary>

`n @ 1..=9 => format!("A few coins ({n}).")`, and a plain `n =>` binding for the last arm. For `is_boss`: `matches!(creature, Creature::Dragon { .. } | Creature::Wraith { hp: 101.., .. })`.

</details>
