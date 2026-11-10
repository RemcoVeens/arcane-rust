# 🏗️ Project 2 — The Mana System

> *"Mana is the currency of magic. To own it is to control it. To lend it is to trust. To lose track of it is to invite chaos."*

Your Chapter 1 spellbook works — but look at how it moves data around. Every function gets mana *by value* and returns the new mana in a tuple. The spell list is a global constant, so you can never learn anything new. Time to rebuild it the Rust way.

## 🎯 The Goal

```text
╔══════════════════════════════════╗
║   ARCANE GRIMOIRE v0.2           ║
║   ✦ The Ownership Edition ✦      ║
╚══════════════════════════════════╝
Welcome, Tiberius. Type 'help' for commands.

[Tiberius | Lv 7 | ✦ 100/100 | 📜 2] > cast arcane blast
Unknown spell: 'arcane blast'
[Tiberius | Lv 7 | ✦ 100/100 | 📜 2] > read
You read the scroll and learn Arcane Blast!
[Tiberius | Lv 7 | ✦ 100/100 | 📜 1] > cast arcane blast
You cast Arcane Blast! (60 damage)
[Tiberius | Lv 7 | ✦ 60/100 | 📜 1] > potion
Glug glug. Mana is now 100.
```

The project lives in `exercises/02_ownership_chronicles/07_project_mana_system`:

- `src/main.rs` — the game loop, **already written**. It owns the state: `mana: u32`, `spellbook: Vec<Spell>`, `scrolls: Vec<Spell>`, and lends them to `handle` on every turn.
- `src/lib.rs` — the functions you implement, plus module `chapter1` with the old by-value spellbook for reference.
- `tests/mana_system.rs` — the spec, in three phases.

Play at any time with `cargo run -p ch02_07_project_mana_system`.

## 📋 Before → After

| Chapter 1 | Chapter 2 | Why |
|---|---|---|
| `find_spell(name)` searches a global | `find_spell(spellbook: &[Spell], name: &str)` | borrow *any* list of spells |
| `drink_potion(mana) -> u32` | `drink_potion(mana: &mut u32)` | change mana in place |
| `cast(..) -> (u32, String)` | `cast(.., mana: &mut u32, ..) -> String` | no more threading mana through tuples |
| fixed `SPELLS` array | `Vec<Spell>` that grows | scrolls *move* spells into it |
| `input.starts_with("cast ")` | `split_command(input) -> (&str, &str)` | slices of the input, zero copies |

## 🧱 Phase 1 — Reading through borrows

Implement `split_command`, `spell_list` and `find_spell`.

- `split_command` trims the input and splits it at the first space into `(command, argument)` — two `&str` **slices of the input**. The argument is trimmed too. Both elements borrow from `input`, which is why the signature needs no extra annotations: one reference goes in, references into it come out.
- `spell_list` and `find_spell` take `&[Spell]`, so they work on the starting book, on `&TOWER_SCROLLS[1..]`, on an empty slice — anything.

<div class="pitfall">

`Spell` is a tuple of `&'static str` and `u32`s — every part is `Copy`, so the whole tuple is `Copy`. That's why `find_spell` can return `Option<Spell>` (a copy) instead of a reference. With `String` names you'd have to return a reference or clone.

</div>

## 🧱 Phase 2 — Mana through `&mut`

Implement `rest`, `drink_potion`, `spend_mana` and `cast`.

`cast` now borrows the spellbook (`&[Spell]`) *and* mutably borrows the mana (`&mut u32`). Two different values, two different borrows — no conflict. Build `cast` on top of `find_spell` and `spend_mana`: mana must only change when the spell actually fires.

Inside `cast`, `mana` is already a `&mut u32`. To pass it on to `spend_mana(mana, cost)` you don't write `&mut mana` — that would be a `&mut &mut u32`. Just pass `mana` (Rust *reborrows* it for the call, so you can still use it afterwards).

## 🧱 Phase 3 — Scrolls & commands

Implement `read_scroll` and `handle`.

`read_scroll` takes the **first** scroll out of `scrolls` and puts it at the end of `spellbook`. `Vec::remove(0)` hands you the element *by value* — it's no longer in the vector — and `push` moves it into the other one. One value, one owner, always.

`handle` ties it all together. Use `split_command`, lowercase the command (`.to_lowercase()` gives a new `String`; compare with `command == "list"`), and dispatch with an `if / else if` chain, as in Chapter 1. It lends `mana`, `spellbook` and `scrolls` onward to the functions you just wrote.

<div class="pitfall">

`handle` receives `spellbook: &mut Vec<Spell>`, but `spell_list` wants `&[Spell]`. Just pass `spellbook` — a `&mut Vec<T>` coerces to `&[T]` automatically. And `read_scroll(scrolls, spellbook)` works with two `&mut` at once because they point at *different* vectors.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/07_project_mana_system`

Implement every `todo!()` in `src/lib.rs`, phase by phase, until all tests in `tests/mana_system.rs` pass. Then play a round, read both scrolls, and try to cast Meteor at level 7.

**Bonus:** once everything is green, delete the `chapter1` module. Then try the experiments below and read the errors.

</div>

### 🧪 Ownership experiments (optional)

1. In `main.rs`, remove the `&mut` before `mana` in the `handle` call. What does the compiler suggest?
2. In `cast`, change `spellbook: &[Spell]` to `spellbook: &mut [Spell]`. Which call sites break, and why is `&` the right choice?
3. In `read_scroll`, keep a reference `let first = &scrolls[0];` *before* `scrolls.remove(0)` and use it after. Which rule of the Compact did you break?

## 📜 The Grimoire

<div class="grimoire">

- Parameters: `&str`, `&[T]` for reading · `&mut T` / `&mut Vec<T>` for changing
- `&mut Vec<T>` and `&Vec<T>` coerce to `&[T]` automatically
- Passing an existing `&mut` along: just `f(mana)`, not `f(&mut mana)`
- `v.remove(i)` takes ownership out of a Vec · `v.push(x)` moves ownership in
- `s.split_once(' ').unwrap_or((s, ""))` · `s.to_lowercase()` · `v.iter().enumerate()`
- `ARRAY.to_vec()` — make an owned, growable Vec from an array (used in `main.rs`)

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — split_command & spell_list</summary>

`let input = input.trim();` then `input.split_once(' ')` gives `Some((before, after))` or `None`; `.unwrap_or((input, ""))` handles the no-argument case. Trim the argument before returning. For the list, look at `chapter1::spell_list` — only the loop header changes: `for (i, (name, cost, power, level)) in spellbook.iter().enumerate()`.

</details>
<details class="hint"><summary>Hint 2 — cast</summary>

Look the spell up first; if `find_spell` returned `None`, return the "Unknown spell" message early (`.is_none()` tests for it, `.unwrap()` takes the value out once you know it's there). Then: level check → `if spend_mana(mana, cost) { success } else { not enough }`.

</details>
<details class="hint"><summary>Hint 3 — read_scroll & handle</summary>

```rust,ignore
if scrolls.is_empty() { return String::from("No scrolls left to read."); }
let spell = scrolls.remove(0);
spellbook.push(spell);
```

For `handle`: `let (command, argument) = split_command(input); let command = command.to_lowercase();` then one branch per command, e.g. `else if command == "potion" { drink_potion(mana); (format!("… {mana}"), true) }`.

</details>
