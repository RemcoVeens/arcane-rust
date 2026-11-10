# ⚔️ 6.1 — Crates & Packages: The Bound Tomes

> *"A single scroll is a module. A bound book is a crate. A shelf of books bought from another library is a dependency. Each has its place — and each must be written in the catalogue before you may read it."*

## ⚡ The Concept

### Package, crate, module

| Word | What it is | Defined by |
|---|---|---|
| **Package** | a folder with a `Cargo.toml` — what Cargo builds, tests, publishes | `Cargo.toml` |
| **Crate** | one compilation unit: a library or a binary | `src/lib.rs`, `src/main.rs`, `src/bin/*.rs` |
| **Module** | a namespace inside a crate | `mod` |

A package contains **at most one library crate** and **any number of binary crates**. Every quest so far was a package with just a library. This quest has both:

```text
01_crates_packages/
├── Cargo.toml
├── src/
│   ├── lib.rs     ← library crate  `ch06_01_crates_packages`
│   └── main.rs    ← binary crate   (a program with `fn main`)
└── runestone/     ← a *separate* package, used as a dependency
    ├── Cargo.toml
    └── src/lib.rs
```

`main.rs` uses the library **by its crate name**, exactly like an outsider would — the package name with `-` replaced by `_`:

```rust,ignore
use ch06_01_crates_packages::{glyph_name, greeting_stone};
```

This "thin `main.rs`, real logic in `lib.rs`" split is idiomatic Rust: the library is testable, the binary only does I/O.

<div class="compare">

🐍 Python: a project with a package *and* a `__main__.py` / console-script entry point. · 🐹 Go: `package main` in `cmd/` importing your own packages. · 🦀 Rust: `lib.rs` + `main.rs` in one package, no extra wiring needed.

</div>

### Dependencies

Dependencies go in `[dependencies]`. They come in three flavours:

```toml
[dependencies]
rand = "0.9"                                   # from crates.io (semver range)
runestone = { path = "runestone" }             # a local folder
spellforge = { git = "https://example.com/spellforge.git" } # a git repo
```

Once declared, the crate's name is available in every `use` — no `extern crate`, no import paths, no `go get`:

```rust,ignore
use runestone::Runestone;            // a type
use runestone::glyphs::to_glyph;     // a function in a module of that crate

let stone = Runestone::carve("Fireball");
let total = runestone::rune_count("Arcane Blast"); // full path works too
```

You only see what the other crate made `pub` — the module rules from 6.0 apply across crates.

<div class="compare">

🐍 Python: `pip install` + `requirements.txt`/`pyproject.toml`. · 🐹 Go: `go get` writes `go.mod`. · 🦀 Rust: edit `Cargo.toml` (or run `cargo add rand`); `Cargo.lock` pins exact versions, like `go.sum` / a lock file.

</div>

### Useful Cargo commands

```bash
cargo add rand                    # add a crates.io dependency
cargo run -p ch06_01_crates_packages -- Tiberius   # run the binary, pass args after --
cargo tree -p ch06_01_crates_packages              # show the dependency tree
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Path deps are relative to the `Cargo.toml` that declares them.** `path = "runestone"`, not `path = "exercises/…/runestone"`.
- **The dependency name is the *package* name** from the other `Cargo.toml` (`name = "runestone"`), not the folder name — they just happen to match here. Hyphens become underscores in code: `my-lib` → `use my_lib::…`.
- **`main.rs` cannot reach `lib.rs` with `crate::`.** They are different crates; `crate::` in `main.rs` means the binary. Use the library's crate name.
- **Private stays private.** `runestone`'s `FUTHARK` table is not `pub`, so you can't use it — go through `glyphs::to_glyph`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/01_crates_packages`

1. Declare `runestone` (in the `runestone/` folder) as a path dependency in this package's `Cargo.toml`.
2. Implement `spell_power`, `glyph_name`, `greeting_stone` and `mightiest` using runestone's public API.
3. Bonus: `cargo run -p ch06_01_crates_packages -- YourName` and read your name in runes.

</div>

Skim `runestone/src/lib.rs` first: reading another crate's public API (and its docs) is a skill in itself.

## 📜 The Grimoire

<div class="grimoire">

- Package = `Cargo.toml`; ≤ 1 lib crate + any number of bin crates
- `[dependencies] name = "1.2"` · `{ path = "…" }` · `{ git = "…" }`
- Use a dependency by name: `use runestone::Runestone;`
- `main.rs` uses `lib.rs` via the crate name, never `crate::`
- `cargo add`, `cargo run -p <pkg> -- <args>`, `cargo tree`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the dependency</summary>

Under `[dependencies]` add one line: the crate's name, `=`, and an inline table with a `path` key pointing at the folder next to `Cargo.toml`.

</details>
<details class="hint"><summary>Hint 2 — the easy three</summary>

`spell_power` is `Runestone::carve(name).power()`. `glyph_name` maps every char: `name.chars().map(runestone::glyphs::to_glyph).collect()`. `greeting_stone` carves a `format!`ted string — remember `carve` takes a `&str`.

</details>
<details class="hint"><summary>Hint 3 — mightiest</summary>

`max_by_key` returns the **last** maximum on ties, but the spec wants the first. Loop over `names`, keep a `best: Option<&'a str>`, and only replace it when the new name has strictly more letters (`runestone::rune_count`).

</details>
