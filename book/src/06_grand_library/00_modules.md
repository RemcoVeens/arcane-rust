# ⚔️ 6.0 — Modules: The Shelves of the Library

> *"A library without shelves is a pile of books. A codebase without modules is a pile of functions. The Head Librarian hands you a heap of scrolls and a single word: 'Organise.'"*

## ⚡ The Concept

### The module tree is declared, not discovered

A crate is a **tree of modules**. The root is `src/lib.rs` (or `src/main.rs`). Every other module exists only because a parent *declares* it with `mod`:

```rust,ignore
// src/lib.rs
pub mod spells;   // → look for src/spells.rs  OR  src/spells/mod.rs
pub mod potions;  // → look for src/potions.rs OR  src/potions/mod.rs
```

A file that nobody declares with `mod` is simply ignored — it isn't compiled at all.

```text
src/
├── lib.rs            crate
├── potions.rs        crate::potions
└── spells/
    ├── mod.rs        crate::spells
    ├── fire.rs       crate::spells::fire     (declared in spells/mod.rs)
    └── frost.rs      crate::spells::frost
```

You can also write a module **inline**, which is handy for small helpers (and it's what `mod tests { … }` has been all along):

```rust
pub mod brewing {
    pub fn brew(herb: &str) -> String {
        format!("{herb} tea")
    }
}
```

<div class="compare">

🐍 Python: every `.py` file is a module automatically; the folder layout *is* the package tree. · 🐹 Go: a package is a directory; every file in it belongs to it. · 🦀 Rust: the tree is spelled out with `mod` declarations, and files are just where the compiler looks for the bodies.

</div>

### Privacy: private by default

Everything is **private to its parent module** unless marked `pub`. A child can see its ancestors' private items; a parent cannot see its children's private items.

```rust
mod frost {
    pub fn frostbolt() -> u32 { 35 }
    fn shatter(power: u32) -> u32 { power * 2 } // only visible inside `frost`
}

fn cast() -> u32 {
    frost::frostbolt()       // ✅ pub
    // frost::shatter(35)    // ❌ error[E0603]: function `shatter` is private
}
```

For a module to be usable from outside the crate, the *whole path* must be `pub`: `pub mod spells` **and** `pub mod frost` **and** `pub fn frostbolt`.

<div class="compare">

🐹 Go decides visibility by capitalisation (`Frostbolt` vs `shatter`). Rust uses the `pub` keyword, and visibility is per module, not per package. There's also `pub(crate)` — "public inside this crate only", the closest thing to Go's package-internal.

</div>

### Paths: `crate::`, `super::`, `self::`

| Path | Means | Like |
|---|---|---|
| `crate::spells::MAX_MANA` | absolute, from the crate root | an absolute import |
| `super::Spell` | the parent module | Python's `from .. import` |
| `self::fire::fireball` | the current module | Python's `from . import` |

Inside `src/spells/frost.rs`, writing `use spells::Spell;` fails — paths in `use` start from a crate name, `crate`, `self` or `super`, never from a sibling module's name.

### `use` and `pub use`

`use` creates a local shortcut. `pub use` creates a shortcut **that others can use too** — a re-export:

```rust,ignore
// src/lib.rs
pub mod spells;
pub use spells::fire::fireball; // now `my_crate::fireball()` works
```

Re-exports let you organise the files however you like while offering users a flat, friendly API. Most crates you'll use do this.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Creating a file is not enough.** Without `mod potions;` in the parent, `src/potions.rs` is never compiled. Conversely, `mod potions;` without the file is `error[E0583]: file not found for module`.
- **Don't write `mod` twice.** Declare a module once (in its parent); everywhere else, refer to it with `use crate::…`. `mod` is not `import`!
- **`pub` on the item is not enough** if a module on the path is private. Read E0603 carefully — it names the private link in the chain.
- **`mod.rs` vs `name.rs`.** Both `src/spells/mod.rs` and `src/spells.rs` (+ a `src/spells/` folder for children) work. Pick one style per module; this quest uses `mod.rs`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/00_modules`

The crate **does not compile**. Fix the shelves:

1. Fix the two broken `use` paths in `src/spells/fire.rs` and `src/spells/frost.rs`.
2. Make the items the tests need public.
3. Create `src/potions.rs` following the spec in `lib.rs` (including an inline `brewing` module).
4. Re-export `fireball` and `frostbolt` at the crate root.

</div>

Work through the errors top to bottom — fixing one often reveals the next.

## 📜 The Grimoire

<div class="grimoire">

- `mod name;` → loads `name.rs` or `name/mod.rs` · `mod name { … }` → inline module
- Private by default; `pub` opens an item to the parent and beyond; `pub(crate)` = crate-wide only
- Paths: `crate::a::b` (absolute) · `super::x` (parent) · `self::x` (current)
- `use path::Item;` = local shortcut · `pub use path::Item;` = re-export
- Children see their ancestors' private items, not the other way round

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the paths</summary>

`MAX_MANA` lives in the `spells` module, not at the crate root: use `super::MAX_MANA` or `crate::spells::MAX_MANA`. In `frost.rs`, `spells::Spell` needs an anchor too.

</details>
<details class="hint"><summary>Hint 2 — privacy</summary>

The tests call `spells::frost::frostbolt()` and `spells::frost::shatter(…)`. Look at how `frost` is declared in `spells/mod.rs`, and at `shatter` itself.

</details>
<details class="hint"><summary>Hint 3 — potions & re-exports</summary>

`src/potions.rs` starts with `use crate::spells::MAX_MANA;`, defines the `pub const` and `pub fn mana_potion`, then `pub mod brewing { pub fn brew(…) -> String { format!(…) } }`. In `lib.rs`, replace the TODO with two `pub use spells::…;` lines.

</details>
