# 🏗️ Project 6 — Grimoire Organization

> *"A well-organised library survives centuries. A well-organised codebase survives its creators. The Head Librarian drops a 450-line scroll on your desk. 'It works,' she says. 'Nobody can find anything in it. Fix that — and break nothing.'"*

## 🎯 The Goal

The whole Chapter 5 game — spells, errors, creatures, the mage, potions, buffs, burns and statistics — lives in **one** `src/lib.rs`. It compiles. Every function works. Your job is pure **refactoring**: split it into modules with a clean public API, without changing behaviour.

You're done when the integration tests in `tests/grimoire.rs`, the game in `src/main.rs` and the example in `examples/quick_battle.rs` — all already written against the *new* module paths — compile and pass.

<div class="quest">

⚔️ **Project:** `exercises/06_grand_library/07_project_grimoire_organization`

- `Cargo.toml` — note `[lib] name = "grimoire"`: the package is called `ch06_07_project_grimoire_organization`, but its library crate is `grimoire`. That's why tests can write `use grimoire::...`.
- `src/lib.rs` — the monolith, split into `// ══ section ══` blocks.
- `src/main.rs`, `examples/quick_battle.rs`, `tests/grimoire.rs` — finished; they define the target layout.

Play when done: `cargo run -p ch06_07_project_grimoire_organization` · `cargo run -p ch06_07_project_grimoire_organization --example quick_battle`

</div>

## 📐 The Target Layout

```text
src/
├── lib.rs            ← crate docs, `pub mod`s, re-exports, prelude
├── main.rs           ← (given) the game
├── spells.rs         ← MAX_MANA, Spell, spellbook
├── error.rs          ← CastError (+ Display, Error)
├── creatures.rs      ← Creature, Goblin, Wraith, Dragon
├── mage.rs           ← Mage (new, find_spell, spend_for, cast, cast_with)
├── effects/
│   ├── mod.rs        ← pub mod potions/buffs/burns + re-exports
│   ├── potions.rs    ← Potion, Satchel, healing_draught, mana_elixir
│   ├── buffs.rs      ← Buff, empower, arcane_focus, chain_power, tick_buffs
│   └── burns.rs      ← Burn, tick_burns
└── stats.rs          ← CastRecord, total_damage, favourite_spell, …
```

`effects/mod.rs` is the classic style; `src/effects.rs` + `src/effects/*.rs` (no `mod.rs`) works just as well. Pick one.

## Phase 1 — The first shelves: `spells` and `error`

1. Create `src/spells.rs` and move the *spells* section into it.
2. Create `src/error.rs` and move the *error* section. It needs `use std::fmt;`.
3. In `lib.rs`, declare `pub mod spells;` and `pub mod error;`.

The rest of `lib.rs` still uses `Spell` and `CastError` — add `use crate::spells::{...};` / `use crate::error::CastError;` at the top of `lib.rs` for now, so everything keeps compiling while you work. **Move one section at a time and run `cargo build` after each.**

## Phase 2 — `creatures` and `mage`

1. `src/creatures.rs` ← the *creatures* section.
2. `src/mage.rs` ← the *mage* section. It reaches into three siblings:

```rust,ignore
use crate::creatures::Creature;
use crate::error::CastError;
use crate::spells::{MAX_MANA, Spell, spellbook};
```

`crate::` starts from the crate root (like an absolute path); `super::` goes one level up (relative). Inside `effects/potions.rs`, `super::` is `effects` and `crate::mage::Mage` is the mage.

## Phase 3 — The effects wing

1. Create the `effects` module with three submodules: `potions`, `buffs`, `burns`.
2. In `effects/mod.rs`, **re-export** the main types so users can write `grimoire::effects::Buff`:

```rust,ignore
pub mod buffs;
pub use buffs::Buff;
```

The test `the_effects_module_reexports_its_types` checks that `effects::Buff` and `effects::buffs::Buff` are the *same* type — a `pub use` gives a second name to one item; it doesn't copy anything.

