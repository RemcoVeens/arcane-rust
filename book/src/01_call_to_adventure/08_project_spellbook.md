# 🏗️ Project 1 — The Spellbook CLI

> *"Knowledge is power. A grimoire is proof of it."*

This is the first stone of your tower: a **Spellbook CLI** that knows your spells, tracks your mana and obeys your commands. You've already written something like it by hand — this time the Gatekeepers will tell you if it's *right*.

```text
╔══════════════════════════╗
║   ✦ ARCANE GRIMOIRE ✦    ║
╚══════════════════════════╝
Welcome, Tiberius. Type 'help' for commands.

[Tiberius | Lv 7 | ✦ 100/100] > cast fireball
You cast Fireball! (45 damage)

[Tiberius | Lv 7 | ✦ 70/100] > cast meteor
Requires level 10
```

## 🧭 The layout

```text
08_project_spellbook/
├── src/lib.rs        ← the game logic: YOU implement this
├── src/main.rs       ← the game loop (stdin/stdout) — already written
└── tests/spellbook.rs← the Gatekeepers (integration tests)
```

The game loop in `main.rs` is deliberately dumb: it reads a line, hands it to `handle`, prints the message. All the logic is in **pure functions** in `lib.rs`, which makes it testable. This split — *logic in a library, IO in a thin binary* — is how real Rust CLIs are built.

Integration tests in `tests/` use the crate like any outside user would: `use ch01_08_project_spellbook::*;`. Only `pub` items are visible.

## ✨ One new tool: `Option`

`find_spell` may not find anything. Rust has no `null`; instead it returns an `Option`:

```rust,ignore
fn find_spell(name: &str) -> Option<Spell> {
    for spell in SPELLS {
        if /* names match */ {
            return Some(spell);
        }
    }
    None
}
```

Consuming it — for now, this pattern is all you need (`let … else` is explained in Chapter 3):

```rust,ignore
let Some((name, cost, power, level)) = find_spell(input) else {
    return (mana, "Unknown spell".to_string());
};
// here the tuple fields are available
```

Also useful: `text.to_lowercase()`, `text.trim()`, `text.starts_with("cast ")`, `text.strip_prefix("cast ")`.

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/08_project_spellbook`

Implement every `todo!()` in `src/lib.rs`. The tests are grouped in three phases — make them pass in order.

</div>

### Phase 1 — The grimoire
`banner`, `find_spell` (case-insensitive!) and `spell_list` (`"1. Fireball — mana 30, power 45, level 1"`, lines joined with `\n`, no trailing newline).

### Phase 2 — Mana
`drink_potion` (capped at `MAX_MANA`) and `cast`, with all four outcomes: unknown spell, level too low, not enough mana, success.

### Phase 3 — Commands
`handle` dispatches the commands. Trim and lowercase the input first.

Then **play it**:

```bash
cargo run -p ch01_08_project_spellbook
```

### 🌟 Side quests (no tests — just for glory)
- Add a `meditate` command that restores 10 mana.
- Add critical hits: `std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() % 5 == 0`.
- Track an enemy's HP in `main.rs` and announce its defeat.

## 📜 Save Point

<div class="grimoire">

- Logic in `lib.rs` (pure functions), IO in `main.rs`
- Integration tests in `tests/` see only `pub` items
- `Option<T>` = `Some(value)` or `None` — Rust's replacement for null
- You built a real, interactive, *tested* CLI. The tower rises.

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — spell_list</summary>

`for (i, (name, cost, power, lvl)) in SPELLS.iter().enumerate()` — and push a `'\n'` *before* every line except the first.

</details>
<details class="hint"><summary>Hint 2 — cast</summary>

Order of checks: unknown → level → mana → success. Each failure returns `(mana, message)` with the mana unchanged.

</details>
<details class="hint"><summary>Hint 3 — handle</summary>

`let input = input.trim().to_lowercase();` then an `if / else if` chain. For casting: `else if let Some(spell) = input.strip_prefix("cast ") { let (m, msg) = cast(spell, mana, level); (m, msg, true) }`.

</details>
