# 📜 Chapter 3 — The Grimoire of Forms

> *"A spell is not just a name and a number. It has form — element, rank, power. To capture reality in code, you need shapes. Structs are your molds. Enums are your choices. Pattern matching is your divination."*

So far your spells were tuples: `("Fireball", 30, 45, 1)`. Quick to write — and impossible to read six months later. Was the third number the power or the level? In this chapter you give your data **names** and **shapes**, and the compiler starts checking your *domain*, not just your types.

Coming from Python, think `@dataclass` + `enum.Enum` — but enums whose variants carry data, and a `match` that refuses to compile when you forget a case. Coming from Go, think `struct` + the sum types Go never got, with methods but no inheritance.

| Quest | Topic |
|---|---|
| 3.0 | [Structs — Crafting Vessels of Magic](00_structs.md) |
| 3.1 | [Tuple & Unit Structs — Minimal Forms](01_tuple_unit_structs.md) |
| 3.2 | [Enums — The Choice of Forms](02_enums.md) |
| 3.3 | [Option — The Absence of Null](03_option.md) |
| 3.4 | [Match — The Art of Divination](04_match.md) |
| 3.5 | [If Let & Let-Else — The Single-Glance Divination](05_if_let.md) |
| 3.6 | [Methods — Breathing Life into Forms](06_methods.md) |
| 🏗️ | [Project: Modeling Magic](07_project_modeling_magic.md) |

<div class="grimoire">

**The whole chapter on one card**

- `struct` = **and** (a mage has a name *and* a level *and* mana)
- `enum` = **or** (a potion is Health *or* Mana *or* Water) — and each variant can carry data
- `Option<T>` = `Some(value)` or `None` — Rust's replacement for null
- `match` must cover **every** case; `if let` / `let … else` when you care about one
- `impl Type { … }` adds methods: `&self` reads, `&mut self` changes, `self` consumes

</div>