<div class="pitfall">

`Potion.effect` and `Buff.modifier` are private fields. That was fine in one file — privacy in Rust is per **module**, and the constructors live in the same module as the structs. Keep `Potion` and its constructors together in `potions.rs` and it still works. Split them up and you'll meet `field 'effect' of struct 'Potion' is private`.

</div>

## Phase 4 — The front door

1. `src/stats.rs` ← the *stats* section (it needs `HashMap` and `Creature`).
2. Re-export the four most-used types at the root, so `grimoire::Mage`, `grimoire::Spell`, `grimoire::CastError` and `grimoire::Creature` work.
3. Add a `pub mod prelude` (inline in `lib.rs` is fine) that `pub use`s everything a battle needs. Check `main.rs` and the prelude test for the full list.
4. Crate-level docs: start `lib.rs` with `//!` lines describing the library — with a doc-test example using the prelude. `cargo doc -p ch06_07_project_grimoire_organization --open` and admire your catalogue.

When you're done, `lib.rs` should contain *no* logic at all — only `//!` docs, `mod` declarations and `use`s.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`mod` declares, `use` imports.** Each file is declared exactly once with `mod name;` in its parent. Writing `mod spells;` in `mage.rs` would look for `src/mage/spells.rs`!
- **Unused import warnings** in `lib.rs` after moving everything out: delete the temporary `use` lines from Phase 1.
- **`pub` on the module *and* on the item.** `pub mod effects;` with a private `mod buffs;` inside hides `effects::buffs::empower` from the tests.
- **Tests can't see `pub(crate)`.** If you'd like to hide `Mage::spend_for` from the outside world, `pub(crate)` is the right tool — nothing in `tests/` uses it.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `mod x;` → `src/x.rs` or `src/x/mod.rs` · nested: `src/x/y.rs`
- `pub mod x;` exposes the module · `pub use x::Item;` re-exports an item
- `crate::a::b` absolute · `super::b` parent · `self::b` current
- `pub(crate)` — visible in this crate only
- `[lib] name = "grimoire"` — rename the library crate
- `pub mod prelude { pub use crate::...; }` + `use grimoire::prelude::*;`
- `//!` documents the enclosing module/crate · `///` the next item

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the order of moves</summary>

Move the leaves first: `spells` and `error` depend on nothing; `creatures` on nothing; `mage` on all three; `effects` on `mage`, `spells` and `creatures`; `stats` on `creatures`. Moving in that order keeps the build green at each step.

</details>
<details class="hint"><summary>Hint 2 — the imports each file needs</summary>

`potions.rs`: `use crate::mage::Mage; use crate::spells::MAX_MANA;` · `burns.rs`: `use crate::creatures::Creature;` · `buffs.rs`: nothing · `stats.rs`: `use std::collections::HashMap; use crate::creatures::Creature;`. When in doubt, let the compiler tell you — `error[E0412]: cannot find type` usually comes with a `help: consider importing` line.

</details>
<details class="hint"><summary>Hint 3 — the finished lib.rs</summary>

Six `pub mod` lines (`creatures`, `effects`, `error`, `mage`, `spells`, `stats`), four root re-exports (`pub use mage::Mage;` …), and:

```rust,ignore
pub mod prelude {
    pub use crate::creatures::{Creature, Dragon, Goblin, Wraith};
    pub use crate::effects::{Buff, Burn, Potion, Satchel};
    pub use crate::effects::buffs::{arcane_focus, chain_power, empower, tick_buffs};
    // … burns::tick_burns, potions::{healing_draught, mana_elixir},
    //   error::CastError, mage::Mage, spells::{MAX_MANA, Spell, spellbook}
}
```

</details>

## 🏆 Victory Condition

`wr` is green, `lib.rs` is a table of contents, and `cargo doc` shows a library a stranger could navigate. The Grand Library is in order.
