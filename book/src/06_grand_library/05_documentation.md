# ⚔️ 6.5 — Documentation: The Archivist's Scrolls

> *"A spell without a scroll is a secret that dies with its caster. The Archivist will not shelve a single tome until every page is explained — and every example has been tried."*

## ⚡ The Concept

### Two kinds of doc comments

```rust
//! Inner doc comment: documents the thing it's *inside* —
//! at the top of lib.rs, that's the whole crate.

/// Outer doc comment: documents the item *below* it.
pub const MAX_LEVEL: u32 = 10;
```

Doc comments are Markdown. `cargo doc --open` turns them into the same kind of website you see on [docs.rs](https://docs.rs) for every published crate.

<div class="compare">

🐍 Python: docstrings + Sphinx/pdoc; doctest exists but is optional. · 🐹 Go: comments above declarations + `go doc`; `Example*` functions are tested. · 🦀 Rust: `///` + `cargo doc`, and every code example is a test by default.

</div>

### The conventional sections

```rust,ignore
/// Drain up to `amount` mana from the crystal.
///
/// Returns how much was actually drained. See also [`ManaCrystal::charge`].
///
/// # Examples
///
/// ```
/// # use my_crate::ManaCrystal;   // hidden line: compiled, not shown
/// let mut crystal = ManaCrystal::new(50);
/// assert_eq!(crystal.drain(80), 50);
/// ```
///
/// # Panics
/// Never.
///
/// # Errors
/// (for functions returning `Result`: when do they return `Err`?)
pub fn drain(&mut self, amount: u32) -> u32 { … }
```

The first line is the summary shown in lists — keep it one sentence.

### Intra-doc links

Square brackets with backticks link to other items, resolved like a `use` path:

```rust,ignore
/// Capped at [`MAX_LEVEL`]. Built by [`Scroll::new`]. See the [`School`] enum.
```

A link to something that doesn't exist (a typo, a renamed item) is a `rustdoc::broken_intra_doc_links` **warning**. With `RUSTDOCFLAGS='-D warnings'` — common in CI — it becomes an error.

### Making documentation mandatory

A lint at the top of `lib.rs` can demand docs on every public item:

```rust,ignore
#![deny(missing_docs)]   // inner attribute: applies to the whole crate
```

`#![…]` (with `!`) applies to the enclosing item — here the crate. Every `pub` struct, field, enum variant, function, method and constant now needs a `///`, and the crate itself needs a `//!`. Private items are exempt: docs are for your *users*.

`warn` instead of `deny` makes it a warning. Other useful lints: `#![warn(clippy::missing_panics_doc)]`, `#![deny(rustdoc::broken_intra_doc_links)]`.

### Viewing the result

```bash
cargo doc -p ch06_05_documentation --no-deps --open
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Examples are compiled.** When you change a signature, the old example breaks `cargo test`. That's the point — but it surprises people.
- **Example ≠ crate.** Doc tests need `use your_crate::…;` — hide it with `# ` if it's noise.
- **`//!` goes at the top** of the file (only attributes like `#![deny]` may come before it). An `//!` after an item is an error.
- **`///` above a `use` or a private item** is legal but pointless: it never shows up in the public docs.
- **Doc tests don't run on `cargo build`** — only `cargo test` (or `cargo test --doc`) catches a broken example.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/05_documentation`

The crate **does not compile** because of `#![deny(missing_docs)]`. Satisfy the Archivist:

1. Add a crate-level `//!` doc comment.
2. Document every public item — the enum's variants and the struct's fields too.
3. Fix the broken example on `Scroll::new` and the broken intra-doc link on `readable_by`.
4. Have **at least 3** doc tests in total.

</div>

`wr` runs `cargo test`, `cargo test --doc` and `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps`. Then admire the result with `cargo doc -p ch06_05_documentation --open`.

## 📜 The Grimoire

<div class="grimoire">

- `//!` documents the enclosing item (crate/module) · `///` documents the next item
- Markdown; sections `# Examples`, `# Panics`, `# Errors`, `# Safety`
- `` [`Item`] `` / `` [`Type::method`] `` = intra-doc link
- `# hidden line` in examples · examples run with `cargo test`
- `#![deny(missing_docs)]` · `cargo doc --no-deps --open`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — where to start</summary>

The compiler lists every undocumented item. Start with the crate itself: a few `//!` lines right below `#![deny(missing_docs)]`.

</details>
<details class="hint"><summary>Hint 2 — the broken bits</summary>

`Scroll::new` takes **three** arguments now, and the example needs to import `School`. Is a level-1 scroll really level 2? For the link, compare the name inside `[`…`]` with the actual method name.

</details>
<details class="hint"><summary>Hint 3 — the third doc test</summary>

Give `is_forbidden` (or the crate-level `//!` docs) an example:

````rust,ignore
/// ```
/// use ch06_05_documentation::{School, Scroll};
///
/// assert!(Scroll::new("Raise Dead", 1, School::Necromancy).is_forbidden());
/// ```
````

</details>
